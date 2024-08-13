//! Routines that allow us to act as a Universal Serial Bus device.
//!
//! Why are USB peripherals on microcontrollers always so hard to work with?


use cortex_m::asm::nop;
use cortex_m::peripheral::NVIC;
use stm32g0b0::{interrupt, Interrupt, Peripherals};
use stm32g0b0::usb::chepnr::{Statrx, Stattx};


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

fn set_chepnr_stattx(chepnr: &stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>, value: Stattx) {
    let current_value = chepnr.read().stattx().bits();
    let new_value: u8 = value.into();
    let set_value = current_value ^ new_value;
    chepnr.modify(|_, w| w
        .stattx().set(set_value)
        .vtrx().set_bit() // leave unchanged
        .vttx().set_bit() // leave unchanged
    );
}

fn set_chepnr_statrx(chepnr: &stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>, value: Statrx) {
    let current_value = chepnr.read().statrx().bits();
    let new_value: u8 = value.into();
    let set_value = current_value ^ new_value;
    chepnr.modify(|_, w| w
        .statrx().set(set_value)
        .vtrx().set_bit() // leave unchanged
        .vttx().set_bit() // leave unchanged
    );
}

fn set_chepnr_tx_dtog(chepnr: &stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>, value: bool) {
    let current_value = chepnr.read().dtogtx().bit_is_set();
    if current_value != value {
        chepnr.modify(|_, w| w
            .dtogtx().set_bit() // toggle
            .vtrx().set_bit() // leave unchanged
            .vttx().set_bit() // leave unchanged
        );
    }
}

fn set_chepnr_rx_dtog(chepnr: &stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>, value: bool) {
    let current_value = chepnr.read().dtogrx().bit_is_set();
    if current_value != value {
        chepnr.modify(|_, w| w
            .dtogrx().set_bit() // toggle
            .vtrx().set_bit() // leave unchanged
            .vttx().set_bit() // leave unchanged
        );
    }
}

#[interrupt]
fn USB() {
    let peripherals = unsafe { Peripherals::steal() };
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
    crate::usb::distribute_usb_interrupt();
}


pub fn distribute_usb_interrupt() {
    let mut peripherals = unsafe { Peripherals::steal() };
    if peripherals.syscfg.itline8().read().usb().bit_is_set() {
        handle_usb_interrupt(&mut peripherals);
    }
}

fn handle_usb_interrupt(peripherals: &mut Peripherals) {
    if peripherals.usb.istr().read().rst_dcon().bit_is_set() {
        // USB reset received
        post_reset_setup(peripherals);
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
        .count_rx().set(EP_BUF_SIZE.try_into().unwrap())
        .num_block().set((EP_BUF_SIZE/32 - 1).try_into().unwrap())
        .blsize().set_bit()
    );

    // set up Ep0 buffer
    peripherals.usb.chepnr(0).modify(|_, w| w
        .ea().set(0x0) // endpoint 0
        .utype().control() // it's a control endpoint
        .vtrx().set_bit() // leave unchanged
        .vttx().set_bit() // leave unchanged
    );
    set_chepnr_statrx(peripherals.usb.chepnr(0), Statrx::Valid);
    set_chepnr_stattx(peripherals.usb.chepnr(0), Stattx::Nak);
    set_chepnr_rx_dtog(peripherals.usb.chepnr(0), false);

    // enable USB function (no address assigned yet)
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

    // enable USB interrupt with priority 3
    unsafe {
        let mut core_peripherals = cortex_m::Peripherals::steal();
        core_peripherals.NVIC.set_priority(Interrupt::USB, 3);
        NVIC::unmask(Interrupt::USB);
    }
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
    // 1µs * 48MHz = 48
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

    // remove spurious interrupt states
    peripherals.usb.istr().write(|w| unsafe { w.bits(0) });

    // configure the device
    peripherals.usb.cntr().modify(|_, w| w
        .host().clear_bit() // I'm a device
        .suspen().clear_bit() // do not suspend me
        .l2res().clear_bit() // no L2 remote wakeup/resume
        .l1res().clear_bit() // no L1 remote wakeup/resume

        // enable most interrupts
        .l1reqm().set_bit()
        .esofm().set_bit()
        .sofm().set_bit()
        .rst_dconm().set_bit()
        .suspm().set_bit()
        .wkupm().set_bit()
        .errm().set_bit()
        .ctrm().set_bit()

        // not these, though
        .pmaovrm().clear_bit()
        .thr512m().clear_bit()
    );

    /*
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
    */

    //post_reset_setup(peripherals);

    // say hello
    peripherals.usb.bcdr().modify(|_, w| w
        .dppu_dpd().set_bit()
    );
}
