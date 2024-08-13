//! Clock management.
//!
//! Due to USB requiring a 48 MHz clock and the STM32 accepting a 4-48 MHz oscillator or crystal,
//! the following clock setup is used:
//!
//! ```plain
//! ┌────────┐   ┌────────┐ ╭────────────╮ ┌────────┐ ╭────────────╮ ┌────────┐
//! │ HSE    ├─┬─┤ SYSCLK ├─┤ AHBPRESC/1 ├─┤ HCLK   ├─┤ APBPRESC/1 ├─┤ PCLK   │
//! │ 48 MHz │ │ │ 48 MHz │ │ 48 MHz     │ │ 48 MHz │ │ 48 MHz     │ │ 48 MHz │
//! └────────┘ │ └────────┘ ╰────────────╯ └────────┘ ╰────────────╯ └────────┘
//!            │
//!            │ ┌────────┐
//!            └─┤ USB    │
//!              │ 48 MHz │
//!              └────────┘
//! ```
//!
//! Glossary:
//! * HSE: High-Speed External (Clock), pins PF0 and PF1
//! * HCLK: AHB (AMBA High-Speed Bus) clock
//! * PCLK: APB (Advanced Peripheral Bus) clock
//! * AMBA: Advanced Microcontroller Bus Architecture


use stm32g0b0::Peripherals;


/// Sets up clocks according to the requirements of the project.
///
/// See the module documentation for a diagram.
pub(crate) fn set_up(peripherals: &mut Peripherals) {
    // startup state: HSI16 (16 MHz) --> HSIDIV = /1 --> HSISYS (16 MHz) --> SYSCLK (16 MHz)

    // slow down flash because we're going above 24 MHz
    peripherals.flash.acr().modify(|_, w| w
        .latency().ws1()
    );
    while !peripherals.flash.acr().read().latency().is_ws1() {
    }

    // disable PLL (can only handle inputs 2.66 MHz <= f <= 16 MHz)
    peripherals.rcc.cr().modify(|_, w| w
        .pllon().clear_bit()
    );

    // wait for PLL to turn off
    while peripherals.rcc.cr().read().pllrdy().bit_is_set() {
    }

    // enable HSE
    peripherals.rcc.cr().modify(|_, w| w
        .hsebyp().clear_bit() // it's crystal, not an external clock signal
        .hseon().set_bit() // enable external high-speed crystal
    );

    // wait for HSE to stabilize
    while peripherals.rcc.cr().read().hserdy().bit_is_clear() {
    }

    // plug HSE into system clock
    peripherals.rcc.cfgr().modify(|_, w| w
        .sw().hse()
    );

    // wait for HSE to become system clock
    while !peripherals.rcc.cfgr().read().sws().is_hse() {
    }

    // pass system clock to high-speed bus without division
    // (DS13565 § 5.3.1: works up to 64 MHz; with 48 MHz, we are below)
    peripherals.rcc.cfgr().modify(|_, w| w
        .hpre().prescale1()
    );
    while !peripherals.rcc.cfgr().read().hpre().is_prescale1() {
    }

    // pass high-speed bus clock to peripheral bus without division
    // (DS13565 § 5.3.1: works up to 64 MHz; with 48 MHz, we are below)
    peripherals.rcc.cfgr().modify(|_, w| w
        .ppre().prescale1()
    );
    while !peripherals.rcc.cfgr().read().ppre().is_prescale1() {
    }

    // also plug HSE into USB clock
    peripherals.rcc.ccipr2().modify(|_, w| w
        .usbsel().hse()
    );
}
