#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    _reserved1: [u8; 0x04],
    keyr: Keyr,
    optkeyr: Optkeyr,
    sr: Sr,
    cr: Cr,
    eccr: Eccr,
    _reserved6: [u8; 0x04],
    optr: Optr,
    _reserved7: [u8; 0x08],
    wrp1ar: Wrp1ar,
    wrp1br: Wrp1br,
    _reserved9: [u8; 0x18],
    wrp2ar: Wrp2ar,
    wrp2br: Wrp2br,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x08 - Flash key register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &Keyr {
        &self.keyr
    }
    #[doc = "0x0c - Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &Optkeyr {
        &self.optkeyr
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x18 - Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(&self) -> &Eccr {
        &self.eccr
    }
    #[doc = "0x20 - Flash option register"]
    #[inline(always)]
    pub const fn optr(&self) -> &Optr {
        &self.optr
    }
    #[doc = "0x2c - Flash WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &Wrp1ar {
        &self.wrp1ar
    }
    #[doc = "0x30 - Flash WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(&self) -> &Wrp1br {
        &self.wrp1br
    }
    #[doc = "0x4c - FLASH WRP2 area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &Wrp2ar {
        &self.wrp2ar
    }
    #[doc = "0x50 - FLASH WRP2 area B address register"]
    #[inline(always)]
    pub const fn wrp2br(&self) -> &Wrp2br {
        &self.wrp2br
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
#[doc(alias = "KEYR")]
pub type Keyr = crate::Reg<keyr::KeyrSpec>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
#[doc(alias = "OPTKEYR")]
pub type Optkeyr = crate::Reg<optkeyr::OptkeyrSpec>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "ECCR (rw) register accessor: Flash ECC register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`]
module"]
#[doc(alias = "ECCR")]
pub type Eccr = crate::Reg<eccr::EccrSpec>;
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "OPTR (rw) register accessor: Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`]
module"]
#[doc(alias = "OPTR")]
pub type Optr = crate::Reg<optr::OptrSpec>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "WRP1AR (r) register accessor: Flash WRP area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1ar`]
module"]
#[doc(alias = "WRP1AR")]
pub type Wrp1ar = crate::Reg<wrp1ar::Wrp1arSpec>;
#[doc = "Flash WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (r) register accessor: Flash WRP area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp1br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1br`]
module"]
#[doc(alias = "WRP1BR")]
pub type Wrp1br = crate::Reg<wrp1br::Wrp1brSpec>;
#[doc = "Flash WRP area B address register"]
pub mod wrp1br;
#[doc = "WRP2AR (rw) register accessor: FLASH WRP2 area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2ar`]
module"]
#[doc(alias = "WRP2AR")]
pub type Wrp2ar = crate::Reg<wrp2ar::Wrp2arSpec>;
#[doc = "FLASH WRP2 area A address register"]
pub mod wrp2ar;
#[doc = "WRP2BR (rw) register accessor: FLASH WRP2 area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2br`]
module"]
#[doc(alias = "WRP2BR")]
pub type Wrp2br = crate::Reg<wrp2br::Wrp2brSpec>;
#[doc = "FLASH WRP2 area B address register"]
pub mod wrp2br;
