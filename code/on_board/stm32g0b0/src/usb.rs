#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chepr: (),
    _reserved1: [u8; 0x40],
    cntr: Cntr,
    istr: Istr,
    fnr: Fnr,
    daddr: Daddr,
    _reserved5: [u8; 0x04],
    lpmcsr: Lpmcsr,
    bcdr: Bcdr,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - USB endpoint/channel register"]
    #[inline(always)]
    pub const fn chepr(&self, n: usize) -> &Chepr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(32 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - USB endpoint/channel register"]
    #[inline(always)]
    pub fn chepr_iter(&self) -> impl Iterator<Item = &Chepr> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(32 * n).cast() })
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
#[doc = "CHEPR (rw) register accessor: USB endpoint/channel register\n\nYou can [`read`](crate::Reg::read) this register and get [`chepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chepr`]
module"]
#[doc(alias = "CHEPR")]
pub type Chepr = crate::Reg<chepr::CheprSpec>;
#[doc = "USB endpoint/channel register"]
pub mod chepr;
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
