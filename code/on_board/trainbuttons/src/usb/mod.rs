//! Routines that allow us to act as a Universal Serial Bus device.
//!
//! Why are USB peripherals on microcontrollers always so hard to work with?

pub(crate) mod chepnr;
mod data;
mod packets;


use cortex_m::asm::nop;
use cortex_m::peripheral::NVIC;
use stm32g0b0::{interrupt, Interrupt, Peripherals};

use crate::usb::chepnr::modify_chepnr;
use crate::usb::data::{
    CONFIG_1_DESCRIPTOR, DEVICE_DESCRIPTOR, HID_REPORT, MAX_STRING_DESCRIPTOR_BUF,
    STRING_DESCRIPTOR_0, STRINGS,
};
use crate::usb::packets::SetupSlicer;


// we only ever use two endpoints in this code
const USED_ENDPOINT_COUNT: usize = 2;

// controller-specific values; these are for STM32G0x0:
const ENDPOINT_CONFIG_COUNT: usize = 8;
const USB_PACKET_RAM_BASE: *mut u8 = 0x4000_9800 as *mut u8; // USB_DRD_PMAADDR or USB_DRD_PMA_BUFF
const USB_PACKET_RAM_SIZE: usize = 2*1024; // USB_DRD_PMA_SIZE

// packet data RAM layout:
//
// 4B endpoint 0 Tx config
// 4B endpoint 0 Rx config
// 4B endpoint 1 Tx config (if applicable)
// 4B endpoint 1 Rx config (if applicable)
// ...
// endpoint 0 Tx buffer
// endpoint 0 Rx buffer
// endpoint 1 Tx buffer (if applicable)
// endpoint 1 Rx buffer (if applicable)
// ...
//
// buffers are apportioned equally among endpoints

const PACKET_DATA_RAM_OFFSET: usize = 2 * 4 * ENDPOINT_CONFIG_COUNT; // (rx+tx) * 4 bytes/register * endpoints
const PACKET_DATA_RAM_SIZE: usize = USB_PACKET_RAM_SIZE - PACKET_DATA_RAM_OFFSET;

const EP_BUF_SIZE: usize = PACKET_DATA_RAM_SIZE / (USED_ENDPOINT_COUNT * 2);


static mut SETTING_ADDRESS: Option<u8> = None;
static mut EXPECTING_STATUS: bool = false;
static mut RESETTING_DATA_TOGGLES: bool = false;


const fn get_usb_endpoint_tx_offset(endpoint: usize) -> usize {
    PACKET_DATA_RAM_OFFSET + 2*endpoint*EP_BUF_SIZE
}
const fn get_usb_endpoint_rx_offset(endpoint: usize) -> usize {
    PACKET_DATA_RAM_OFFSET + (2*endpoint + 1)*EP_BUF_SIZE
}


pub(crate) fn enable_peripheral_clocks(peripherals: &Peripherals) {
    // plug HSE into USB clock
    peripherals.rcc.ccipr2().modify(|_, w| w
        .usbsel().hse()
    );

    // send power to the USB macrocell
    peripherals.pwr.cr2().modify(|_, w| w
        .usv().set_bit()
    );

    // send a clock to the USB macrocell
    peripherals.rcc.apbenr1().modify(|_, w| w
        .usben().set_bit()
    );
}

pub(crate) fn start_reset(peripherals: &Peripherals) {
    // reset USB
    peripherals.rcc.apbrstr1().modify(|_, w| w
        .usbrst().set_bit()
    );
}

pub(crate) fn stop_reset(peripherals: &Peripherals) {
    // disable reset of USB
    peripherals.rcc.apbrstr1().modify(|_, w| w
        .usbrst().clear_bit()
    );
}


