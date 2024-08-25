use stm32g0b0::Peripherals;


pub(crate) fn set_up(peripherals: &Peripherals) {
    // use peripheral clock (equal to undivided HSE = 48 MHz)
    peripherals.rcc.ccipr().modify(|_, w| w
        .usart1sel().pclk()
    );

    // send clock to USART1
    peripherals.rcc.apbenr2().modify(|_, w| w
        .usart1en().set_bit()
    );

    // also send clock to GPIOA
    peripherals.rcc.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
    );

    // configure pins: PA9 = Tx, PA10 = Rx (alternative function 1)
    peripherals.gpioa.moder().modify(|_, w| w
        .moder9().alternate_function()
        .moder10().alternate_function()
    );
    peripherals.gpioa.afrh().modify(|_, w| w
        .afsel9().af1()
        .afsel10().af1()
    );

    // disable USART1 to reconfigure it
    peripherals.usart1.cr1_fifo_disabled().modify(|_, w| w
        .ue().clear_bit()
    );

    // set up USART1
    peripherals.usart1.cr1_fifo_disabled().modify(|_, w| w
        .uesm().clear_bit() // disable USART in sleep mode
        .re().clear_bit() // disable receiver
        .te().clear_bit() // disable transmitter (initially)
        .idleie().clear_bit() // disable "idle" interrupt
        .pce().clear_bit() // disable parity calculation
        .m0().clear_bit() // 8 bits per byte (1/2)
        .m1().clear_bit() // 8 bits per byte (2/2)
        .mme().clear_bit() // disable mute mode
        .over8().clear_bit() // 16-bit oversampling
        .fifoen().clear_bit() // disable FIFO mode

        .rxneie().clear_bit() // disable "receive data register not empty" interrupt
        .tcie().clear_bit() // disable "transmission complete" interrupt
        .txeie().clear_bit() // disable "transmit data register empty" interrupt
        .peie().clear_bit() // disable "parity error" interrupt
        .cmie().clear_bit() // disable "character match" interrupt
        .rtoie().clear_bit() // disable "receiver timeout" interrupt
        .eobie().clear_bit() // disable "end of block" interrupt
    );
    peripherals.usart1.cr2().modify(|_, w| w
        .slven().clear_bit() // disable synchronous slave mode
        .lbdie().clear_bit() // disable "break detection" interrupt
        .clken().clear_bit() // disable SCLK pin
        .stop().stop_1() // one stop bit
        .linen().clear_bit() // disable LIN mode
        .swap().clear_bit() // do not swap Rx and Tx pins
        .rxinv().clear_bit() // do not invert Rx polarity
        .txinv().clear_bit() // do not invert Tx polarity
        .datainv().clear_bit() // do not invert data polarity
        .msbfirst().clear_bit() // send LSB first
        .abren().clear_bit() // no automatic baud rate detection
        .rtoen().clear_bit() // disable receiver timeout
    );
    peripherals.usart1.cr3().modify(|_, w| w
        .eie().clear_bit() // disable "error" interrupt
        .iren().clear_bit() // disable IrDA mode
        .hdsel().clear_bit() // disable half-duplex
        .nack().clear_bit() // disable smartcard NACK
        .scen().clear_bit() // disable smartcard mode
        .dmar().clear_bit() // disable reception Direct Memory Access
        .dmat().clear_bit() // disable transmittion DMA
        .rtse().clear_bit() // disable RTS hardware flow control
        .ctse().clear_bit() // disable CTS hardware flow control
        .ctsie().clear_bit() // disable "CTS" interrupt
        .onebit().clear_bit() // disable one-bit sampling method
        .ovrdis().clear_bit() // disable receive overrun detection
        .dem().clear_bit() // disable Driver Enable mode
        .wufie().clear_bit() // disable "wake-up from low-power mode" interrupt
        .txftie().clear_bit() // disable "TXFIFO threshold" interrupt
        .tcbgtie().clear_bit() // disable "transmission complete before guard time" interrupt
        .rxftie().clear_bit() // disable "RXFIFO threshold" interrupt
    );

    // set baud rate
    // 48_000_000 Hz / x = 115_200 b/s
    // x = 416
    peripherals.usart1.brr().modify(|_, w| w
        .brr().set(416)
    );

    // enable USART
    peripherals.usart1.cr1_fifo_disabled().modify(|_, w| w
        .ue().set_bit()
    );

    // enable transmitter
    peripherals.usart1.cr1_fifo_disabled().modify(|_, w| w
        .te().set_bit()
    );
}

pub(crate) fn write_bytes(peripherals: &Peripherals, bytes: &[u8]) {
    for &b in bytes {
        // wait for transmission register to empty
        while peripherals.usart1.isr_fifo_disabled().read().txe().bit_is_clear() {
        }

        peripherals.usart1.tdr().modify(|_, w| w
            .tdr().set(b.into())
        );
    }

    // wait for transmission register to empty
    while peripherals.usart1.isr_fifo_disabled().read().txe().bit_is_clear() {
    }

    // wait until transmission is complete
    while peripherals.usart1.isr_fifo_disabled().read().tc().bit_is_clear() {
    }
}

fn nibble_to_hex(nibble: u8) -> u8 {
    match nibble {
        0..=9 => b'0' + nibble,
        10..=15 => b'A' - 10 + nibble,
        _ => 0,
    }
}

fn byte_to_hex(byte: u8) -> [u8; 2] {
    [
        nibble_to_hex(byte >> 4),
        nibble_to_hex(byte & 0xF),
    ]
}

pub(crate) fn write_hex_dump(peripherals: &Peripherals, bytes: &[u8]) {
    for &b in bytes {
        for hex in byte_to_hex(b) {
            // wait for transmission register to empty
            while peripherals.usart1.isr_fifo_disabled().read().txe().bit_is_clear() {
            }

            peripherals.usart1.tdr().modify(|_, w| w
                .tdr().set(hex.into())
            );
        }
    }

    // wait for transmission register to empty
    while peripherals.usart1.isr_fifo_disabled().read().txe().bit_is_clear() {
    }

    // wait until transmission is complete
    while peripherals.usart1.isr_fifo_disabled().read().tc().bit_is_clear() {
    }
}
