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
