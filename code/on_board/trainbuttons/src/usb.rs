//! Routines that allow us to act as a Universal Serial Bus device.
//!
//! Why are USB peripherals on microcontrollers always so hard to work with?


use cortex_m::asm::nop;
use cortex_m::peripheral::NVIC;
use stm32g0b0::{interrupt, Interrupt, Peripherals};


// controller-specific values; these are for STM32G0x0:
const ENDPOINT_CONFIG_COUNT: usize = 8;
const USB_PACKET_RAM_BASE: *mut u8 = 0x4000_9800 as *mut u8; // USB_DRD_PMAADDR or USB_DRD_PMA_BUFF
const USB_PACKET_RAM_SIZE: usize = 2*1024; // USB_DRD_PMA_SIZE

// packet data RAM layout:
//
// 4B endpoint 0 Tx config
// 4B endpoint 0 Rx config
// 4B endpoint 1 Tx config (if applicable)
// 4B endpoint 1 Rx config (if applicable)
// ...
// endpoint 0 Tx buffer
// endpoint 0 Rx buffer
// endpoint 1 Tx buffer (if applicable)
// endpoint 1 Rx buffer (if applicable)
// ...
//
// buffers are apportioned equally among endpoints

const PACKET_DATA_RAM_OFFSET: usize = 2 * 4 * ENDPOINT_CONFIG_COUNT; // (rx+tx) * 4 bytes/register * endpoints
const PACKET_DATA_RAM_POINTER: *mut u8 = USB_PACKET_RAM_BASE.wrapping_add(PACKET_DATA_RAM_OFFSET);
const PACKET_DATA_RAM_SIZE: usize = USB_PACKET_RAM_SIZE - PACKET_DATA_RAM_OFFSET;

const EP_BUF_SIZE: usize = PACKET_DATA_RAM_SIZE / (ENDPOINT_CONFIG_COUNT * 2);
const EP0_TX_OFFSET: usize = PACKET_DATA_RAM_OFFSET + 0*EP_BUF_SIZE;
const EP0_RX_OFFSET: usize = PACKET_DATA_RAM_OFFSET + 1*EP_BUF_SIZE;


fn get_usb_ep0_tx_buf() -> &'static mut [u8] {
    unsafe {
        core::slice::from_raw_parts_mut(
            PACKET_DATA_RAM_POINTER.wrapping_add(EP0_TX_OFFSET),
            EP_BUF_SIZE,
        )
    }
}
fn get_usb_ep0_rx_buf() -> &'static mut [u8] {
    unsafe {
        core::slice::from_raw_parts_mut(
            PACKET_DATA_RAM_POINTER.wrapping_add(EP0_RX_OFFSET),
            EP_BUF_SIZE,
        )
    }
}


#[interrupt]
fn USB() {
    handle_usb_interrupt();
}

fn handle_usb_interrupt() {
    // USB interrupt
    let mut peripherals = unsafe { Peripherals::steal() };

    // blink the LED
    loop {
        peripherals.gpiob.bsrr().write(|w| w
            .bs13().set_bit()
        );
        for _ in 0..4*1024*1024 {
            cortex_m::asm::nop();
        }
        peripherals.gpiob.bsrr().write(|w| w
            .br13().set_bit()
        );
        for _ in 0..4*1024*1024 {
            cortex_m::asm::nop();
        }
    }

    if peripherals.usb.istr().read().rst_dcon().bit_is_set() {
        // USB reset received
        post_reset_setup(&mut peripherals);
    }
    if peripherals.usb.istr().read().ctr().bit_is_set() {
        let endpoint = peripherals.usb.istr().read().idn().bits();
        let endpoint_register = peripherals.usb.chepnr(endpoint.into());
    }
}


/// Setup operations to be performed before the first activation as well as after every reset.
fn post_reset_setup(peripherals: &mut Peripherals) {
    // clear most other interrupts except the following
    peripherals.usb.istr().write(|w| w
        .pmaovr().set_bit()
        .err().set_bit()
        .wkup().set_bit()
        .susp().set_bit()
        .sof().set_bit()
        .esof().set_bit()
    );

    // define space for Ep0 buffers
    peripherals.usb_ram1.single_buffered(0).chep_txrxbd_0().modify(|_, w| w
        .addr_tx().set(EP0_TX_OFFSET.try_into().unwrap())
        .count_tx().set(0)
    );
    peripherals.usb_ram1.single_buffered(0).chep_rxtxbd_0().modify(|_, w| w
        .addr_rx().set(EP0_RX_OFFSET.try_into().unwrap())
        .count_rx().set(0)
        .num_block().set((EP_BUF_SIZE/32 - 1).try_into().unwrap())
        .blsize().set_bit()
    );

    // set up Ep0 buffer
    peripherals.usb.chepnr(0).modify(|_, w| w
        .ea().set(0x0) // endpoint 0
        .utype().control() // it's a control endpoint
        .statrx().valid() // I am ready to receive
    );

    // enable USB function for address 0x00 (no address assigned yet)
    peripherals.usb.daddr().modify(|_, w| w
        .ef().set_bit()
        .add().set(0x00)
    );
}


