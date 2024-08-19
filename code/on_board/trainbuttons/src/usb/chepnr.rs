//! Utilities for CHEPnR (Channel/Endpoint n Register) modification.


use stm32g0b0::usb::chepnr::{Statrx, Stattx, Utype};


macro_rules! bit_mask {
    ($lowest_bit_index:expr, $bit_count:expr) => {
        (((1 << $bit_count) - 1) << $lowest_bit_index)
    };
}
macro_rules! extract_bits {
    ($register:expr, $lowest_bit_index:expr, $bit_count:expr) => {
        (($register >> $lowest_bit_index) & bit_mask!($lowest_bit_index, $bit_count))
    };
}
macro_rules! emplace_bits {
    ($register:expr, $value:expr, $lowest_bit_index:expr, $bit_count:expr) => {
        $register = ($register & (!bit_mask!($lowest_bit_index, $bit_count))) | (($value << $lowest_bit_index) & bit_mask!($lowest_bit_index, $bit_count))
    };
}


pub(crate) struct ChepnrModifier<'a> {
    chepnr: &'a stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>,
    initial_value: u32,
    set_value: u32,
}
impl<'a> ChepnrModifier<'a> {
    fn new(chepnr: &'a stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>) -> Self {
        let initial_value = chepnr.read().bits();
        let set_value =
            // transfer r/w fields 1:1
            (initial_value & 0b0000_0001_0111_1111_0000_0111_0000_1111)

            // set write-zero-to-reset fields to 1 by default
            | 0b0000_0110_1000_0000_1000_0000_1000_0000

            // keep toggle fields at zero by default
        ;
        Self {
            chepnr,
            initial_value,
            set_value,
        }
    }

    // ea = standard write
    #[allow(dead_code)]
    pub fn endpoint_address(mut self, new_ea: u8) -> Self {
        emplace_bits!(self.set_value, (new_ea as u32), 0, 4);
        self
    }

    // stattx = write 1 to toggle
    #[allow(dead_code)]
    pub fn stattx_variant(mut self, value: Stattx) -> Self {
        let initial_bits = extract_bits!(self.initial_value, 4, 2);
        let toggle_bits = initial_bits ^ ((value as u8) as u32);
        emplace_bits!(self.set_value, toggle_bits, 4, 2);
        self
    }
    #[allow(dead_code)]
    pub fn stattx_disabled(self) -> Self { self.stattx_variant(Stattx::Disabled) }
    #[allow(dead_code)]
    pub fn stattx_stall(self) -> Self { self.stattx_variant(Stattx::Stall) }
    #[allow(dead_code)]
    pub fn stattx_nak(self) -> Self { self.stattx_variant(Stattx::Nak) }
    #[allow(dead_code)]
    pub fn stattx_valid(self) -> Self { self.stattx_variant(Stattx::Valid) }

    // dtogtx = write 1 to toggle
    #[allow(dead_code)]
    pub fn dtogtx(mut self, value: bool) -> Self {
        let initial_bits = extract_bits!(self.initial_value, 6, 1);
        let toggle_bits = initial_bits ^ (if value { 1 } else { 0 });
        emplace_bits!(self.set_value, toggle_bits, 6, 1);
        self
    }