fn copy_to_endpoint_tx_buffer(endpoint: usize, data: &[u8]) {
    // data must be transferred in units of 4 bytes
    // otherwise we get spurious bit errors
    let ep_buffer = unsafe {
        core::slice::from_raw_parts_mut(
            USB_PACKET_RAM_BASE.wrapping_add(get_usb_endpoint_tx_offset(endpoint)) as *mut u32,
            EP_BUF_SIZE / 4
        )
    };
    let mut iterations = data.len() / 4;
    if data.len() % 4 != 0 {
        iterations += 1;
    }

    for i in 0..iterations {
        let mut value = (data[4*i + 0] as u32) <<  0;
        if data.len() > 4*i + 1 {
            value |= (data[4*i + 1] as u32) <<  8;
        }
        if data.len() > 4*i + 2 {
            value |= (data[4*i + 2] as u32) << 16;
        }
        if data.len() > 4*i + 3 {
            value |= (data[4*i + 3] as u32) << 24;
        }
        ep_buffer[i] = value;
    }
}

fn copy_from_endpoint_rx_buffer(endpoint: usize, data: &mut [u8]) {
    // data must be transferred in units of 4 bytes
    // otherwise we get spurious bit errors
    let ep_buffer = unsafe {
        core::slice::from_raw_parts(
            USB_PACKET_RAM_BASE.wrapping_add(get_usb_endpoint_rx_offset(endpoint)) as *mut u32,
            EP_BUF_SIZE / 4
        )
    };
    let mut i = 0;
    for &chunk in ep_buffer {
        if i >= data.len() { break; }
        data[i] = ((chunk >>  0) & 0xFF) as u8;
        i += 1;
        if i >= data.len() { break; }
        data[i] = ((chunk >>  8) & 0xFF) as u8;
        i += 1;
        if i >= data.len() { break; }
        data[i] = ((chunk >> 16) & 0xFF) as u8;
        i += 1;
        if i >= data.len() { break; }
        data[i] = ((chunk >> 24) & 0xFF) as u8;
        i += 1;
    }
}


/// Clears only some USB interrupt bits.
///
/// Use the `writer` to clear those bits that should be set to zero.
fn clear_only_some_usb_interrupts<F>(peripherals: &Peripherals, writer: F)
    where
        for<'w> F: FnOnce(&'w mut stm32g0b0::generic::W<stm32g0b0::usb::istr::IstrSpec>) -> &'w mut stm32g0b0::generic::W<stm32g0b0::usb::istr::IstrSpec>, {
    peripherals.usb.istr().write(|w| {
        writer(
            w
                .l1req().set_bit()
                .esof().set_bit()
                .sof().set_bit()
                .rst_dcon().set_bit()
                .susp().set_bit()
                .wkup().set_bit()
                .err().set_bit()
                .pmaovr().set_bit()
                .thr512().set_bit()
        )
    })
}

#[interrupt]
fn USB() {
    crate::usb::distribute_usb_interrupt();
}


pub fn distribute_usb_interrupt() {
    let mut peripherals = unsafe { Peripherals::steal() };
    // theoretically, SYSCFG.ITLINE8.USB should be set
    // practically, it is not
    // however, line 8 on STM32G0x0 is only triggered by USB
    handle_usb_interrupt(&mut peripherals);
}

