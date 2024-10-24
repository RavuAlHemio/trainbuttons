#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ier: Ier,
    cr: Cr,
    cfgr1: Cfgr1,
    cfgr2: Cfgr2,
    smpr: Smpr,
    _reserved6: [u8; 0x08],
    awd1tr: Awd1tr,
    awd2tr: Awd2tr,
    _reserved_8_chselr_: [u8; 0x04],
    awd3tr: Awd3tr,
    _reserved10: [u8; 0x10],
    dr: Dr,
    _reserved11: [u8; 0x5c],
    awd2cr: Awd2cr,
    awd3cr: Awd3cr,
    _reserved13: [u8; 0x0c],
    calfact: Calfact,
    _reserved14: [u8; 0x0250],
    ccr: Ccr,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x14 - ADC sampling time register"]
    #[inline(always)]
    pub const fn smpr(&self) -> &Smpr {
        &self.smpr
    }
    #[doc = "0x20 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn awd1tr(&self) -> &Awd1tr {
        &self.awd1tr
    }
    #[doc = "0x24 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn awd2tr(&self) -> &Awd2tr {
        &self.awd2tr
    }
    #[doc = "0x28 - channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[inline(always)]
    pub const fn chselr_1(&self) -> &Chselr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - ADC channel selection register \\[alternate\\]"]
    #[inline(always)]
    pub const fn chselr_0(&self) -> &Chselr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn awd3tr(&self) -> &Awd3tr {
        &self.awd3tr
    }
    #[doc = "0x40 - ADC data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &Awd2cr {
        &self.awd2cr
    }
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &Awd3cr {
        &self.awd3cr
    }
    #[doc = "0xb4 - ADC Calibration factor"]
    #[inline(always)]
    pub const fn calfact(&self) -> &Calfact {
        &self.calfact
    }
    #[doc = "0x308 - ADC common configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR1 (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "ADC configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR (rw) register accessor: ADC sampling time register\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr`]
module"]
#[doc(alias = "SMPR")]
pub type Smpr = crate::Reg<smpr::SmprSpec>;
#[doc = "ADC sampling time register"]
pub mod smpr;
#[doc = "AWD1TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd1tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd1tr`]
module"]
#[doc(alias = "AWD1TR")]
pub type Awd1tr = crate::Reg<awd1tr::Awd1trSpec>;
#[doc = "ADC watchdog threshold register"]
pub mod awd1tr;
#[doc = "AWD2TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2tr`]
module"]
#[doc(alias = "AWD2TR")]
pub type Awd2tr = crate::Reg<awd2tr::Awd2trSpec>;
#[doc = "ADC watchdog threshold register"]
pub mod awd2tr;
#[doc = "CHSELR_0 (rw) register accessor: ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`chselr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr_0`]
module"]
#[doc(alias = "CHSELR_0")]
pub type Chselr0 = crate::Reg<chselr_0::Chselr0Spec>;
#[doc = "ADC channel selection register \\[alternate\\]"]
pub mod chselr_0;
#[doc = "CHSELR_1 (rw) register accessor: channel selection register CHSELRMOD = 1 in ADC_CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`chselr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr_1`]
module"]
#[doc(alias = "CHSELR_1")]
pub type Chselr1 = crate::Reg<chselr_1::Chselr1Spec>;
#[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
pub mod chselr_1;
#[doc = "AWD3TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3tr`]
module"]
#[doc(alias = "AWD3TR")]
pub type Awd3tr = crate::Reg<awd3tr::Awd3trSpec>;
#[doc = "ADC watchdog threshold register"]
pub mod awd3tr;
#[doc = "DR (r) register accessor: ADC data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "ADC data register"]
pub mod dr;
#[doc = "AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`]
module"]
#[doc(alias = "AWD2CR")]
pub type Awd2cr = crate::Reg<awd2cr::Awd2crSpec>;
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`]
module"]
#[doc(alias = "AWD3CR")]
pub type Awd3cr = crate::Reg<awd3cr::Awd3crSpec>;
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod awd3cr;
#[doc = "CALFACT (rw) register accessor: ADC Calibration factor\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`]
module"]
#[doc(alias = "CALFACT")]
pub type Calfact = crate::Reg<calfact::CalfactSpec>;
#[doc = "ADC Calibration factor"]
pub mod calfact;
#[doc = "CCR (rw) register accessor: ADC common configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "ADC common configuration register"]
pub mod ccr;