    // vttx = write 0 to reset
    #[allow(dead_code)]
    pub fn reset_vttx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b1, 7, 1);
        self
    }
    #[allow(dead_code)]
    pub fn dont_reset_vttx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b0, 7, 1);
        self
    }

    // epkind = standard write
    #[allow(dead_code)]
    pub fn ep_kind(mut self, new_ep_kind: bool) -> Self {
        emplace_bits!(self.set_value, if new_ep_kind { 1 } else { 0 }, 8, 1);
        self
    }

    // utype = standard write
    #[allow(dead_code)]
    pub fn utype_variant(mut self, value: Utype) -> Self {
        // standard write
        emplace_bits!(self.set_value, ((value as u8) as u32), 9, 2);
        self
    }
    #[allow(dead_code)]
    pub fn utype_bulk(self) -> Self { self.utype_variant(Utype::Bulk) }
    #[allow(dead_code)]
    pub fn utype_control(self) -> Self { self.utype_variant(Utype::Control) }
    #[allow(dead_code)]
    pub fn utype_iso(self) -> Self { self.utype_variant(Utype::Iso) }
    #[allow(dead_code)]
    pub fn utype_interrupt(self) -> Self { self.utype_variant(Utype::Interrupt) }

    // setup = read-only

    // statrx = write 1 to toggle
    #[allow(dead_code)]
    pub fn statrx_variant(mut self, value: Statrx) -> Self {
        let initial_bits = extract_bits!(self.initial_value, 12, 2);
        let toggle_bits = initial_bits ^ ((value as u8) as u32);
        emplace_bits!(self.set_value, toggle_bits, 12, 2);
        self
    }
    #[allow(dead_code)]
    pub fn statrx_disabled(self) -> Self { self.statrx_variant(Statrx::Disabled) }
    #[allow(dead_code)]
    pub fn statrx_stall(self) -> Self { self.statrx_variant(Statrx::Stall) }
    #[allow(dead_code)]
    pub fn statrx_nak(self) -> Self { self.statrx_variant(Statrx::Nak) }
    #[allow(dead_code)]
    pub fn statrx_valid(self) -> Self { self.statrx_variant(Statrx::Valid) }

    // dtogrx = write 1 to toggle
    #[allow(dead_code)]
    pub fn dtogrx(mut self, value: bool) -> Self {
        let initial_bits = extract_bits!(self.initial_value, 14, 1);
        let toggle_bits = initial_bits ^ (if value { 1 } else { 0 });
        emplace_bits!(self.set_value, toggle_bits, 14, 1);
        self
    }

    // vtrx = write 0 to reset
    #[allow(dead_code)]
    pub fn reset_vtrx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b1, 15, 1);
        self
    }
    #[allow(dead_code)]
    pub fn dont_reset_vtrx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b0, 15, 1);
        self
    }

    // devaddr = standard write
    #[allow(dead_code)]
    pub fn devaddr_value(mut self, value: u8) -> Self {
        emplace_bits!(self.set_value, ((value as u8) as u32), 16, 7);
        self
    }

    // nak = write 0 to reset
    #[allow(dead_code)]
    pub fn reset_nak(mut self) -> Self {
        emplace_bits!(self.set_value, 0b1, 23, 1);
        self
    }
    #[allow(dead_code)]
    pub fn dont_reset_nak(mut self) -> Self {
        emplace_bits!(self.set_value, 0b0, 23, 1);
        self
    }

    // ls_ep = standard write
    #[allow(dead_code)]
    pub fn ls_ep(mut self, new_ls_ep: bool) -> Self {
        // standard write
        emplace_bits!(self.set_value, if new_ls_ep { 1 } else { 0 }, 24, 1);
        self
    }

    // nak = write 0 to reset
    #[allow(dead_code)]
    pub fn reset_err_tx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b1, 25, 1);
        self
    }
    #[allow(dead_code)]
    pub fn dont_reset_err_tx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b0, 25, 1);
        self
    }

    // nak = write 0 to reset
    #[allow(dead_code)]
    pub fn reset_err_rx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b1, 26, 1);
        self
    }
    #[allow(dead_code)]
    pub fn dont_reset_err_rx(mut self) -> Self {
        emplace_bits!(self.set_value, 0b0, 26, 1);
        self
    }

    fn perform(self) {
        self.chepnr.write(|w| unsafe { w.bits(self.set_value) });
    }
}

pub(crate) fn modify_chepnr<'a, F: FnOnce(ChepnrModifier<'a>) -> ChepnrModifier<'a>>(chepnr: &'a stm32g0b0::generic::Reg<stm32g0b0::usb::chepnr::ChepnrSpec>, modifier: F) {
    let chepnr_modifier = ChepnrModifier::new(chepnr);
    modifier(chepnr_modifier)
        .perform()
}
