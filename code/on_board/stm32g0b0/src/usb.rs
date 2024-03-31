#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usb_chep0r: UsbChep0r,
    usb_chep1r: UsbChep1r,
    usb_chep2r: UsbChep2r,
    usb_chep3r: UsbChep3r,
    usb_chep4r: UsbChep4r,
    usb_chep5r: UsbChep5r,
    usb_chep6r: UsbChep6r,
    usb_chep7r: UsbChep7r,
    _reserved8: [u8; 0x20],
    usb_cntr: UsbCntr,
    usb_istr: UsbIstr,
    usb_fnr: UsbFnr,
    usb_daddr: UsbDaddr,
    _reserved12: [u8; 0x04],
    usb_lpmcsr: UsbLpmcsr,
    usb_bcdr: UsbBcdr,
}
impl RegisterBlock {
    #[doc = "0x00 - USB endpoint/channel 0 register"]
    #[inline(always)]
    pub const fn usb_chep0r(&self) -> &UsbChep0r {
        &self.usb_chep0r
    }
    #[doc = "0x04 - USB endpoint/channel 1 register"]
    #[inline(always)]
    pub const fn usb_chep1r(&self) -> &UsbChep1r {
        &self.usb_chep1r
    }
    #[doc = "0x08 - USB endpoint/channel 2 register"]
    #[inline(always)]
    pub const fn usb_chep2r(&self) -> &UsbChep2r {
        &self.usb_chep2r
    }
    #[doc = "0x0c - USB endpoint/channel 3 register"]
    #[inline(always)]
    pub const fn usb_chep3r(&self) -> &UsbChep3r {
        &self.usb_chep3r
    }
    #[doc = "0x10 - USB endpoint/channel 4 register"]
    #[inline(always)]
    pub const fn usb_chep4r(&self) -> &UsbChep4r {
        &self.usb_chep4r
    }
    #[doc = "0x14 - USB endpoint/channel 5 register"]
    #[inline(always)]
    pub const fn usb_chep5r(&self) -> &UsbChep5r {
        &self.usb_chep5r
    }
    #[doc = "0x18 - USB endpoint/channel 6 register"]
    #[inline(always)]
    pub const fn usb_chep6r(&self) -> &UsbChep6r {
        &self.usb_chep6r
    }
    #[doc = "0x1c - USB endpoint/channel 7 register"]
    #[inline(always)]
    pub const fn usb_chep7r(&self) -> &UsbChep7r {
        &self.usb_chep7r
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn usb_cntr(&self) -> &UsbCntr {
        &self.usb_cntr
    }
    #[doc = "0x44 - USB interrupt status register"]
    #[inline(always)]
    pub const fn usb_istr(&self) -> &UsbIstr {
        &self.usb_istr
    }
    #[doc = "0x48 - USB frame number register"]
    #[inline(always)]
    pub const fn usb_fnr(&self) -> &UsbFnr {
        &self.usb_fnr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn usb_daddr(&self) -> &UsbDaddr {
        &self.usb_daddr
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn usb_lpmcsr(&self) -> &UsbLpmcsr {
        &self.usb_lpmcsr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn usb_bcdr(&self) -> &UsbBcdr {
        &self.usb_bcdr
    }
}
#[doc = "USB_CHEP0R (rw) register accessor: USB endpoint/channel 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep0r`]
module"]
#[doc(alias = "USB_CHEP0R")]
pub type UsbChep0r = crate::Reg<usb_chep0r::UsbChep0rSpec>;
#[doc = "USB endpoint/channel 0 register"]
pub mod usb_chep0r;
#[doc = "USB_CHEP1R (rw) register accessor: USB endpoint/channel 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep1r`]
module"]
#[doc(alias = "USB_CHEP1R")]
pub type UsbChep1r = crate::Reg<usb_chep1r::UsbChep1rSpec>;
#[doc = "USB endpoint/channel 1 register"]
pub mod usb_chep1r;
#[doc = "USB_CHEP2R (rw) register accessor: USB endpoint/channel 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep2r`]
module"]
#[doc(alias = "USB_CHEP2R")]
pub type UsbChep2r = crate::Reg<usb_chep2r::UsbChep2rSpec>;
#[doc = "USB endpoint/channel 2 register"]
pub mod usb_chep2r;
#[doc = "USB_CHEP3R (rw) register accessor: USB endpoint/channel 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep3r`]
module"]
#[doc(alias = "USB_CHEP3R")]
pub type UsbChep3r = crate::Reg<usb_chep3r::UsbChep3rSpec>;
#[doc = "USB endpoint/channel 3 register"]
pub mod usb_chep3r;
#[doc = "USB_CHEP4R (rw) register accessor: USB endpoint/channel 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep4r`]
module"]
#[doc(alias = "USB_CHEP4R")]
pub type UsbChep4r = crate::Reg<usb_chep4r::UsbChep4rSpec>;
#[doc = "USB endpoint/channel 4 register"]
pub mod usb_chep4r;
#[doc = "USB_CHEP5R (rw) register accessor: USB endpoint/channel 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep5r`]
module"]
#[doc(alias = "USB_CHEP5R")]
pub type UsbChep5r = crate::Reg<usb_chep5r::UsbChep5rSpec>;
#[doc = "USB endpoint/channel 5 register"]
pub mod usb_chep5r;
#[doc = "USB_CHEP6R (rw) register accessor: USB endpoint/channel 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep6r`]
module"]
#[doc(alias = "USB_CHEP6R")]
pub type UsbChep6r = crate::Reg<usb_chep6r::UsbChep6rSpec>;
#[doc = "USB endpoint/channel 6 register"]
pub mod usb_chep6r;
#[doc = "USB_CHEP7R (rw) register accessor: USB endpoint/channel 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep7r`]
module"]
#[doc(alias = "USB_CHEP7R")]
pub type UsbChep7r = crate::Reg<usb_chep7r::UsbChep7rSpec>;
#[doc = "USB endpoint/channel 7 register"]
pub mod usb_chep7r;
#[doc = "USB_CNTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_cntr`]
module"]
#[doc(alias = "USB_CNTR")]
pub type UsbCntr = crate::Reg<usb_cntr::UsbCntrSpec>;
#[doc = ""]
pub mod usb_cntr;
#[doc = "USB_ISTR (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_istr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_istr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_istr`]
module"]
#[doc(alias = "USB_ISTR")]
pub type UsbIstr = crate::Reg<usb_istr::UsbIstrSpec>;
#[doc = "USB interrupt status register"]
pub mod usb_istr;
#[doc = "USB_FNR (r) register accessor: USB frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_fnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_fnr`]
module"]
#[doc(alias = "USB_FNR")]
pub type UsbFnr = crate::Reg<usb_fnr::UsbFnrSpec>;
#[doc = "USB frame number register"]
pub mod usb_fnr;
#[doc = "USB_DADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_daddr`]
module"]
#[doc(alias = "USB_DADDR")]
pub type UsbDaddr = crate::Reg<usb_daddr::UsbDaddrSpec>;
#[doc = ""]
pub mod usb_daddr;
#[doc = "USB_LPMCSR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_lpmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_lpmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_lpmcsr`]
module"]
#[doc(alias = "USB_LPMCSR")]
pub type UsbLpmcsr = crate::Reg<usb_lpmcsr::UsbLpmcsrSpec>;
#[doc = ""]
pub mod usb_lpmcsr;
#[doc = "USB_BCDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_bcdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_bcdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_bcdr`]
module"]
#[doc(alias = "USB_BCDR")]
pub type UsbBcdr = crate::Reg<usb_bcdr::UsbBcdrSpec>;
#[doc = ""]
pub mod usb_bcdr;
