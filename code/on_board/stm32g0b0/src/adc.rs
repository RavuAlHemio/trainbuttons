#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc_isr: AdcIsr,
    adc_ier: AdcIer,
    adc_cr: AdcCr,
    adc_cfgr1: AdcCfgr1,
    adc_cfgr2: AdcCfgr2,
    adc_smpr: AdcSmpr,
    _reserved6: [u8; 0x08],
    adc_awd1tr: AdcAwd1tr,
    adc_awd2tr: AdcAwd2tr,
    _reserved_8_adc_chselr_: [u8; 0x04],
    adc_awd3tr: AdcAwd3tr,
    _reserved10: [u8; 0x10],
    adc_dr: AdcDr,
    _reserved11: [u8; 0x5c],
    adc_awd2cr: AdcAwd2cr,
    adc_awd3cr: AdcAwd3cr,
    _reserved13: [u8; 0x0c],
    adc_calfact: AdcCalfact,
    _reserved14: [u8; 0x0250],
    adc_ccr: AdcCcr,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn adc_isr(&self) -> &AdcIsr {
        &self.adc_isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn adc_ier(&self) -> &AdcIer {
        &self.adc_ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn adc_cr(&self) -> &AdcCr {
        &self.adc_cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn adc_cfgr1(&self) -> &AdcCfgr1 {
        &self.adc_cfgr1
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn adc_cfgr2(&self) -> &AdcCfgr2 {
        &self.adc_cfgr2
    }
    #[doc = "0x14 - ADC sampling time register"]
    #[inline(always)]
    pub const fn adc_smpr(&self) -> &AdcSmpr {
        &self.adc_smpr
    }
    #[doc = "0x20 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn adc_awd1tr(&self) -> &AdcAwd1tr {
        &self.adc_awd1tr
    }
    #[doc = "0x24 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn adc_awd2tr(&self) -> &AdcAwd2tr {
        &self.adc_awd2tr
    }
    #[doc = "0x28 - channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[inline(always)]
    pub const fn adc_chselr_1(&self) -> &AdcChselr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - ADC channel selection register \\[alternate\\]"]
    #[inline(always)]
    pub const fn adc_chselr_0(&self) -> &AdcChselr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn adc_awd3tr(&self) -> &AdcAwd3tr {
        &self.adc_awd3tr
    }
    #[doc = "0x40 - ADC data register"]
    #[inline(always)]
    pub const fn adc_dr(&self) -> &AdcDr {
        &self.adc_dr
    }
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    #[inline(always)]
    pub const fn adc_awd2cr(&self) -> &AdcAwd2cr {
        &self.adc_awd2cr
    }
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    #[inline(always)]
    pub const fn adc_awd3cr(&self) -> &AdcAwd3cr {
        &self.adc_awd3cr
    }
    #[doc = "0xb4 - ADC Calibration factor"]
    #[inline(always)]
    pub const fn adc_calfact(&self) -> &AdcCalfact {
        &self.adc_calfact
    }
    #[doc = "0x308 - ADC common configuration register"]
    #[inline(always)]
    pub const fn adc_ccr(&self) -> &AdcCcr {
        &self.adc_ccr
    }
}
#[doc = "ADC_ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_isr`]
module"]
#[doc(alias = "ADC_ISR")]
pub type AdcIsr = crate::Reg<adc_isr::AdcIsrSpec>;
#[doc = "ADC interrupt and status register"]
pub mod adc_isr;
#[doc = "ADC_IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ier`]
module"]
#[doc(alias = "ADC_IER")]
pub type AdcIer = crate::Reg<adc_ier::AdcIerSpec>;
#[doc = "ADC interrupt enable register"]
pub mod adc_ier;
#[doc = "ADC_CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cr`]
module"]
#[doc(alias = "ADC_CR")]
pub type AdcCr = crate::Reg<adc_cr::AdcCrSpec>;
#[doc = "ADC control register"]
pub mod adc_cr;
#[doc = "ADC_CFGR1 (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfgr1`]
module"]
#[doc(alias = "ADC_CFGR1")]
pub type AdcCfgr1 = crate::Reg<adc_cfgr1::AdcCfgr1Spec>;
#[doc = "ADC configuration register 1"]
pub mod adc_cfgr1;
#[doc = "ADC_CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfgr2`]
module"]
#[doc(alias = "ADC_CFGR2")]
pub type AdcCfgr2 = crate::Reg<adc_cfgr2::AdcCfgr2Spec>;
#[doc = "ADC configuration register 2"]
pub mod adc_cfgr2;
#[doc = "ADC_SMPR (rw) register accessor: ADC sampling time register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_smpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_smpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_smpr`]
module"]
#[doc(alias = "ADC_SMPR")]
pub type AdcSmpr = crate::Reg<adc_smpr::AdcSmprSpec>;
#[doc = "ADC sampling time register"]
pub mod adc_smpr;
#[doc = "ADC_AWD1TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_awd1tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd1tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd1tr`]
module"]
#[doc(alias = "ADC_AWD1TR")]
pub type AdcAwd1tr = crate::Reg<adc_awd1tr::AdcAwd1trSpec>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd1tr;
#[doc = "ADC_AWD2TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_awd2tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd2tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd2tr`]
module"]
#[doc(alias = "ADC_AWD2TR")]
pub type AdcAwd2tr = crate::Reg<adc_awd2tr::AdcAwd2trSpec>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd2tr;
#[doc = "ADC_CHSELR_0 (rw) register accessor: ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chselr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chselr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chselr_0`]
module"]
#[doc(alias = "ADC_CHSELR_0")]
pub type AdcChselr0 = crate::Reg<adc_chselr_0::AdcChselr0Spec>;
#[doc = "ADC channel selection register \\[alternate\\]"]
pub mod adc_chselr_0;
#[doc = "ADC_CHSELR_1 (rw) register accessor: channel selection register CHSELRMOD = 1 in ADC_CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chselr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chselr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chselr_1`]
module"]
#[doc(alias = "ADC_CHSELR_1")]
pub type AdcChselr1 = crate::Reg<adc_chselr_1::AdcChselr1Spec>;
#[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
pub mod adc_chselr_1;
#[doc = "ADC_AWD3TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_awd3tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd3tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd3tr`]
module"]
#[doc(alias = "ADC_AWD3TR")]
pub type AdcAwd3tr = crate::Reg<adc_awd3tr::AdcAwd3trSpec>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd3tr;
#[doc = "ADC_DR (r) register accessor: ADC data register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dr`]
module"]
#[doc(alias = "ADC_DR")]
pub type AdcDr = crate::Reg<adc_dr::AdcDrSpec>;
#[doc = "ADC data register"]
pub mod adc_dr;
#[doc = "ADC_AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd2cr`]
module"]
#[doc(alias = "ADC_AWD2CR")]
pub type AdcAwd2cr = crate::Reg<adc_awd2cr::AdcAwd2crSpec>;
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC_AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd3cr`]
module"]
#[doc(alias = "ADC_AWD3CR")]
pub type AdcAwd3cr = crate::Reg<adc_awd3cr::AdcAwd3crSpec>;
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC_CALFACT (rw) register accessor: ADC Calibration factor\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_calfact`]
module"]
#[doc(alias = "ADC_CALFACT")]
pub type AdcCalfact = crate::Reg<adc_calfact::AdcCalfactSpec>;
#[doc = "ADC Calibration factor"]
pub mod adc_calfact;
#[doc = "ADC_CCR (rw) register accessor: ADC common configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ccr`]
module"]
#[doc(alias = "ADC_CCR")]
pub type AdcCcr = crate::Reg<adc_ccr::AdcCcrSpec>;
#[doc = "ADC common configuration register"]
pub mod adc_ccr;
