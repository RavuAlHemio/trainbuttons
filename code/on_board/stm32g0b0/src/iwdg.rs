#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iwdg_kr: IwdgKr,
    iwdg_pr: IwdgPr,
    iwdg_rlr: IwdgRlr,
    iwdg_sr: IwdgSr,
    iwdg_winr: IwdgWinr,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register"]
    #[inline(always)]
    pub const fn iwdg_kr(&self) -> &IwdgKr {
        &self.iwdg_kr
    }
    #[doc = "0x04 - Prescaler register"]
    #[inline(always)]
    pub const fn iwdg_pr(&self) -> &IwdgPr {
        &self.iwdg_pr
    }
    #[doc = "0x08 - Reload register"]
    #[inline(always)]
    pub const fn iwdg_rlr(&self) -> &IwdgRlr {
        &self.iwdg_rlr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn iwdg_sr(&self) -> &IwdgSr {
        &self.iwdg_sr
    }
    #[doc = "0x10 - Window register"]
    #[inline(always)]
    pub const fn iwdg_winr(&self) -> &IwdgWinr {
        &self.iwdg_winr
    }
}
#[doc = "IWDG_KR (w) register accessor: Key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_kr`]
module"]
#[doc(alias = "IWDG_KR")]
pub type IwdgKr = crate::Reg<iwdg_kr::IwdgKrSpec>;
#[doc = "Key register"]
pub mod iwdg_kr;
#[doc = "IWDG_PR (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_pr`]
module"]
#[doc(alias = "IWDG_PR")]
pub type IwdgPr = crate::Reg<iwdg_pr::IwdgPrSpec>;
#[doc = "Prescaler register"]
pub mod iwdg_pr;
#[doc = "IWDG_RLR (rw) register accessor: Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_rlr`]
module"]
#[doc(alias = "IWDG_RLR")]
pub type IwdgRlr = crate::Reg<iwdg_rlr::IwdgRlrSpec>;
#[doc = "Reload register"]
pub mod iwdg_rlr;
#[doc = "IWDG_SR (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_sr`]
module"]
#[doc(alias = "IWDG_SR")]
pub type IwdgSr = crate::Reg<iwdg_sr::IwdgSrSpec>;
#[doc = "Status register"]
pub mod iwdg_sr;
#[doc = "IWDG_WINR (rw) register accessor: Window register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_winr`]
module"]
#[doc(alias = "IWDG_WINR")]
pub type IwdgWinr = crate::Reg<iwdg_winr::IwdgWinrSpec>;
#[doc = "Window register"]
pub mod iwdg_winr;