fn handle_usb_interrupt(peripherals: &Peripherals) {
    while peripherals.usb.istr().read().ctr().bit_is_set() {
        let endpoint = peripherals.usb.istr().read().idn().bits();
        let endpoint_register = peripherals.usb.chepnr(endpoint.into());
        let endpoint_register_state = endpoint_register.read();
        let received = peripherals.usb.istr().read().dir().bit_is_set();

        let rx_set = endpoint_register_state.vtrx().bit_is_set();
        let tx_set = endpoint_register_state.vttx().bit_is_set();
        let is_setup = endpoint_register_state.setup().bit_is_set();
        if received && rx_set {
            // clear VTRX bit
            modify_chepnr(endpoint_register, |m| m
                .reset_vtrx()
            );

            if is_setup {
                // SETUP packet
                if endpoint == 0 {
                    let read_bytes: usize = peripherals.usb_ram1.single_buffered(0).chep_rxtxbd_0().read().count_rx().bits().into();
                    let mut buf = [0u8; EP_BUF_SIZE];
                    copy_from_endpoint_rx_buffer(endpoint.into(), &mut buf);

                    crate::uart::write_bytes(peripherals, b"USB setup: >");
                    crate::uart::write_hex_dump(peripherals, &buf[..read_bytes]);
                    crate::uart::write_bytes(peripherals, b"<\r\n");

                    // reset Rx buffer for next transmission
                    prepare_rx_buffer(peripherals, 0);

                    // stall Rx until we have sent a response
                    modify_chepnr(endpoint_register, |m| m
                        .statrx_stall()
                    );

                    let setup_slicer = SetupSlicer::new(&buf[..read_bytes]);
                    if setup_slicer.direction_is_device_to_host() && setup_slicer.recipient_is_device() && setup_slicer.type_is_standard() && setup_slicer.request_is_get_descriptor() {
                        // GET DESCRIPTOR on device
                        let mut handled = false;
                        if setup_slicer.descriptor_type_is_device() {
                            let want_len = DEVICE_DESCRIPTOR.len().min(setup_slicer.length_usize());
                            transmit_packet(peripherals, 0, &DEVICE_DESCRIPTOR[..want_len]);
                            handled = true;
                        } else if setup_slicer.descriptor_type_is_configuration() {
                            let want_len = CONFIG_1_DESCRIPTOR.len().min(setup_slicer.length_usize());
                            transmit_packet(peripherals, 0, &CONFIG_1_DESCRIPTOR[..want_len]);
                            handled = true;
                        } else if setup_slicer.descriptor_type_is_string() {
                            if setup_slicer.descriptor_index() == 0 {
                                let want_len = STRING_DESCRIPTOR_0.len().min(setup_slicer.length_usize());
                                transmit_packet(peripherals, 0, &STRING_DESCRIPTOR_0[..want_len]);
                                handled = true;
                            } else {
                                let want_index: usize = (setup_slicer.descriptor_index() - 1).into();
                                if want_index < STRINGS.len() {
                                    // that fits
                                    let mut out_buf = [0u8; MAX_STRING_DESCRIPTOR_BUF];

                                    // encode string as UTF-8
                                    let mut string_len = 2;
                                    for w in STRINGS[want_index].encode_utf16() {
                                        let bs = w.to_le_bytes();
                                        out_buf[string_len] = bs[0];
                                        out_buf[string_len+1] = bs[1];
                                        string_len += 2;
                                    }

                                    // set length
                                    out_buf[0] = string_len.try_into().unwrap();

                                    // set type
                                    out_buf[1] = 3; // string descriptor

                                    // send
                                    let want_len = string_len.min(setup_slicer.length_usize());
                                    transmit_packet(peripherals, 0, &out_buf[..want_len]);
                                    handled = true;
                                }
                            }
                        }

                        if handled {
                            // wait for confirmation
                            unsafe { EXPECTING_STATUS = true };
                        } else {
                            // nope, can't deal with this one
                            modify_chepnr(endpoint_register, |m| m
                                .stattx_stall()
                            );
                        }
                    } else if setup_slicer.direction_is_device_to_host() && setup_slicer.recipient_is_interface() && setup_slicer.type_is_standard() && setup_slicer.request_is_get_descriptor() {
                        // GET DESCRIPTOR on interface
                        let mut handled = false;
                        if setup_slicer.descriptor_type_is(0x22) {
                            // HID report
                            let want_len = HID_REPORT.len().min(setup_slicer.length_usize());
                            transmit_packet(peripherals, 0, &HID_REPORT[..want_len]);
                            handled = true;
                        }

                        if handled {
                            // wait for confirmation
                            unsafe { EXPECTING_STATUS = true };
                        } else {
                            // nope, can't deal with this one
                            modify_chepnr(endpoint_register, |m| m
                                .stattx_stall()
                            );
                        }
                    } else if setup_slicer.direction_is_host_to_device() && setup_slicer.recipient_is_device() && setup_slicer.type_is_standard() && setup_slicer.request_is_set_address() {
                        // SET ADDRESS
                        let address = u16::from_le_bytes([buf[2], buf[3]]);
                        if address < 128 {
                            // remember this for once the response has been sent
                            unsafe { SETTING_ADDRESS = Some(address.try_into().unwrap()) };

                            // say we're done
                            transmit_packet(peripherals, 0, &[]);
                        } else {
                            // nope
                            modify_chepnr(endpoint_register, |m| m
                                .stattx_stall()
                            );
                        }
                    } else if setup_slicer.direction_is_host_to_device() && setup_slicer.recipient_is_device() && setup_slicer.type_is_standard() && setup_slicer.request_is_set_configuration() {
                        // SET CONFIGURATION
                        let config_value = u16::from_le_bytes([buf[2], buf[3]]);
                        if config_value == 1 {
                            // sure
                            unsafe { RESETTING_DATA_TOGGLES = true };
                            transmit_packet(peripherals, 0, &[]);
                        } else {
                            // nope
                            modify_chepnr(endpoint_register, |m| m
                                .stattx_stall()
                            );
                        }
                    } else {
                        // can't handle whatever this is
                        modify_chepnr(endpoint_register, |m| m
                            .stattx_stall()
                        );
                    }

                    // read the next response
                    modify_chepnr(endpoint_register, |m| m
                        .statrx_valid()
                    );
                } else if usize::from(endpoint) < USED_ENDPOINT_COUNT {
                    let read_bytes: usize = peripherals.usb_ram1.single_buffered(endpoint.into()).chep_rxtxbd_0().read().count_rx().bits().into();
                    let mut buf = [0u8; EP_BUF_SIZE];
                    copy_from_endpoint_rx_buffer(endpoint.into(), &mut buf);
                    crate::uart::write_bytes(peripherals, b"received USB SETUP on endpoint 0x");
                    crate::uart::write_hex_dump(peripherals, &[endpoint]);
                    crate::uart::write_bytes(peripherals, b": >");
                    crate::uart::write_hex_dump(peripherals, &buf[..read_bytes]);
                    crate::uart::write_bytes(peripherals, b"<\r\n");

                    prepare_rx_buffer(peripherals, endpoint.into());

                    // get ready to receive again
                    modify_chepnr(endpoint_register, |w| w
                        .statrx_valid()
                    );
                }
            } else {
                // non-SETUP packet
                let read_bytes: usize = peripherals.usb_ram1.single_buffered(endpoint.into()).chep_rxtxbd_0().read().count_rx().bits().into();
                if usize::from(endpoint) < USED_ENDPOINT_COUNT {
                    let is_expecting_status = unsafe { EXPECTING_STATUS };
                    if is_expecting_status {
                        if read_bytes != 0 {
                            crate::uart::write_bytes(peripherals, b"expecting status but received data!\r\n");
                            modify_chepnr(endpoint_register, |w| w
                                .stattx_stall()
                            );
                        } else {
                            unsafe { EXPECTING_STATUS = false };

                            // reset Rx buffer for next transmission
                            prepare_rx_buffer(peripherals, endpoint.into());

                            // get ready to receive again
                            modify_chepnr(endpoint_register, |w| w
                                .statrx_valid()
                            );
                        }
                    } else {
                        // read it out
                        let mut buf = [0u8; EP_BUF_SIZE];
                        copy_from_endpoint_rx_buffer(endpoint.into(), &mut buf[..read_bytes]);
                        crate::uart::write_bytes(peripherals, b"received on endpoint 0x");
                        crate::uart::write_hex_dump(peripherals, &[endpoint]);
                        crate::uart::write_bytes(peripherals, b" via USB: >");
                        crate::uart::write_hex_dump(peripherals, &buf[..read_bytes]);
                        crate::uart::write_bytes(peripherals, b"<\r\n");

                        // reset Rx buffer for next transmission
                        prepare_rx_buffer(peripherals, endpoint.into());

                        // get ready to receive again
                        modify_chepnr(endpoint_register, |w| w
                            .statrx_valid()
                        );
                    }
                } else {
                    crate::uart::write_bytes(peripherals, b"non-SETUP packet to unexpected endpoint\r\n");

                    // clear the Rx flag and complain
                    modify_chepnr(endpoint_register, |w| w
                        .stattx_stall()
                    );
                }
            }

            // process all Rx before we start on Tx
            continue;
        }
        if tx_set {
            // clear the Tx flag
            modify_chepnr(endpoint_register, |w| w
                .reset_vttx()
            );

            if endpoint == 0 {
                // are we supposed to change our address?
                let set_addr_opt = unsafe { SETTING_ADDRESS };
                if let Some(set_addr) = set_addr_opt {
                    // yes
                    peripherals.usb.daddr().modify(|_, w| w
                        .add().set(set_addr)
                    );

                    // don't do this again
                    unsafe { SETTING_ADDRESS = None };
                }

                // are we supposed to reset our data toggles?
                let reset_data_toggles = unsafe { RESETTING_DATA_TOGGLES };
                if reset_data_toggles {
                    for n in 0..USED_ENDPOINT_COUNT {
                        let ep_type = peripherals.usb.chepnr(n).read().utype();
                        if ep_type.is_bulk() || ep_type.is_interrupt() {
                            // yup; reset the flags
                            modify_chepnr(peripherals.usb.chepnr(n), |m| m
                                .dtogtx(false)
                                .dtogrx(false)
                            );
                        }
                    }

                    // don't do this again
                    unsafe { RESETTING_DATA_TOGGLES = false };
                }
            } else if endpoint == 1 {
                // endpoint 1 just sent off its current state

                // update button states
                // 21 bits of buttons
                // 12 bits of X axis
                let all_button_states: u64 = crate::pins::read_button_states(peripherals).into();
                let x_axis: u64 = unsafe { crate::adc::ADC_VALUES[0] }.into();
                let report =
                    (all_button_states << 0)
                    | (x_axis << 21)
                ;

                // reports are filled up from the least significant bit,
                // but sent out in little-endian order
                copy_to_endpoint_tx_buffer(1, &report.to_le_bytes()[0..5]);

                // ready to send five bytes again
                peripherals.usb_ram1.single_buffered(1).chep_txrxbd_0().modify(|_, w| w
                    .addr_tx().set(get_usb_endpoint_tx_offset(1).try_into().unwrap())
                    .count_tx().set(5)
                );
                modify_chepnr(endpoint_register, |m| m
                    .stattx_valid()
                );
            }
        }
    }
    if peripherals.usb.istr().read().rst_dcon().bit_is_set() {
        // USB reset received
        crate::uart::write_bytes(peripherals, b"USB reset\r\n");
        post_reset_setup(peripherals);
    }
    if peripherals.usb.istr().read().pmaovr().bit_is_set() {
        // packet memory area overrun
        crate::uart::write_bytes(peripherals, b"USB packet memory area overrun\r\n");
        // oh well, clear the flag
        clear_only_some_usb_interrupts(peripherals, |w| w.pmaovr().clear_bit());
    }
    if peripherals.usb.istr().read().err().bit_is_set() {
        // error received
        // oh well, clear the flag
        clear_only_some_usb_interrupts(peripherals, |w| w.err().clear_bit());
    }
    if peripherals.usb.istr().read().wkup().bit_is_set() {
        // wakeup received

        // resume USB peripheral
        peripherals.usb.cntr().modify(|_, w| w
            .suspen().clear_bit()
        );

        // clear interrupt
        // (can only happen after peripheral has been resumed)
        clear_only_some_usb_interrupts(peripherals, |w| w.wkup().clear_bit());
    }
    if peripherals.usb.istr().read().susp().bit_is_set() {
        // suspend received

        // suspend USB peripheral
        peripherals.usb.cntr().modify(|_, w| w
            .suspen().set_bit()
        );

        // clear interrupt
        // (can only happen after peripheral has been suspended)
        clear_only_some_usb_interrupts(peripherals, |w| w.susp().clear_bit());
    }
    if peripherals.usb.istr().read().sof().bit_is_set() {
        // Start of Frame received
        // clear the flag
        clear_only_some_usb_interrupts(peripherals, |w| w.sof().clear_bit());
    }
    if peripherals.usb.istr().read().esof().bit_is_set() {
        // expected Start of Frame, didn't get it
        // oh well, clear the flag
        clear_only_some_usb_interrupts(peripherals, |w| w.esof().clear_bit());
    }
}


