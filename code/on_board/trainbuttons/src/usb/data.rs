macro_rules! wlb {
    ($b:expr) => ((((($b as u16)) >> 0) & 0xFF) as u8);
}
macro_rules! whb {
    ($b:expr) => ((((($b as u16)) >> 8) & 0xFF) as u8);
}

pub const ID_VENDOR: u16 = 0x0466; // obsolete code for HP
pub const ID_PRODUCT: u16 = 0x6900;
pub const BCD_DEVICE: u16 = 0x0000; // device revision

pub const DEVICE_DESCRIPTOR: [u8; 18] = [
    18, // bLength
    1, // bDescriptorType == DEVICE
    wlb!(0x0200), whb!(0x0200), // bcdUSB == 0x0200
    0x00, // bDeviceClass == ask_the_interface
    0x00, // bDeviceSubClass == ask_the_interface
    0x00, // bDeviceProtocol == ask_the_interface
    64, // bMaxPacketSize0 == 64 (must be 8/16/32/64)
    wlb!(ID_VENDOR), whb!(ID_VENDOR), // idVendor
    wlb!(ID_PRODUCT), whb!(ID_PRODUCT), // idProduct
    wlb!(BCD_DEVICE), whb!(BCD_DEVICE), // bcdDevice
    1, // iManufacturer -- index of manufacturer name string
    2, // iProduct -- index of device name string
    0, // iSerialNumber -- index of serial number string (no serial number on this one)
    1, // bNumConfigurations -- number of available configurations
];

const CONFIG_1_DESCRIPTOR_LEN: usize = 34;
pub const CONFIG_1_DESCRIPTOR: [u8; CONFIG_1_DESCRIPTOR_LEN] = [
    // first section: configuration descriptor
    9, // bLength
    2, // bDescriptorType == CONFIGURATION
    wlb!(CONFIG_1_DESCRIPTOR_LEN), whb!(CONFIG_1_DESCRIPTOR_LEN), // wTotalLength (of all descriptors)
    1, // bNumInterfaces
    1, // bConfigurationValue
    0, // iConfiguration (no string)
    0b1000_0000, // bmAttributes
    150, // bMaxPower == 300 mA

    // second section: interface descriptor
    9, // bLength
    4, // bDescriptorType == INTERFACE
    0, // bInterfaceNumber
    0, // bAlternateSetting
    1, // bNumEndpoints (apart from control endpoint 0)
    0x03, // bInterfaceClass == HID
    0, // bInterfaceSubClass
    0, // bInterfaceProtocol
    0, // iInterface (no string)

    // third section: HID descriptor
    9, // bLength
    0x21, // bDescriptorType == HID
    wlb!(0x0111), whb!(0x0111), // bcdHID == 1.11
    0, // bCountryCode
    1, // bNumDescriptors
    0x22, // bDescriptorType == HID_REPORT
    wlb!(HID_REPORT.len()), whb!(HID_REPORT.len()), // wDescriptorLength

    // fourth section: endpoint descriptor
    7, // bLength
    5, // bDescriptorType == ENDPOINT
    0b1000_0000 | 1, // bEndpointAddress == IN (device-to-host) endpoint 1
    0b11 | (0b00 << 4), // bmAttributes = INTERRUPT_PERIODIC
    wlb!(64), whb!(64), // wMaxPacketSize
    1, // bInterval
];

pub const STRING_DESCRIPTOR_0: [u8; 4] = [
    4, // bLength
    3, // bDescriptorType == STRING
    wlb!(1033), whb!(1033), // English (US)
];

/// Length of the longest string in [STRINGS] encoded in UTF-16LE + 2 bytes header
pub const MAX_STRING_DESCRIPTOR_BUF: usize = 36;
pub const STRINGS: [&str; 2] = [
    "Horrible Products",
    "Train Buttons",
];

macro_rules! hid_short_header {
    (@size, bytes0) => (0b00);
    (@size, bytes1) => (0b01);
    (@size, bytes2) => (0b10);
    (@size, bytes4) => (0b11);

    (@type, main) => ((0b00 << 2));
    (@type, global) => ((0b01 << 2));
    (@type, local) => ((0b10 << 2));
    (@type, reserved) => ((0b11 << 2));

    (@tag, $t:expr) => (($t << 4));

    ($size:ident, $type:ident, $tag:expr) => ((
        (hid_short_header!(@size, $size))
        | (hid_short_header!(@type, $type))
        | (hid_short_header!(@tag, $tag))
    ));
}

macro_rules! hid_input_flags {
    (@dc, data) => (0b0);
    (@dc, constant) => (0b1);

    (@av, array) => ((0b0 << 1));
    (@av, variable) => ((0b1 << 1));

    (@ar, absolute) => ((0b0 << 2));
    (@ar, relative) => ((0b1 << 2));

    ($dc:ident, $av:ident, $ar:ident) => ((
        (hid_input_flags!(@dc, $dc))
        | (hid_input_flags!(@av, $av))
        | (hid_input_flags!(@ar, $ar))
    ));
}

const HID_USAGE_PAGE: u8 = hid_short_header!(bytes1, global, 0);
const HID_USAGE: u8 = hid_short_header!(bytes1, local, 0);
const HID_COLLECTION: u8 = hid_short_header!(bytes1, main, 10);
const HID_END_COLLECTION: u8 = hid_short_header!(bytes0, main, 12);
const HID_USAGE_MINIMUM: u8 = hid_short_header!(bytes1, local, 1);
const HID_USAGE_MAXIMUM: u8 = hid_short_header!(bytes1, local, 2);
const HID_LOGICAL_MINIMUM: u8 = hid_short_header!(bytes1, global, 1);
const HID_LOGICAL_MAXIMUM: u8 = hid_short_header!(bytes1, global, 2);
const HID_REPORT_SIZE: u8 = hid_short_header!(bytes1, global, 7);
const HID_REPORT_COUNT: u8 = hid_short_header!(bytes1, global, 9);
const HID_INPUT: u8 = hid_short_header!(bytes1, main, 8);
const HID_DATA_VAR_ABSOLUTE: u8 = hid_input_flags!(data, variable, absolute);
const HID_CONSTANT: u8 = hid_input_flags!(constant, array, absolute);

pub const HID_REPORT: [u8; 32] = [
    HID_USAGE_PAGE, 0x01, // generic desktop controls
    HID_USAGE, 0x05, // gamepad
    HID_COLLECTION, 0x01, // application
        HID_COLLECTION, 0x00, // physical
            HID_USAGE_PAGE, 0x09, // buttons
            HID_USAGE_MINIMUM, 1, // button 1
            HID_USAGE_MAXIMUM, 22, // button 22
            HID_LOGICAL_MINIMUM, 0x00, // button not pressed
            HID_LOGICAL_MAXIMUM, 0x01, // button pressed
            HID_REPORT_SIZE, 0x01, // bits per report (= 1 bit 0/1)
            HID_REPORT_COUNT, 21, // number of reports (= buttons)
            HID_INPUT, HID_DATA_VAR_ABSOLUTE, // add the buttons
            HID_REPORT_SIZE, 3, // padding bits to get up to a full byte
            HID_REPORT_COUNT, 1, // one of those padding reports
            HID_INPUT, HID_CONSTANT,
        HID_END_COLLECTION,
    HID_END_COLLECTION,
];
