#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    cr4: Cr4,
    sr1: Sr1,
    sr2: Sr2,
    scr: Scr,
    _reserved7: [u8; 0x04],
    pucra: Pucra,
    pdcra: Pdcra,
    pucrb: Pucrb,
    pdcrb: Pdcrb,
    pucrc: Pucrc,
    pdcrc: Pdcrc,
    pucrd: Pucrd,
    pdcrd: Pdcrd,
    pucre: Pucre,
    pdcre: Pdcre,
    pucrf: Pucrf,
    pdcrf: Pdcrf,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Power control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Power control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x0c - Power control register 4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &Cr4 {
        &self.cr4
    }
    #[doc = "0x10 - Power status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x14 - Power status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &Sr2 {
        &self.sr2
    }
    #[doc = "0x18 - Power status clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x20 - Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn pucra(&self) -> &Pucra {
        &self.pucra
    }
    #[doc = "0x24 - Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn pdcra(&self) -> &Pdcra {
        &self.pdcra
    }
    #[doc = "0x28 - Power Port B pull-up control register"]
    #[inline(always)]
    pub const fn pucrb(&self) -> &Pucrb {
        &self.pucrb
    }
    #[doc = "0x2c - Power Port B pull-down control register"]
    #[inline(always)]
    pub const fn pdcrb(&self) -> &Pdcrb {
        &self.pdcrb
    }
    #[doc = "0x30 - Power Port C pull-up control register"]
    #[inline(always)]
    pub const fn pucrc(&self) -> &Pucrc {
        &self.pucrc
    }
    #[doc = "0x34 - Power Port C pull-down control register"]
    #[inline(always)]
    pub const fn pdcrc(&self) -> &Pdcrc {
        &self.pdcrc
    }
    #[doc = "0x38 - Power Port D pull-up control register"]
    #[inline(always)]
    pub const fn pucrd(&self) -> &Pucrd {
        &self.pucrd
    }
    #[doc = "0x3c - Power Port D pull-down control register"]
    #[inline(always)]
    pub const fn pdcrd(&self) -> &Pdcrd {
        &self.pdcrd
    }
    #[doc = "0x40 - Power Port E pull-UP control register"]
    #[inline(always)]
    pub const fn pucre(&self) -> &Pucre {
        &self.pucre
    }
    #[doc = "0x44 - Power Port E pull-down control register"]
    #[inline(always)]
    pub const fn pdcre(&self) -> &Pdcre {
        &self.pdcre
    }
    #[doc = "0x48 - Power Port F pull-up control register"]
    #[inline(always)]
    pub const fn pucrf(&self) -> &Pucrf {
        &self.pucrf
    }
    #[doc = "0x4c - Power Port F pull-down control register"]
    #[inline(always)]
    pub const fn pdcrf(&self) -> &Pdcrf {
        &self.pdcrf
    }
}
#[doc = "CR1 (rw) register accessor: Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Power control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Power control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "Power control register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: Power control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`]
module"]
#[doc(alias = "CR4")]
pub type Cr4 = crate::Reg<cr4::Cr4Spec>;
#[doc = "Power control register 4"]
pub mod cr4;
#[doc = "SR1 (r) register accessor: Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "Power status register 1"]
pub mod sr1;
#[doc = "SR2 (r) register accessor: Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
#[doc(alias = "SR2")]
pub type Sr2 = crate::Reg<sr2::Sr2Spec>;
#[doc = "Power status register 2"]
pub mod sr2;
#[doc = "SCR (w) register accessor: Power status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Power status clear register"]
pub mod scr;
#[doc = "PUCRA (rw) register accessor: Power Port A pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucra`]
module"]
#[doc(alias = "PUCRA")]
pub type Pucra = crate::Reg<pucra::PucraSpec>;
#[doc = "Power Port A pull-up control register"]
pub mod pucra;
#[doc = "PDCRA (rw) register accessor: Power Port A pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcra`]
module"]
#[doc(alias = "PDCRA")]
pub type Pdcra = crate::Reg<pdcra::PdcraSpec>;
#[doc = "Power Port A pull-down control register"]
pub mod pdcra;
#[doc = "PUCRB (rw) register accessor: Power Port B pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrb`]
module"]
#[doc(alias = "PUCRB")]
pub type Pucrb = crate::Reg<pucrb::PucrbSpec>;
#[doc = "Power Port B pull-up control register"]
pub mod pucrb;
#[doc = "PDCRB (rw) register accessor: Power Port B pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrb`]
module"]
#[doc(alias = "PDCRB")]
pub type Pdcrb = crate::Reg<pdcrb::PdcrbSpec>;
#[doc = "Power Port B pull-down control register"]
pub mod pdcrb;
#[doc = "PUCRC (rw) register accessor: Power Port C pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrc`]
module"]
#[doc(alias = "PUCRC")]
pub type Pucrc = crate::Reg<pucrc::PucrcSpec>;
#[doc = "Power Port C pull-up control register"]
pub mod pucrc;
#[doc = "PDCRC (rw) register accessor: Power Port C pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrc`]
module"]
#[doc(alias = "PDCRC")]
pub type Pdcrc = crate::Reg<pdcrc::PdcrcSpec>;
#[doc = "Power Port C pull-down control register"]
pub mod pdcrc;
#[doc = "PUCRD (rw) register accessor: Power Port D pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrd`]
module"]
#[doc(alias = "PUCRD")]
pub type Pucrd = crate::Reg<pucrd::PucrdSpec>;
#[doc = "Power Port D pull-up control register"]
pub mod pucrd;
#[doc = "PDCRD (rw) register accessor: Power Port D pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrd`]
module"]
#[doc(alias = "PDCRD")]
pub type Pdcrd = crate::Reg<pdcrd::PdcrdSpec>;
#[doc = "Power Port D pull-down control register"]
pub mod pdcrd;
#[doc = "PUCRE (rw) register accessor: Power Port E pull-UP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucre`]
module"]
#[doc(alias = "PUCRE")]
pub type Pucre = crate::Reg<pucre::PucreSpec>;
#[doc = "Power Port E pull-UP control register"]
pub mod pucre;
#[doc = "PDCRE (rw) register accessor: Power Port E pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcre`]
module"]
#[doc(alias = "PDCRE")]
pub type Pdcre = crate::Reg<pdcre::PdcreSpec>;
#[doc = "Power Port E pull-down control register"]
pub mod pdcre;
#[doc = "PUCRF (rw) register accessor: Power Port F pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrf`]
module"]
#[doc(alias = "PUCRF")]
pub type Pucrf = crate::Reg<pucrf::PucrfSpec>;
#[doc = "Power Port F pull-up control register"]
pub mod pucrf;
#[doc = "PDCRF (rw) register accessor: Power Port F pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrf`]
module"]
#[doc(alias = "PDCRF")]
pub type Pdcrf = crate::Reg<pdcrf::PdcrfSpec>;
#[doc = "Power Port F pull-down control register"]
pub mod pdcrf;
