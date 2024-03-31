//! Clock management.
//!
//! Due to USB requiring a 48 MHz clock and the STM32 accepting a 4-48 MHz oscillator or crystal,
//! the following clock setup is used:
//!
//! ```plain
//! ┌────────┐   ┌────────┐
//! │ HSE    ├─┬─┤ SYSCLK │
//! │ 48 MHz │ │ │ 48 MHz │
//! └────────┘ │ └────────┘
//!            │
//!            │ ┌────────┐
//!            └─┤ USB    │
//!              │ 48 MHz │
//!              └────────┘
//! ```
//!
//! Glossary:
//! * HSE: High-Speed External (Clock), pins PF0 and PF1


use stm32g0b0::Peripherals;


/// Sets up clocks according to the requirements of the project.
///
/// See the module documentation for a diagram.
pub(crate) fn set_up(peripherals: &mut Peripherals) {
    // startup state: HSI16 (16 MHz) --> HSIDIV = /1 --> HSISYS (16 MHz) --> SYSCLK (16 MHz)

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

    // also plug HSE into USB clock
    peripherals.rcc.ccipr2().modify(|_, w| w
        .usbsel().hse()
    );
}
