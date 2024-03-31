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



    loop {
    }
}
