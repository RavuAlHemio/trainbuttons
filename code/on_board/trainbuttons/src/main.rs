#![no_std]
#![no_main]


mod clock;
mod uart;
mod usb;


use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32g0b0::Peripherals;


static mut BLINK_ME: bool = false;


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
    let mut peripherals = unsafe { Peripherals::steal() };

    crate::clock::set_up(&mut peripherals);

    // send clock to GPIOA and GPIOB
    peripherals.rcc.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
        .gpioben().set_bit()
    );

    // reset GPIOA and GPIOB
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().set_bit()
        .gpiobrst().set_bit()
    );
    // give it a bit to reset
    for _ in 0..64 {
        cortex_m::asm::nop();
    }
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().clear_bit()
        .gpiobrst().clear_bit()
    );
    // give it a bit to reinitialize
    for _ in 0..64 {
        cortex_m::asm::nop();
    }

    // prepare LED for blinking
    peripherals.gpiob.moder().modify(|_, w| w
        .moder13().general_purpose_output()
    );
    peripherals.gpiob.otyper().modify(|_, w| w
        .ot13().push_pull()
    );

    // set up UART and USB
    crate::uart::set_up(&mut peripherals);
    crate::usb::set_up(&mut peripherals);

    // send initial data via UART
    crate::uart::write_bytes(&mut peripherals, b"\r\nHi, world!\r\n");

    /*
    // disable USB IRQ so we can poke around
    cortex_m::peripheral::NVIC::mask(stm32g0b0::Interrupt::USB);
    tell_gdb_to_return_from_this_function();
    */

    loop {
        cortex_m::asm::wfe();
    }
}