fn transmit_packet(peripherals: &Peripherals, endpoint: usize, data: &[u8]) {
    // copy over
    copy_to_endpoint_tx_buffer(endpoint, data);

    // set length
    peripherals.usb_ram1.single_buffered(endpoint).chep_txrxbd_0().modify(|_, w| w
        .addr_tx().set(get_usb_endpoint_tx_offset(endpoint).try_into().unwrap())
        .count_tx().set(data.len().try_into().unwrap())
    );

    // fire
    modify_chepnr(peripherals.usb.chepnr(endpoint), |w| w
        .stattx_valid()
    );
}


fn prepare_rx_buffer(peripherals: &Peripherals, endpoint: usize) {
    if EP_BUF_SIZE > 62 {
        peripherals.usb_ram1.single_buffered(endpoint).chep_rxtxbd_0().modify(|_, w| w
            .addr_rx().set(get_usb_endpoint_rx_offset(endpoint).try_into().unwrap())
            .count_rx().set(0)
            .num_block().set((EP_BUF_SIZE/32 - 1).try_into().unwrap())
            .blsize().set_bit()
        );
    } else {
        peripherals.usb_ram1.single_buffered(endpoint).chep_rxtxbd_0().modify(|_, w| w
            .addr_rx().set(get_usb_endpoint_rx_offset(endpoint).try_into().unwrap())
            .count_rx().set(0)
            .num_block().set((EP_BUF_SIZE/2).try_into().unwrap())
            .blsize().clear_bit()
        );
    }
}