/// Sets up USB device support.
///
/// Assumes that clocks have already been set up using [`crate::clock::set_up`].
pub(crate) fn set_up(peripherals: &mut Peripherals) {
    // send power to the USB macrocell
    peripherals.pwr.cr2().modify(|_, w| w
        .usv().set_bit()
    );

    // send a clock to the USB macrocell
    peripherals.rcc.apbenr1().modify(|_, w| w
        .usben().set_bit()
    );

    // give it a bit
    for _ in 0..8 {
        nop();
    }

    // trigger USB macrocell reset
    peripherals.rcc.apbrstr1().modify(|_, w| w
        .usbrst().set_bit()
    );
    for _ in 0..8 {
        nop();
    }
    peripherals.rcc.apbrstr1().modify(|_, w| w
        .usbrst().clear_bit()
    );
    for _ in 0..8 {
        nop();
    }

    // enable USB reset and power-down condition
    peripherals.usb.cntr().modify(|_, w| w
        .usbrst().set_bit()
        .pdwn().set_bit()
    );
    for _ in 0..8 {
        nop();
    }

    // enable USB transceiver
    peripherals.usb.cntr().modify(|_, w| w
        .pdwn().clear_bit()
    );

    // wait t_{STARTUP}
    // RM says it's specified in DS, nothing relevant in DS
    // ST case 00202347 clarifies: it's max. 1µs
    // 1µs / 48MHz = 48
    for _ in 0..48 {
        nop();
    }

    // remove USB reset condition
    peripherals.usb.cntr().modify(|_, w| w
        .usbrst().clear_bit()
    );
    for _ in 0..8 {
        nop();
    }

    // enable USB interrupt and set its priority to 0
    unsafe { NVIC::unmask(Interrupt::USB) };
    let mut core_peripherals = unsafe { cortex_m::Peripherals::steal() };
    unsafe { core_peripherals.NVIC.set_priority(Interrupt::USB, 0) };
    for _ in 0..8 {
        nop();
    }

    // disable most interrupts
    peripherals.usb.cntr().modify(|_, w| w
        /*
        .l1reqm().clear_bit() // disable "LPM L1 state request" interrupt
        .esofm().clear_bit() // disable "expected start of frame" interrupt
        .sofm().clear_bit() // disable "start of frame" interrupt
        .rst_dconm().set_bit() // enable "reset" (device mode) or "device disconnected" (host mode) interrupt
        .suspm().clear_bit() // disable "suspend mode" interrupt
        .wkupm().clear_bit() // disable "wakeup" interrupt
        .errm().clear_bit() // disable "error" interrupt
        .pmaovrm().clear_bit() // disable "packet memory area overrun/underrun" interrupt
        .ctrm().clear_bit() // disable "correct transfer" interrupt
        .thr512m().clear_bit() // disable "512 byte threshold" interrupt
        */

        .l1reqm().set_bit()
        .esofm().set_bit()
        .sofm().set_bit()
        .rst_dconm().set_bit()
        .suspm().set_bit()
        .wkupm().set_bit()
        .errm().set_bit()
        .pmaovrm().set_bit()
        .ctrm().set_bit()
        .thr512m().set_bit()
    );

    // remove spurious interrupt states
    peripherals.usb.istr().write(|w| w
        .l1req().clear_bit()
        .esof().clear_bit()
        .sof().clear_bit()
        .rst_dcon().clear_bit()
        .susp().clear_bit()
        .wkup().clear_bit()
        .err().clear_bit()
        .pmaovr().clear_bit()
        .thr512().clear_bit()
    );

    post_reset_setup(peripherals);

    // say hello
    peripherals.usb.bcdr().modify(|_, w| w
        .dppu_dpd().set_bit()
    );
}
