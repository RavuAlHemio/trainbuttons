#![no_std]
#![no_main]


mod clock;
mod usb;


use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32g0b0::Peripherals;


#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {
    }
}


#[entry]
fn main() -> ! {
    let mut peripherals = unsafe { Peripherals::steal() };
    crate::clock::set_up(&mut peripherals);

    // send clock to GPIOB
    peripherals.rcc.iopenr().modify(|_, w| w
        .gpioben().set_bit()
    );

    // reset GPIOB
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpiobrst().set_bit()
    );
    // give it a bit to reset
    for _ in 0..64 {
        cortex_m::asm::nop();
    }
    peripherals.rcc.ioprstr().modify(|_, w| w
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

    crate::usb::set_up(&mut peripherals);

    loop {
    }
}
