#![no_std]
#![no_main]


mod clock;
mod pins;
mod uart;
mod usb;


use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32g0b0::Peripherals;


#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {
    }
}


#[inline(never)]
fn tell_gdb_to_return_from_this_function() {
    // address 4 (reset interrupt) is never 1
    const RESET_INTERRUPT_PTR: *const u32 = 0x0000_0004 as *const u32;
    loop {
        let isvp = unsafe { core::ptr::read_volatile(RESET_INTERRUPT_PTR) };
        if isvp == 1 {
            return;
        }
    }
}


#[entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };

    crate::clock::set_up(&peripherals);

    // disable pulldowns
    peripherals.syscfg.cfgr1().modify(|_, w| w
        .ucpd1_strobe().set_bit() // disable pulldowns on PA8 and PB15
        .ucpd2_strobe().set_bit() // disable pulldowns on PD0 and PD2
    );

    // send clock to all relevant peripherals
    crate::pins::enable_peripheral_clocks(&peripherals);
    crate::uart::enable_peripheral_clocks(&peripherals);
    crate::usb::enable_peripheral_clocks(&peripherals);

    // reset all relevant peripherals
    crate::pins::start_reset(&peripherals);
    crate::uart::start_reset(&peripherals);
    crate::usb::start_reset(&peripherals);
    // give it a bit to reset
    for _ in 0..64 {
        cortex_m::asm::nop();
    }
    crate::pins::stop_reset(&peripherals);
    crate::uart::stop_reset(&peripherals);
    crate::usb::stop_reset(&peripherals);
    // give it a bit to reinitialize
    for _ in 0..64 {
        cortex_m::asm::nop();
    }

    // set up pins (buttons and LEDs)
    crate::pins::set_up(&peripherals);

    // turn on LED on other board
    peripherals.gpiob.bsrr().write(|w| w
        .bs12().set_bit()
    );

    // set up UART and USB
    crate::uart::set_up(&peripherals);
    crate::usb::set_up(&peripherals);

    // send initial data via UART
    crate::uart::write_bytes(&peripherals, b"\r\nHi, world!\r\n");

    /*
    // disable USB IRQ so we can poke around
    cortex_m::peripheral::NVIC::mask(stm32g0b0::Interrupt::USB);
    tell_gdb_to_return_from_this_function();
    */

    loop {
        cortex_m::asm::wfe();
    }
}
