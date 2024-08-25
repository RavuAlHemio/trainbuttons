#![no_std]
#![no_main]


mod clock;
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

    // send clock to GPIOA, GPIOB and GPIOD
    peripherals.rcc.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
        .gpioben().set_bit()
        .gpioden().set_bit()
    );

    // reset GPIOA, GPIOB and GPIOD
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().set_bit()
        .gpiobrst().set_bit()
        .gpiodrst().set_bit()
    );
    // give it a bit to reset
    for _ in 0..64 {
        cortex_m::asm::nop();
    }
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().clear_bit()
        .gpiobrst().clear_bit()
        .gpiodrst().clear_bit()
    );
    // give it a bit to reinitialize
    for _ in 0..64 {
        cortex_m::asm::nop();
    }

    // set up buttons (digital input, pull up)
    // and LEDs (output, push-pull)
    peripherals.gpioa.moder().modify(|_, w| w
        .moder1().input()
        .moder2().input()
        .moder3().input()
        .moder4().input()
        .moder7().input()
        .moder15().input()
    );
    peripherals.gpiob.moder().modify(|_, w| w
        .moder1().input()
        .moder2().input()
        .moder3().input()
        .moder4().input()
        .moder5().input()
        .moder6().input()
        .moder7().input()
        .moder8().input()
        .moder9().input()
        .moder10().input()
        .moder11().input()
        .moder12().general_purpose_output()
        .moder13().general_purpose_output()
    );
    peripherals.gpiod.moder().modify(|_, w| w
        .moder0().input()
        .moder1().input()
        .moder2().input()
        .moder3().input()
    );
    peripherals.gpioa.pupdr().modify(|_, w| w
        .pupdr1().pull_up()
        .pupdr2().pull_up()
        .pupdr3().pull_up()
        .pupdr4().pull_up()
        .pupdr7().pull_up()
        .pupdr15().pull_up()
    );
    peripherals.gpiob.pupdr().modify(|_, w| w
        .pupdr1().pull_up()
        .pupdr2().pull_up()
        .pupdr3().pull_up()
        .pupdr4().pull_up()
        .pupdr5().pull_up()
        .pupdr6().pull_up()
        .pupdr7().pull_up()
        .pupdr8().pull_up()
        .pupdr9().pull_up()
        .pupdr10().pull_up()
        .pupdr11().pull_up()
    );
    peripherals.gpiod.pupdr().modify(|_, w| w
        .pupdr0().pull_up()
        .pupdr1().pull_up()
        .pupdr2().pull_up()
        .pupdr3().pull_up()
    );
    peripherals.gpiob.otyper().modify(|_, w| w
        .ot12().push_pull()
        .ot13().push_pull()
    );

    // turn on other device
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
