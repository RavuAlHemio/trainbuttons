use bitmacros::{bit_mask, extract_bits};
use stm32g0b0::Peripherals;


pub(crate) fn enable_peripheral_clocks(peripherals: &Peripherals) {
    // send clock to GPIOA, GPIOB and GPIOD
    peripherals.rcc.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
        .gpioben().set_bit()
        .gpioden().set_bit()
    );
}

pub(crate) fn start_reset(peripherals: &Peripherals) {
    // reset GPIOA, GPIOB and GPIOD
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().set_bit()
        .gpiobrst().set_bit()
        .gpiodrst().set_bit()
    );
}

pub(crate) fn stop_reset(peripherals: &Peripherals) {
    // disable reset of GPIOA, GPIOB and GPIOD
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().clear_bit()
        .gpiobrst().clear_bit()
        .gpiodrst().clear_bit()
    );
}

pub(crate) fn set_up(peripherals: &Peripherals) {
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
}

pub(crate) fn read_button_states(peripherals: &Peripherals) -> u32 {
    // buttons are:
    // PA1-PA4, PA7, PA15 (6 bits)
    // PB1-PB11 (11 bits)
    // [PB12 and PB13 are LEDs]
    // PD0-PD3 (4 bits)
    let pa_bits = peripherals.gpioa.idr().read().bits();
    let pa_button_states: u32 =
        (extract_bits!(pa_bits, 1, 4) ^ bit_mask!(0, 4))
        | ((extract_bits!(pa_bits, 7, 1) ^ bit_mask!(0, 1)) << 4)
        | ((extract_bits!(pa_bits, 15, 1) ^ bit_mask!(0, 1)) << 5)
    ;
    let pb_bits = peripherals.gpiob.idr().read().bits();
    let pb_button_states: u32 =
        extract_bits!(pb_bits, 1, 11) ^ bit_mask!(0, 11)
    ;
    let pd_bits = peripherals.gpiod.idr().read().bits();
    let pd_button_states: u32 =
        extract_bits!(pd_bits, 0, 4) ^ bit_mask!(0, 4)
    ;

    // fill up from the least significant bit
    // 000D DDDB BBBB BBBB BBAA AAAA
    let all_button_states =
        pa_button_states
        | (pb_button_states << 6)
        | (pd_button_states << (6 + 11))
    ;
    all_button_states
}
