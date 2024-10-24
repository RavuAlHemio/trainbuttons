#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wwdg_cr: WwdgCr,
    wwdg_cfr: WwdgCfr,
    wwdg_sr: WwdgSr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn wwdg_cr(&self) -> &WwdgCr {
        &self.wwdg_cr
    }
    #[doc = "0x04 - Configuration register"]
    #[inline(always)]
    pub const fn wwdg_cfr(&self) -> &WwdgCfr {
        &self.wwdg_cfr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn wwdg_sr(&self) -> &WwdgSr {
        &self.wwdg_sr
    }
}
#[doc = "WWDG_CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdg_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_cr`]
module"]
#[doc(alias = "WWDG_CR")]
pub type WwdgCr = crate::Reg<wwdg_cr::WwdgCrSpec>;
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "WWDG_CFR (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdg_cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_cfr`]
module"]
#[doc(alias = "WWDG_CFR")]
pub type WwdgCfr = crate::Reg<wwdg_cfr::WwdgCfrSpec>;
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "WWDG_SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdg_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_sr`]
module"]
#[doc(alias = "WWDG_SR")]
pub type WwdgSr = crate::Reg<wwdg_sr::WwdgSrSpec>;
#[doc = "Status register"]
pub mod wwdg_sr;
