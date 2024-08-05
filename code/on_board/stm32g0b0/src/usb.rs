#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chep0r: Chep0r,
    chep1r: Chep1r,
    chep2r: Chep2r,
    chep3r: Chep3r,
    chep4r: Chep4r,
    chep5r: Chep5r,
    chep6r: Chep6r,
    chep7r: Chep7r,
    _reserved8: [u8; 0x20],
    cntr: Cntr,
    istr: Istr,
    fnr: Fnr,
    daddr: Daddr,
    _reserved12: [u8; 0x04],
    lpmcsr: Lpmcsr,
    bcdr: Bcdr,
}
impl RegisterBlock {
    #[doc = "0x00 - USB endpoint/channel 0 register"]
    #[inline(always)]
    pub const fn chep0r(&self) -> &Chep0r {
        &self.chep0r
    }
    #[doc = "0x04 - USB endpoint/channel 1 register"]
    #[inline(always)]
    pub const fn chep1r(&self) -> &Chep1r {
        &self.chep1r
    }
    #[doc = "0x08 - USB endpoint/channel 2 register"]
    #[inline(always)]
    pub const fn chep2r(&self) -> &Chep2r {
        &self.chep2r
    }
    #[doc = "0x0c - USB endpoint/channel 3 register"]
    #[inline(always)]
    pub const fn chep3r(&self) -> &Chep3r {
        &self.chep3r
    }
    #[doc = "0x10 - USB endpoint/channel 4 register"]
    #[inline(always)]
    pub const fn chep4r(&self) -> &Chep4r {
        &self.chep4r
    }
    #[doc = "0x14 - USB endpoint/channel 5 register"]
    #[inline(always)]
    pub const fn chep5r(&self) -> &Chep5r {
        &self.chep5r
    }
    #[doc = "0x18 - USB endpoint/channel 6 register"]
    #[inline(always)]
    pub const fn chep6r(&self) -> &Chep6r {
        &self.chep6r
    }
    #[doc = "0x1c - USB endpoint/channel 7 register"]
    #[inline(always)]
    pub const fn chep7r(&self) -> &Chep7r {
        &self.chep7r
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x44 - USB interrupt status register"]
    #[inline(always)]
    pub const fn istr(&self) -> &Istr {
        &self.istr
    }
    #[doc = "0x48 - USB frame number register"]
    #[inline(always)]
    pub const fn fnr(&self) -> &Fnr {
        &self.fnr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn daddr(&self) -> &Daddr {
        &self.daddr
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &Lpmcsr {
        &self.lpmcsr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn bcdr(&self) -> &Bcdr {
        &self.bcdr
    }
}
#[doc = "CHEP0R (rw) register accessor: USB endpoint/channel 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep0r`]
module"]
#[doc(alias = "CHEP0R")]
pub type Chep0r = crate::Reg<chep0r::Chep0rSpec>;
#[doc = "USB endpoint/channel 0 register"]
pub mod chep0r;
#[doc = "CHEP1R (rw) register accessor: USB endpoint/channel 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep1r`]
module"]
#[doc(alias = "CHEP1R")]
pub type Chep1r = crate::Reg<chep1r::Chep1rSpec>;
#[doc = "USB endpoint/channel 1 register"]
pub mod chep1r;
#[doc = "CHEP2R (rw) register accessor: USB endpoint/channel 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep2r`]
module"]
#[doc(alias = "CHEP2R")]
pub type Chep2r = crate::Reg<chep2r::Chep2rSpec>;
#[doc = "USB endpoint/channel 2 register"]
pub mod chep2r;
#[doc = "CHEP3R (rw) register accessor: USB endpoint/channel 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep3r`]
module"]
#[doc(alias = "CHEP3R")]
pub type Chep3r = crate::Reg<chep3r::Chep3rSpec>;
#[doc = "USB endpoint/channel 3 register"]
pub mod chep3r;
#[doc = "CHEP4R (rw) register accessor: USB endpoint/channel 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep4r`]
module"]
#[doc(alias = "CHEP4R")]
pub type Chep4r = crate::Reg<chep4r::Chep4rSpec>;
#[doc = "USB endpoint/channel 4 register"]
pub mod chep4r;
#[doc = "CHEP5R (rw) register accessor: USB endpoint/channel 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep5r`]
module"]
#[doc(alias = "CHEP5R")]
pub type Chep5r = crate::Reg<chep5r::Chep5rSpec>;
#[doc = "USB endpoint/channel 5 register"]
pub mod chep5r;
#[doc = "CHEP6R (rw) register accessor: USB endpoint/channel 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep6r`]
module"]
#[doc(alias = "CHEP6R")]
pub type Chep6r = crate::Reg<chep6r::Chep6rSpec>;
#[doc = "USB endpoint/channel 6 register"]
pub mod chep6r;
#[doc = "CHEP7R (rw) register accessor: USB endpoint/channel 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep7r`]
module"]
#[doc(alias = "CHEP7R")]
pub type Chep7r = crate::Reg<chep7r::Chep7rSpec>;
#[doc = "USB endpoint/channel 7 register"]
pub mod chep7r;
#[doc = "CNTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = ""]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`]
module"]
#[doc(alias = "ISTR")]
pub type Istr = crate::Reg<istr::IstrSpec>;
#[doc = "USB interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: USB frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`]
module"]
#[doc(alias = "FNR")]
pub type Fnr = crate::Reg<fnr::FnrSpec>;
#[doc = "USB frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
#[doc(alias = "DADDR")]
pub type Daddr = crate::Reg<daddr::DaddrSpec>;
#[doc = ""]
pub mod daddr;
#[doc = "LPMCSR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcsr`]
module"]
#[doc(alias = "LPMCSR")]
pub type Lpmcsr = crate::Reg<lpmcsr::LpmcsrSpec>;
#[doc = ""]
pub mod lpmcsr;
#[doc = "BCDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdr`]
module"]
#[doc(alias = "BCDR")]
pub type Bcdr = crate::Reg<bcdr::BcdrSpec>;
#[doc = ""]
pub mod bcdr;
