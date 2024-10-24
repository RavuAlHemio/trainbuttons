//! Interfacing with the analog-digital converter (ADC).


use stm32g0b0::Peripherals;


pub(crate) static mut ADC_VALUES: [u32; 1] = [0; 1];


pub(crate) fn enable_peripheral_clocks(peripherals: &Peripherals) {
    // send clock to GPIOA, ADC and DMA1 (includes DMAMUX)
    peripherals.rcc.ahbenr().modify(|_, w| w
        .dma1en().set_bit()
    );
    peripherals.rcc.apbenr2().modify(|_, w| w
        .adcen().set_bit()
    );
    peripherals.rcc.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
    );
}

pub(crate) fn start_reset(peripherals: &Peripherals) {
    // reset GPIOA, ADC and DMA1
    peripherals.rcc.ahbrstr().modify(|_, w| w
        .dma1rst().set_bit()
    );
    peripherals.rcc.apbrstr2().modify(|_, w| w
        .adcrst().set_bit()
    );
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().set_bit()
    );
}

pub(crate) fn stop_reset(peripherals: &Peripherals) {
    // disable reset of GPIOA, ADC and DMA1
    peripherals.rcc.ahbrstr().modify(|_, w| w
        .dma1rst().clear_bit()
    );
    peripherals.rcc.apbrstr2().modify(|_, w| w
        .adcrst().clear_bit()
    );
    peripherals.rcc.ioprstr().modify(|_, w| w
        .gpioarst().clear_bit()
    );
}


pub(crate) fn set_up(peripherals: &Peripherals) {
    // set PA6 to analog, don't pull
    peripherals.gpioa.moder().modify(|_, w| w
        .moder6().analog()
    );
    peripherals.gpioa.pupdr().modify(|_, w| w
        .pupdr6().no()
    );

    // enable ADC voltage regulator
    peripherals.adc.cr().modify(|_, w| w
        .advregen().set_bit()
    );
    // wait time = 20µs
    // clock speed (due to USB) = 48 MHz
    // most conservative case (completely unrolled loop): 1 cycle per nop
    // => 960 nops
    for _ in 0..960 {
        cortex_m::asm::nop();
    }

    // prepare for calibration
    peripherals.adc.cr().modify(|_, w| w
        .aden().clear_bit()
    );
    peripherals.adc.cfgr1().modify(|_, w| w
        .dmaen().clear_bit()
    );

    // start calibration
    peripherals.adc.cr().modify(|_, w| w
        .adcal().set_bit()
    );
    // wait for it to finish
    while peripherals.adc.cr().read().adcal().bit_is_set() {
    }

    // enable ADC
    peripherals.adc.isr().modify(|_, w| w
        .adrdy().set_bit() // set-to-clear
    );
    peripherals.adc.cr().modify(|_, w| w
        .aden().set_bit()
    );
    while peripherals.adc.isr().read().adrdy().bit_is_clear() {
    }

    // take clock from peripheral bus, divided by 2
    peripherals.adc.cfgr2().modify(|_, w| w
        .ckmode().pclk_by_2()
    );

    // clear CCRDY
    peripherals.adc.isr().modify(|_, w| w
        .ccrdy().set_bit() // set-to-clear
    );

    // scan channels sequentially
    peripherals.adc.cfgr1().modify(|_, w| w
        .chselrmod().numeric_order()
        .scandir().upward()
    );

    // only scan channel 6
    peripherals.adc.chselr_0().modify(|_, w| w
        .chsel6().set_bit()

        .chsel0().clear_bit()
        .chsel1().clear_bit()
        .chsel2().clear_bit()
        .chsel3().clear_bit()
        .chsel4().clear_bit()
        .chsel5().clear_bit()
        .chsel7().clear_bit()
        .chsel8().clear_bit()
        .chsel9().clear_bit()
        .chsel10().clear_bit()
        .chsel11().clear_bit()
        .chsel12().clear_bit()
        .chsel13().clear_bit()
        .chsel14().clear_bit()
        .chsel15().clear_bit()
        .chsel16().clear_bit()
        .chsel17().clear_bit()
        .chsel18().clear_bit()
    );

    // wait for CCRDY
    while peripherals.adc.isr().read().ccrdy().bit_is_clear() {
    }

    // excursion: configure DMA
    peripherals.dma1.cpar1().modify(|_, w| w
        .pa().set(peripherals.adc.dr().as_ptr() as u32) // ADC data register
    );
    let adc_ptr = unsafe { ADC_VALUES.as_mut_ptr() };
    peripherals.dma1.cmar1().modify(|_, w| w
        .ma().set(adc_ptr as u32) // memory address
    );
    let adc_count = unsafe { ADC_VALUES.len() };
    peripherals.dma1.cndtr1().modify(|_, w| w
        .ndt().set(adc_count.try_into().unwrap()) // number of data units to transfer = number of ADC channels being used
    );
    peripherals.dma1.ccr1().modify(|_, w| w
        .mem2mem().clear_bit() // peripheral-to-memory mode
        .pl().low() // regular priority
        .dir().peripheral_to_memory()
        .circ().set_bit() // circular mode on
        .pinc().clear_bit() // don't increment peripheral address
        .minc().set_bit() // increment memory address
        .psize().bits_32() // peripheral sends 32 bits
        .msize().bits_32() // we receive 32 bits
        .htie().clear_bit() // no interrupt at half-transfer
        .tcie().clear_bit() // no interrupt at transfer complete
        .teie().clear_bit() // no interrupt at transfer error
    );

    // configure DMAMUX, connecting ADC to DMA1 channel 1
    peripherals.dmamux.c0cr().modify(|_, w| w
        .dmareq_id().adc() // ADC as the soure
        .soie().clear_bit() // disable synchronization overrun interrupt
        .ege().set_bit() // enable event generation
        .se().clear_bit() // disable synchronization
    );

    // enable DMA channel
    peripherals.dma1.ccr1().modify(|_, w| w
        .en().set_bit() // enable this channel
    );

    // configure particulars
    peripherals.adc.cfgr1().modify(|_, w| w
        .cont().set_bit() // compare continuously
        .discen().clear_bit() // don't compare discontinuously
        .exten().software() // software trigger only
        .res().bits_12() // 12-bit resolution
        .align().right() // align sampled data at LSB
        .ovrmod().set_bit() // overwrite data if not read on time
        .dmaen().set_bit() // enable direct memory access
        .dmacfg().circular() // circular mode
    );

    // go
    peripherals.adc.cr().modify(|_, w| w
        .adstart().set_bit()
    );
}
