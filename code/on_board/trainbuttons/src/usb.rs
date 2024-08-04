//! Routines that allow us to act as a Universal Serial Bus device.
//!
//! Why are USB peripherals on microcontrollers always so hard to work with?


use cortex_m::asm::nop;
use cortex_m::peripheral::NVIC;
use stm32g0b0::{interrupt, Interrupt, Peripherals};


#[interrupt]
fn USB() {
    // USB interrupt
    let peripherals = unsafe { Peripherals::steal() };

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

    // enable USB interrupt
    unsafe { NVIC::unmask(Interrupt::USB) };

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

    // enable USB reset condition
    peripherals.usb.cntr().modify(|_, w| w
        .usbrst().set_bit()
    );

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

    // disable most interrupts
    peripherals.usb.cntr().modify(|_, w| w
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

    // set up buffers
    peripherals.usb_ram1.single_buffered(0);
}
