use bitmacros::extract_bits;


pub struct SetupSlicer<'a> {
    bytes: &'a [u8],
}
impl<'a> SetupSlicer<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
        }
    }

    pub const fn recipient_is_device(&self) -> bool { extract_bits!(self.bytes[0], 0, 5) == 0 }
    pub const fn recipient_is_interface(&self) -> bool { extract_bits!(self.bytes[0], 0, 5) == 1 }
    pub const fn recipient_is_endpoint(&self) -> bool { extract_bits!(self.bytes[0], 0, 5) == 2 }
    pub const fn recipient_is_other(&self) -> bool { extract_bits!(self.bytes[0], 0, 5) == 3 }

    pub const fn type_is_standard(&self) -> bool { extract_bits!(self.bytes[0], 5, 2) == 0 }
    pub const fn type_is_class(&self) -> bool { extract_bits!(self.bytes[0], 5, 2) == 1 }
    pub const fn type_is_vendor(&self) -> bool { extract_bits!(self.bytes[0], 5, 2) == 2 }
    pub const fn type_is_reserved(&self) -> bool { extract_bits!(self.bytes[0], 5, 2) == 3 }

    pub const fn request_is_get_status(&self) -> bool { self.bytes[1] == 0 }
    pub const fn request_is_clear_feature(&self) -> bool { self.bytes[1] == 1 }
    pub const fn request_is_set_feature(&self) -> bool { self.bytes[1] == 2 }
    pub const fn request_is_set_address(&self) -> bool { self.bytes[1] == 5 }
    pub const fn request_is_get_descriptor(&self) -> bool { self.bytes[1] == 6 }
    pub const fn request_is_set_descriptor(&self) -> bool { self.bytes[1] == 7 }
    pub const fn request_is_get_configuration(&self) -> bool { self.bytes[1] == 8 }
    pub const fn request_is_set_configuration(&self) -> bool { self.bytes[1] == 9 }
    pub const fn request_is_get_interface(&self) -> bool { self.bytes[1] == 10 }
    pub const fn request_is_set_interface(&self) -> bool { self.bytes[1] == 11 }
    pub const fn request_is_synch_frame(&self) -> bool { self.bytes[1] == 12 }

    pub const fn direction_is_device_to_host(&self) -> bool { extract_bits!(self.bytes[0], 7, 1) == 1 }
    pub const fn direction_is_host_to_device(&self) -> bool { extract_bits!(self.bytes[0], 7, 1) == 0 }

    pub const fn descriptor_type_is_device(&self) -> bool { self.bytes[3] == 1 }
    pub const fn descriptor_type_is_configuration(&self) -> bool { self.bytes[3] == 2 }
    pub const fn descriptor_type_is_string(&self) -> bool { self.bytes[3] == 3 }
    pub const fn descriptor_type_is_interface(&self) -> bool { self.bytes[3] == 4 }
    pub const fn descriptor_type_is_endpoint(&self) -> bool { self.bytes[3] == 5 }
    pub const fn descriptor_type_is_device_qualifier(&self) -> bool { self.bytes[3] == 6 }
    pub const fn descriptor_type_is_other_speed_config(&self) -> bool { self.bytes[3] == 7 }
    pub const fn descriptor_type_is_interface_power(&self) -> bool { self.bytes[3] == 8 }
    pub const fn descriptor_type_is_on_the_go(&self) -> bool { self.bytes[3] == 9 }
    pub const fn descriptor_type_is(&self, expected: u8) -> bool { self.bytes[3] == expected }

    pub const fn descriptor_index(&self) -> u8 { self.bytes[2] }

    pub const fn length(&self) -> u16 { u16::from_le_bytes([self.bytes[6], self.bytes[7]]) }
    pub fn length_usize(&self) -> usize { self.length().into() }
}
