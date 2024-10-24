#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    icscr: Icscr,
    cfgr: Cfgr,
    pllsyscfgr: Pllsyscfgr,
    _reserved4: [u8; 0x08],
    cier: Cier,
    cifr: Cifr,
    cicr: Cicr,
    ioprstr: Ioprstr,
    ahbrstr: Ahbrstr,
    apbrstr1: Apbrstr1,
    apbrstr2: Apbrstr2,
    iopenr: Iopenr,
    ahbenr: Ahbenr,
    apbenr1: Apbenr1,
    apbenr2: Apbenr2,
    iopsmenr: Iopsmenr,
    ahbsmenr: Ahbsmenr,
    apbsmenr1: Apbsmenr1,
    apbsmenr2: Apbsmenr2,
    ccipr: Ccipr,
    ccipr2: Ccipr2,
    bdcr: Bdcr,
    csr: Csr,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn icscr(&self) -> &Icscr {
        &self.icscr
    }
    #[doc = "0x08 - Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x0c - PLL configuration register"]
    #[inline(always)]
    pub const fn pllsyscfgr(&self) -> &Pllsyscfgr {
        &self.pllsyscfgr
    }
    #[doc = "0x18 - Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(&self) -> &Cier {
        &self.cier
    }
    #[doc = "0x1c - Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &Cifr {
        &self.cifr
    }
    #[doc = "0x20 - Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &Cicr {
        &self.cicr
    }
    #[doc = "0x24 - I/O port reset register"]
    #[inline(always)]
    pub const fn ioprstr(&self) -> &Ioprstr {
        &self.ioprstr
    }
    #[doc = "0x28 - AHB peripheral reset register"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &Ahbrstr {
        &self.ahbrstr
    }
    #[doc = "0x2c - APB peripheral reset register 1"]
    #[inline(always)]
    pub const fn apbrstr1(&self) -> &Apbrstr1 {
        &self.apbrstr1
    }
    #[doc = "0x30 - APB peripheral reset register 2"]
    #[inline(always)]
    pub const fn apbrstr2(&self) -> &Apbrstr2 {
        &self.apbrstr2
    }
    #[doc = "0x34 - GPIO clock enable register"]
    #[inline(always)]
    pub const fn iopenr(&self) -> &Iopenr {
        &self.iopenr
    }
    #[doc = "0x38 - AHB peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &Ahbenr {
        &self.ahbenr
    }
    #[doc = "0x3c - APB peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn apbenr1(&self) -> &Apbenr1 {
        &self.apbenr1
    }
    #[doc = "0x40 - APB peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apbenr2(&self) -> &Apbenr2 {
        &self.apbenr2
    }
    #[doc = "0x44 - GPIO in Sleep mode clock enable register"]
    #[inline(always)]
    pub const fn iopsmenr(&self) -> &Iopsmenr {
        &self.iopsmenr
    }
    #[doc = "0x48 - AHB peripheral clock enable in Sleep mode register"]
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &Ahbsmenr {
        &self.ahbsmenr
    }
    #[doc = "0x4c - APB peripheral clock enable in Sleep mode register 1"]
    #[inline(always)]
    pub const fn apbsmenr1(&self) -> &Apbsmenr1 {
        &self.apbsmenr1
    }
    #[doc = "0x50 - APB peripheral clock enable in Sleep mode register 2"]
    #[inline(always)]
    pub const fn apbsmenr2(&self) -> &Apbsmenr2 {
        &self.apbsmenr2
    }
    #[doc = "0x54 - Peripherals independent clock configuration register"]
    #[inline(always)]
    pub const fn ccipr(&self) -> &Ccipr {
        &self.ccipr
    }
    #[doc = "0x58 - Peripherals independent clock configuration register 2"]
    #[inline(always)]
    pub const fn ccipr2(&self) -> &Ccipr2 {
        &self.ccipr2
    }
    #[doc = "0x5c - RTC domain control register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x60 - Control/status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
}
#[doc = "CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: Internal clock sources calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icscr`]
module"]
#[doc(alias = "ICSCR")]
pub type Icscr = crate::Reg<icscr::IcscrSpec>;
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLLSYSCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsyscfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsyscfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllsyscfgr`]
module"]
#[doc(alias = "PLLSYSCFGR")]
pub type Pllsyscfgr = crate::Reg<pllsyscfgr::PllsyscfgrSpec>;
#[doc = "PLL configuration register"]
pub mod pllsyscfgr;
#[doc = "CIER (rw) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
module"]
#[doc(alias = "CIER")]
pub type Cier = crate::Reg<cier::CierSpec>;
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: Clock interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`]
module"]
#[doc(alias = "CIFR")]
pub type Cifr = crate::Reg<cifr::CifrSpec>;
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR (w) register accessor: Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`]
module"]
#[doc(alias = "CICR")]
pub type Cicr = crate::Reg<cicr::CicrSpec>;
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "IOPRSTR (rw) register accessor: I/O port reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioprstr`]
module"]
#[doc(alias = "IOPRSTR")]
pub type Ioprstr = crate::Reg<ioprstr::IoprstrSpec>;
#[doc = "I/O port reset register"]
pub mod ioprstr;
#[doc = "AHBRSTR (rw) register accessor: AHB peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`]
module"]
#[doc(alias = "AHBRSTR")]
pub type Ahbrstr = crate::Reg<ahbrstr::AhbrstrSpec>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APBRSTR1 (rw) register accessor: APB peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrstr1`]
module"]
#[doc(alias = "APBRSTR1")]
pub type Apbrstr1 = crate::Reg<apbrstr1::Apbrstr1Spec>;
#[doc = "APB peripheral reset register 1"]
pub mod apbrstr1;
#[doc = "APBRSTR2 (rw) register accessor: APB peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrstr2`]
module"]
#[doc(alias = "APBRSTR2")]
pub type Apbrstr2 = crate::Reg<apbrstr2::Apbrstr2Spec>;
#[doc = "APB peripheral reset register 2"]
pub mod apbrstr2;
#[doc = "IOPENR (rw) register accessor: GPIO clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopenr`]
module"]
#[doc(alias = "IOPENR")]
pub type Iopenr = crate::Reg<iopenr::IopenrSpec>;
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHBENR (rw) register accessor: AHB peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`]
module"]
#[doc(alias = "AHBENR")]
pub type Ahbenr = crate::Reg<ahbenr::AhbenrSpec>;
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APBENR1 (rw) register accessor: APB peripheral clock enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbenr1`]
module"]
#[doc(alias = "APBENR1")]
pub type Apbenr1 = crate::Reg<apbenr1::Apbenr1Spec>;
#[doc = "APB peripheral clock enable register 1"]
pub mod apbenr1;
#[doc = "APBENR2 (rw) register accessor: APB peripheral clock enable register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbenr2`]
module"]
#[doc(alias = "APBENR2")]
pub type Apbenr2 = crate::Reg<apbenr2::Apbenr2Spec>;
#[doc = "APB peripheral clock enable register 2"]
pub mod apbenr2;
#[doc = "IOPSMENR (rw) register accessor: GPIO in Sleep mode clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopsmenr`]
module"]
#[doc(alias = "IOPSMENR")]
pub type Iopsmenr = crate::Reg<iopsmenr::IopsmenrSpec>;
#[doc = "GPIO in Sleep mode clock enable register"]
pub mod iopsmenr;
#[doc = "AHBSMENR (rw) register accessor: AHB peripheral clock enable in Sleep mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbsmenr`]
module"]
#[doc(alias = "AHBSMENR")]
pub type Ahbsmenr = crate::Reg<ahbsmenr::AhbsmenrSpec>;
#[doc = "AHB peripheral clock enable in Sleep mode register"]
pub mod ahbsmenr;
#[doc = "APBSMENR1 (rw) register accessor: APB peripheral clock enable in Sleep mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbsmenr1`]
module"]
#[doc(alias = "APBSMENR1")]
pub type Apbsmenr1 = crate::Reg<apbsmenr1::Apbsmenr1Spec>;
#[doc = "APB peripheral clock enable in Sleep mode register 1"]
pub mod apbsmenr1;
#[doc = "APBSMENR2 (rw) register accessor: APB peripheral clock enable in Sleep mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbsmenr2`]
module"]
#[doc(alias = "APBSMENR2")]
pub type Apbsmenr2 = crate::Reg<apbsmenr2::Apbsmenr2Spec>;
#[doc = "APB peripheral clock enable in Sleep mode register 2"]
pub mod apbsmenr2;
#[doc = "CCIPR (rw) register accessor: Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr`]
module"]
#[doc(alias = "CCIPR")]
pub type Ccipr = crate::Reg<ccipr::CciprSpec>;
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "CCIPR2 (rw) register accessor: Peripherals independent clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr2`]
module"]
#[doc(alias = "CCIPR2")]
pub type Ccipr2 = crate::Reg<ccipr2::Ccipr2Spec>;
#[doc = "Peripherals independent clock configuration register 2"]
pub mod ccipr2;
#[doc = "BDCR (rw) register accessor: RTC domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = "RTC domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: Control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Control/status register"]
pub mod csr;