/// Setup operations to be performed before the first activation as well as after every reset.
fn post_reset_setup(peripherals: &Peripherals) {
    // clear a couple of interrupts we are not interested in
    clear_only_some_usb_interrupts(peripherals, |w| w
        .l1req().clear_bit()
        .rst_dcon().clear_bit()
        .thr512().clear_bit()
    );

    // define space for Ep0 and Ep1 Tx buffers
    peripherals.usb_ram1.single_buffered(0).chep_txrxbd_0().modify(|_, w| w
        .addr_tx().set(get_usb_endpoint_tx_offset(0).try_into().unwrap())
        .count_tx().set(0)
    );
    peripherals.usb_ram1.single_buffered(1).chep_txrxbd_0().modify(|_, w| w
        .addr_tx().set(get_usb_endpoint_tx_offset(1).try_into().unwrap())
        .count_tx().set(0)
    );
    prepare_rx_buffer(peripherals, 0);
    prepare_rx_buffer(peripherals, 1);

    // set up Ep0 buffer
    modify_chepnr(peripherals.usb.chepnr(0), |w| w
        .endpoint_address(0x0) // endpoint 0
        .utype_control() // it's a control endpoint
        .statrx_valid() // ready to receive
        .stattx_nak() // nothing to send
        .dtogrx(false) // set DTOGRX (reception data toggle) to 0
        .dtogtx(false) // set DTOGTX (transmission data toggle) to 0
    );

    // set up Ep1 buffer
    copy_to_endpoint_tx_buffer(1, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    peripherals.usb_ram1.single_buffered(1).chep_txrxbd_0().modify(|_, w| w
        .addr_tx().set(get_usb_endpoint_tx_offset(1).try_into().unwrap())
        .count_tx().set(5)
    );
    modify_chepnr(peripherals.usb.chepnr(1), |w| w
        .endpoint_address(0x1) // endpoint 1
        .utype_interrupt() // it's an interrupt endpoint
        .statrx_disabled() // disabled for the time being
        .stattx_disabled() // disabled for the time being
        .dtogrx(false) // set DTOGRX (reception data toggle) to 0
        .dtogtx(false) // set DTOGTX (transmission data toggle) to 0
        .reset_nak()
    );
    modify_chepnr(peripherals.usb.chepnr(1), |w| w
        .statrx_nak() // nothing to receive
        .stattx_valid() // ready to send
    );

    // enable USB function (no address assigned yet)
    peripherals.usb.daddr().modify(|_, w| w
        .ef().set_bit()
        .add().set(0x00)
    );
}


/// Sets up USB device support.
///
/// Assumes that clocks have already been set up using [`crate::clock::set_up`].
pub(crate) fn set_up(peripherals: &Peripherals) {
    // enable USB interrupt with priority 3
    unsafe {
        let mut core_peripherals = cortex_m::Peripherals::steal();
        core_peripherals.NVIC.set_priority(Interrupt::USB, 3);
        NVIC::unmask(Interrupt::USB);
    }
    for _ in 0..8 {
        nop();
    }

    // enable USB reset and power-down condition
    peripherals.usb.cntr().modify(|_, w| w
        .usbrst().set_bit()
        .pdwn().set_bit()
    );
    for _ in 0..8 {
        nop();
    }

    // enable USB transceiver
    peripherals.usb.cntr().modify(|_, w| w
        .pdwn().clear_bit()
    );

    // wait t_{STARTUP}
    // RM says it's specified in DS, nothing relevant in DS
    // ST case 00202347 clarifies: it's max. 1µs
    // 1µs * 48MHz = 48
    for _ in 0..48 {
        nop();
    }

    // remove USB reset condition
    peripherals.usb.cntr().modify(|_, w| w
        .usbrst().clear_bit()
    );
    for _ in 0..8 {
        nop();
    }

    // remove spurious interrupt states
    peripherals.usb.istr().write(|w| unsafe { w.bits(0) });

    // configure the device
    peripherals.usb.cntr().modify(|_, w| w
        .host().clear_bit() // I'm a device
        .suspen().clear_bit() // do not suspend me
        .l2res().clear_bit() // no L2 remote wakeup/resume
        .l1res().clear_bit() // no L1 remote wakeup/resume

        // enable most interrupts
        .l1reqm().set_bit()
        .esofm().set_bit()
        .sofm().set_bit()
        .rst_dconm().set_bit()
        .suspm().set_bit()
        .wkupm().set_bit()
        .errm().set_bit()
        .ctrm().set_bit()

        // not these, though
        .pmaovrm().clear_bit()
        .thr512m().clear_bit()
    );

    /*
    // remove spurious interrupt states
    peripherals.usb.istr().write(|w| w
        .l1req().clear_bit()
        .esof().clear_bit()
        .sof().clear_bit()
        .rst_dcon().clear_bit()
        .susp().clear_bit()
        .wkup().clear_bit()
        .err().clear_bit()
        .pmaovr().clear_bit()
        .thr512().clear_bit()
    );
    */

    //post_reset_setup(peripherals);

    // say hello
    peripherals.usb.bcdr().modify(|_, w| w
        .dppu_dpd().set_bit()
    );
}
