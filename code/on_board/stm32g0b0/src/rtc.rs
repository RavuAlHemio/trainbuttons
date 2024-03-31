#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtc_tr: RtcTr,
    rtc_dr: RtcDr,
    rtc_ssr: RtcSsr,
    rtc_icsr: RtcIcsr,
    rtc_prer: RtcPrer,
    rtc_wutr: RtcWutr,
    rtc_cr: RtcCr,
    _reserved7: [u8; 0x08],
    rtc_wpr: RtcWpr,
    rtc_calr: RtcCalr,
    rtc_shiftr: RtcShiftr,
    rtc_tstr: RtcTstr,
    rtc_tsdr: RtcTsdr,
    rtc_tsssr: RtcTsssr,
    _reserved13: [u8; 0x04],
    rtc_alrmar: RtcAlrmar,
    rtc_alrmassr: RtcAlrmassr,
    rtc_alrmbr: RtcAlrmbr,
    rtc_alrmbssr: RtcAlrmbssr,
    rtc_sr: RtcSr,
    rtc_misr: RtcMisr,
    _reserved19: [u8; 0x04],
    rtc_scr: RtcScr,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RtcTr {
        &self.rtc_tr
    }
    #[doc = "0x04 - RTC date register"]
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RtcDr {
        &self.rtc_dr
    }
    #[doc = "0x08 - RTC sub second register"]
    #[inline(always)]
    pub const fn rtc_ssr(&self) -> &RtcSsr {
        &self.rtc_ssr
    }
    #[doc = "0x0c - RTC initialization control and status register"]
    #[inline(always)]
    pub const fn rtc_icsr(&self) -> &RtcIcsr {
        &self.rtc_icsr
    }
    #[doc = "0x10 - RTC prescaler register"]
    #[inline(always)]
    pub const fn rtc_prer(&self) -> &RtcPrer {
        &self.rtc_prer
    }
    #[doc = "0x14 - RTC wakeup timer register"]
    #[inline(always)]
    pub const fn rtc_wutr(&self) -> &RtcWutr {
        &self.rtc_wutr
    }
    #[doc = "0x18 - control register"]
    #[inline(always)]
    pub const fn rtc_cr(&self) -> &RtcCr {
        &self.rtc_cr
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn rtc_wpr(&self) -> &RtcWpr {
        &self.rtc_wpr
    }
    #[doc = "0x28 - RTC calibration register"]
    #[inline(always)]
    pub const fn rtc_calr(&self) -> &RtcCalr {
        &self.rtc_calr
    }
    #[doc = "0x2c - RTC shift control register"]
    #[inline(always)]
    pub const fn rtc_shiftr(&self) -> &RtcShiftr {
        &self.rtc_shiftr
    }
    #[doc = "0x30 - RTC timestamp time register"]
    #[inline(always)]
    pub const fn rtc_tstr(&self) -> &RtcTstr {
        &self.rtc_tstr
    }
    #[doc = "0x34 - RTC timestamp date register"]
    #[inline(always)]
    pub const fn rtc_tsdr(&self) -> &RtcTsdr {
        &self.rtc_tsdr
    }
    #[doc = "0x38 - RTC timestamp sub second register"]
    #[inline(always)]
    pub const fn rtc_tsssr(&self) -> &RtcTsssr {
        &self.rtc_tsssr
    }
    #[doc = "0x40 - RTC alarm A register"]
    #[inline(always)]
    pub const fn rtc_alrmar(&self) -> &RtcAlrmar {
        &self.rtc_alrmar
    }
    #[doc = "0x44 - RTC alarm A sub second register"]
    #[inline(always)]
    pub const fn rtc_alrmassr(&self) -> &RtcAlrmassr {
        &self.rtc_alrmassr
    }
    #[doc = "0x48 - RTC alarm B register"]
    #[inline(always)]
    pub const fn rtc_alrmbr(&self) -> &RtcAlrmbr {
        &self.rtc_alrmbr
    }
    #[doc = "0x4c - RTC alarm B sub second register"]
    #[inline(always)]
    pub const fn rtc_alrmbssr(&self) -> &RtcAlrmbssr {
        &self.rtc_alrmbssr
    }
    #[doc = "0x50 - RTC status register"]
    #[inline(always)]
    pub const fn rtc_sr(&self) -> &RtcSr {
        &self.rtc_sr
    }
    #[doc = "0x54 - RTC masked interrupt status register"]
    #[inline(always)]
    pub const fn rtc_misr(&self) -> &RtcMisr {
        &self.rtc_misr
    }
    #[doc = "0x5c - RTC status clear register"]
    #[inline(always)]
    pub const fn rtc_scr(&self) -> &RtcScr {
        &self.rtc_scr
    }
}
#[doc = "RTC_TR (rw) register accessor: RTC time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tr`]
module"]
#[doc(alias = "RTC_TR")]
pub type RtcTr = crate::Reg<rtc_tr::RtcTrSpec>;
#[doc = "RTC time register"]
pub mod rtc_tr;
#[doc = "RTC_DR (rw) register accessor: RTC date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_dr`]
module"]
#[doc(alias = "RTC_DR")]
pub type RtcDr = crate::Reg<rtc_dr::RtcDrSpec>;
#[doc = "RTC date register"]
pub mod rtc_dr;
#[doc = "RTC_SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ssr`]
module"]
#[doc(alias = "RTC_SSR")]
pub type RtcSsr = crate::Reg<rtc_ssr::RtcSsrSpec>;
#[doc = "RTC sub second register"]
pub mod rtc_ssr;
#[doc = "RTC_ICSR (rw) register accessor: RTC initialization control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_icsr`]
module"]
#[doc(alias = "RTC_ICSR")]
pub type RtcIcsr = crate::Reg<rtc_icsr::RtcIcsrSpec>;
#[doc = "RTC initialization control and status register"]
pub mod rtc_icsr;
#[doc = "RTC_PRER (rw) register accessor: RTC prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_prer`]
module"]
#[doc(alias = "RTC_PRER")]
pub type RtcPrer = crate::Reg<rtc_prer::RtcPrerSpec>;
#[doc = "RTC prescaler register"]
pub mod rtc_prer;
#[doc = "RTC_WUTR (rw) register accessor: RTC wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wutr`]
module"]
#[doc(alias = "RTC_WUTR")]
pub type RtcWutr = crate::Reg<rtc_wutr::RtcWutrSpec>;
#[doc = "RTC wakeup timer register"]
pub mod rtc_wutr;
#[doc = "RTC_CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cr`]
module"]
#[doc(alias = "RTC_CR")]
pub type RtcCr = crate::Reg<rtc_cr::RtcCrSpec>;
#[doc = "control register"]
pub mod rtc_cr;
#[doc = "RTC_WPR (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wpr`]
module"]
#[doc(alias = "RTC_WPR")]
pub type RtcWpr = crate::Reg<rtc_wpr::RtcWprSpec>;
#[doc = "write protection register"]
pub mod rtc_wpr;
#[doc = "RTC_CALR (rw) register accessor: RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_calr`]
module"]
#[doc(alias = "RTC_CALR")]
pub type RtcCalr = crate::Reg<rtc_calr::RtcCalrSpec>;
#[doc = "RTC calibration register"]
pub mod rtc_calr;
#[doc = "RTC_SHIFTR (w) register accessor: RTC shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_shiftr`]
module"]
#[doc(alias = "RTC_SHIFTR")]
pub type RtcShiftr = crate::Reg<rtc_shiftr::RtcShiftrSpec>;
#[doc = "RTC shift control register"]
pub mod rtc_shiftr;
#[doc = "RTC_TSTR (r) register accessor: RTC timestamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tstr`]
module"]
#[doc(alias = "RTC_TSTR")]
pub type RtcTstr = crate::Reg<rtc_tstr::RtcTstrSpec>;
#[doc = "RTC timestamp time register"]
pub mod rtc_tstr;
#[doc = "RTC_TSDR (r) register accessor: RTC timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsdr`]
module"]
#[doc(alias = "RTC_TSDR")]
pub type RtcTsdr = crate::Reg<rtc_tsdr::RtcTsdrSpec>;
#[doc = "RTC timestamp date register"]
pub mod rtc_tsdr;
#[doc = "RTC_TSSSR (r) register accessor: RTC timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsssr`]
module"]
#[doc(alias = "RTC_TSSSR")]
pub type RtcTsssr = crate::Reg<rtc_tsssr::RtcTsssrSpec>;
#[doc = "RTC timestamp sub second register"]
pub mod rtc_tsssr;
#[doc = "RTC_ALRMAR (rw) register accessor: RTC alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmar`]
module"]
#[doc(alias = "RTC_ALRMAR")]
pub type RtcAlrmar = crate::Reg<rtc_alrmar::RtcAlrmarSpec>;
#[doc = "RTC alarm A register"]
pub mod rtc_alrmar;
#[doc = "RTC_ALRMASSR (rw) register accessor: RTC alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmassr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmassr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmassr`]
module"]
#[doc(alias = "RTC_ALRMASSR")]
pub type RtcAlrmassr = crate::Reg<rtc_alrmassr::RtcAlrmassrSpec>;
#[doc = "RTC alarm A sub second register"]
pub mod rtc_alrmassr;
#[doc = "RTC_ALRMBR (rw) register accessor: RTC alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmbr`]
module"]
#[doc(alias = "RTC_ALRMBR")]
pub type RtcAlrmbr = crate::Reg<rtc_alrmbr::RtcAlrmbrSpec>;
#[doc = "RTC alarm B register"]
pub mod rtc_alrmbr;
#[doc = "RTC_ALRMBSSR (rw) register accessor: RTC alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmbssr`]
module"]
#[doc(alias = "RTC_ALRMBSSR")]
pub type RtcAlrmbssr = crate::Reg<rtc_alrmbssr::RtcAlrmbssrSpec>;
#[doc = "RTC alarm B sub second register"]
pub mod rtc_alrmbssr;
#[doc = "RTC_SR (r) register accessor: RTC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_sr`]
module"]
#[doc(alias = "RTC_SR")]
pub type RtcSr = crate::Reg<rtc_sr::RtcSrSpec>;
#[doc = "RTC status register"]
pub mod rtc_sr;
#[doc = "RTC_MISR (r) register accessor: RTC masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_misr`]
module"]
#[doc(alias = "RTC_MISR")]
pub type RtcMisr = crate::Reg<rtc_misr::RtcMisrSpec>;
#[doc = "RTC masked interrupt status register"]
pub mod rtc_misr;
#[doc = "RTC_SCR (w) register accessor: RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_scr`]
module"]
#[doc(alias = "RTC_SCR")]
pub type RtcScr = crate::Reg<rtc_scr::RtcScrSpec>;
#[doc = "RTC status clear register"]
pub mod rtc_scr;
