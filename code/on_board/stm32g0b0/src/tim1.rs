#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim1_cr1: Tim1Cr1,
    tim1_cr2: Tim1Cr2,
    tim1_smcr: Tim1Smcr,
    tim1_dier: Tim1Dier,
    tim1_sr: Tim1Sr,
    tim1_egr: Tim1Egr,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    tim1_ccer: Tim1Ccer,
    tim1_cnt: Tim1Cnt,
    tim1_psc: Tim1Psc,
    tim1_arr: Tim1Arr,
    tim1_rcr: Tim1Rcr,
    tim1_ccr1: Tim1Ccr1,
    tim1_ccr2: Tim1Ccr2,
    tim1_ccr3: Tim1Ccr3,
    tim1_ccr4: Tim1Ccr4,
    tim1_bdtr: Tim1Bdtr,
    tim1_dcr: Tim1Dcr,
    tim1_dmar: Tim1Dmar,
    tim1_or1: Tim1Or1,
    ccmr3_output: Ccmr3Output,
    tim1_ccr5: Tim1Ccr5,
    tim1_ccr6: Tim1Ccr6,
    tim1_af1: Tim1Af1,
    tim1_af2: Tim1Af2,
    tim1_tisel: Tim1Tisel,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn tim1_cr1(&self) -> &Tim1Cr1 {
        &self.tim1_cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn tim1_cr2(&self) -> &Tim1Cr2 {
        &self.tim1_cr2
    }
    #[doc = "0x08 - slave mode control register"]
    #[inline(always)]
    pub const fn tim1_smcr(&self) -> &Tim1Smcr {
        &self.tim1_smcr
    }
    #[doc = "0x0c - DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn tim1_dier(&self) -> &Tim1Dier {
        &self.tim1_dier
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn tim1_sr(&self) -> &Tim1Sr {
        &self.tim1_sr
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn tim1_egr(&self) -> &Tim1Egr {
        &self.tim1_egr
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &Ccmr1Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &Ccmr1Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &Ccmr2Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &Ccmr2Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - capture/compare enable register"]
    #[inline(always)]
    pub const fn tim1_ccer(&self) -> &Tim1Ccer {
        &self.tim1_ccer
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn tim1_cnt(&self) -> &Tim1Cnt {
        &self.tim1_cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn tim1_psc(&self) -> &Tim1Psc {
        &self.tim1_psc
    }
    #[doc = "0x2c - auto-reload register"]
    #[inline(always)]
    pub const fn tim1_arr(&self) -> &Tim1Arr {
        &self.tim1_arr
    }
    #[doc = "0x30 - repetition counter register"]
    #[inline(always)]
    pub const fn tim1_rcr(&self) -> &Tim1Rcr {
        &self.tim1_rcr
    }
    #[doc = "0x34 - capture/compare register 1"]
    #[inline(always)]
    pub const fn tim1_ccr1(&self) -> &Tim1Ccr1 {
        &self.tim1_ccr1
    }
    #[doc = "0x38 - capture/compare register 2"]
    #[inline(always)]
    pub const fn tim1_ccr2(&self) -> &Tim1Ccr2 {
        &self.tim1_ccr2
    }
    #[doc = "0x3c - capture/compare register 3"]
    #[inline(always)]
    pub const fn tim1_ccr3(&self) -> &Tim1Ccr3 {
        &self.tim1_ccr3
    }
    #[doc = "0x40 - capture/compare register 4"]
    #[inline(always)]
    pub const fn tim1_ccr4(&self) -> &Tim1Ccr4 {
        &self.tim1_ccr4
    }
    #[doc = "0x44 - break and dead-time register"]
    #[inline(always)]
    pub const fn tim1_bdtr(&self) -> &Tim1Bdtr {
        &self.tim1_bdtr
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn tim1_dcr(&self) -> &Tim1Dcr {
        &self.tim1_dcr
    }
    #[doc = "0x4c - DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim1_dmar(&self) -> &Tim1Dmar {
        &self.tim1_dmar
    }
    #[doc = "0x50 - option register 1"]
    #[inline(always)]
    pub const fn tim1_or1(&self) -> &Tim1Or1 {
        &self.tim1_or1
    }
    #[doc = "0x54 - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr3_output(&self) -> &Ccmr3Output {
        &self.ccmr3_output
    }
    #[doc = "0x58 - capture/compare register 4"]
    #[inline(always)]
    pub const fn tim1_ccr5(&self) -> &Tim1Ccr5 {
        &self.tim1_ccr5
    }
    #[doc = "0x5c - capture/compare register 4"]
    #[inline(always)]
    pub const fn tim1_ccr6(&self) -> &Tim1Ccr6 {
        &self.tim1_ccr6
    }
    #[doc = "0x60 - DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim1_af1(&self) -> &Tim1Af1 {
        &self.tim1_af1
    }
    #[doc = "0x64 - DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim1_af2(&self) -> &Tim1Af2 {
        &self.tim1_af2
    }
    #[doc = "0x68 - TIM1 timer input selection register"]
    #[inline(always)]
    pub const fn tim1_tisel(&self) -> &Tim1Tisel {
        &self.tim1_tisel
    }
}
#[doc = "TIM1_CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cr1`]
module"]
#[doc(alias = "TIM1_CR1")]
pub type Tim1Cr1 = crate::Reg<tim1_cr1::Tim1Cr1Spec>;
#[doc = "control register 1"]
pub mod tim1_cr1;
#[doc = "TIM1_CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cr2`]
module"]
#[doc(alias = "TIM1_CR2")]
pub type Tim1Cr2 = crate::Reg<tim1_cr2::Tim1Cr2Spec>;
#[doc = "control register 2"]
pub mod tim1_cr2;
#[doc = "TIM1_SMCR (rw) register accessor: slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_smcr`]
module"]
#[doc(alias = "TIM1_SMCR")]
pub type Tim1Smcr = crate::Reg<tim1_smcr::Tim1SmcrSpec>;
#[doc = "slave mode control register"]
pub mod tim1_smcr;
#[doc = "TIM1_DIER (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dier`]
module"]
#[doc(alias = "TIM1_DIER")]
pub type Tim1Dier = crate::Reg<tim1_dier::Tim1DierSpec>;
#[doc = "DMA/Interrupt enable register"]
pub mod tim1_dier;
#[doc = "TIM1_SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_sr`]
module"]
#[doc(alias = "TIM1_SR")]
pub type Tim1Sr = crate::Reg<tim1_sr::Tim1SrSpec>;
#[doc = "status register"]
pub mod tim1_sr;
#[doc = "TIM1_EGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_egr`]
module"]
#[doc(alias = "TIM1_EGR")]
pub type Tim1Egr = crate::Reg<tim1_egr::Tim1EgrSpec>;
#[doc = "event generation register"]
pub mod tim1_egr;
#[doc = "CCMR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`]
module"]
#[doc(alias = "CCMR1_Output")]
pub type Ccmr1Output = crate::Reg<ccmr1_output::Ccmr1OutputSpec>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`]
module"]
#[doc(alias = "CCMR1_Input")]
pub type Ccmr1Input = crate::Reg<ccmr1_input::Ccmr1InputSpec>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_input;
#[doc = "CCMR2_Output (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_output`]
module"]
#[doc(alias = "CCMR2_Output")]
pub type Ccmr2Output = crate::Reg<ccmr2_output::Ccmr2OutputSpec>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "CCMR2_Input (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_input`]
module"]
#[doc(alias = "CCMR2_Input")]
pub type Ccmr2Input = crate::Reg<ccmr2_input::Ccmr2InputSpec>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_input;
#[doc = "TIM1_CCER (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccer`]
module"]
#[doc(alias = "TIM1_CCER")]
pub type Tim1Ccer = crate::Reg<tim1_ccer::Tim1CcerSpec>;
#[doc = "capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "TIM1_CNT (rw) register accessor: counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cnt`]
module"]
#[doc(alias = "TIM1_CNT")]
pub type Tim1Cnt = crate::Reg<tim1_cnt::Tim1CntSpec>;
#[doc = "counter"]
pub mod tim1_cnt;
#[doc = "TIM1_PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_psc`]
module"]
#[doc(alias = "TIM1_PSC")]
pub type Tim1Psc = crate::Reg<tim1_psc::Tim1PscSpec>;
#[doc = "prescaler"]
pub mod tim1_psc;
#[doc = "TIM1_ARR (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_arr`]
module"]
#[doc(alias = "TIM1_ARR")]
pub type Tim1Arr = crate::Reg<tim1_arr::Tim1ArrSpec>;
#[doc = "auto-reload register"]
pub mod tim1_arr;
#[doc = "TIM1_RCR (rw) register accessor: repetition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_rcr`]
module"]
#[doc(alias = "TIM1_RCR")]
pub type Tim1Rcr = crate::Reg<tim1_rcr::Tim1RcrSpec>;
#[doc = "repetition counter register"]
pub mod tim1_rcr;
#[doc = "TIM1_CCR1 (rw) register accessor: capture/compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr1`]
module"]
#[doc(alias = "TIM1_CCR1")]
pub type Tim1Ccr1 = crate::Reg<tim1_ccr1::Tim1Ccr1Spec>;
#[doc = "capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "TIM1_CCR2 (rw) register accessor: capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr2`]
module"]
#[doc(alias = "TIM1_CCR2")]
pub type Tim1Ccr2 = crate::Reg<tim1_ccr2::Tim1Ccr2Spec>;
#[doc = "capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "TIM1_CCR3 (rw) register accessor: capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr3`]
module"]
#[doc(alias = "TIM1_CCR3")]
pub type Tim1Ccr3 = crate::Reg<tim1_ccr3::Tim1Ccr3Spec>;
#[doc = "capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "TIM1_CCR4 (rw) register accessor: capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr4`]
module"]
#[doc(alias = "TIM1_CCR4")]
pub type Tim1Ccr4 = crate::Reg<tim1_ccr4::Tim1Ccr4Spec>;
#[doc = "capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "TIM1_BDTR (rw) register accessor: break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_bdtr`]
module"]
#[doc(alias = "TIM1_BDTR")]
pub type Tim1Bdtr = crate::Reg<tim1_bdtr::Tim1BdtrSpec>;
#[doc = "break and dead-time register"]
pub mod tim1_bdtr;
#[doc = "TIM1_DCR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dcr`]
module"]
#[doc(alias = "TIM1_DCR")]
pub type Tim1Dcr = crate::Reg<tim1_dcr::Tim1DcrSpec>;
#[doc = "DMA control register"]
pub mod tim1_dcr;
#[doc = "TIM1_DMAR (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dmar`]
module"]
#[doc(alias = "TIM1_DMAR")]
pub type Tim1Dmar = crate::Reg<tim1_dmar::Tim1DmarSpec>;
#[doc = "DMA address for full transfer"]
pub mod tim1_dmar;
#[doc = "TIM1_OR1 (rw) register accessor: option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_or1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_or1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_or1`]
module"]
#[doc(alias = "TIM1_OR1")]
pub type Tim1Or1 = crate::Reg<tim1_or1::Tim1Or1Spec>;
#[doc = "option register 1"]
pub mod tim1_or1;
#[doc = "CCMR3_Output (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3_output`]
module"]
#[doc(alias = "CCMR3_Output")]
pub type Ccmr3Output = crate::Reg<ccmr3_output::Ccmr3OutputSpec>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr3_output;
#[doc = "TIM1_CCR5 (rw) register accessor: capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr5`]
module"]
#[doc(alias = "TIM1_CCR5")]
pub type Tim1Ccr5 = crate::Reg<tim1_ccr5::Tim1Ccr5Spec>;
#[doc = "capture/compare register 4"]
pub mod tim1_ccr5;
#[doc = "TIM1_CCR6 (rw) register accessor: capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr6`]
module"]
#[doc(alias = "TIM1_CCR6")]
pub type Tim1Ccr6 = crate::Reg<tim1_ccr6::Tim1Ccr6Spec>;
#[doc = "capture/compare register 4"]
pub mod tim1_ccr6;
#[doc = "TIM1_AF1 (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_af1`]
module"]
#[doc(alias = "TIM1_AF1")]
pub type Tim1Af1 = crate::Reg<tim1_af1::Tim1Af1Spec>;
#[doc = "DMA address for full transfer"]
pub mod tim1_af1;
#[doc = "TIM1_AF2 (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_af2`]
module"]
#[doc(alias = "TIM1_AF2")]
pub type Tim1Af2 = crate::Reg<tim1_af2::Tim1Af2Spec>;
#[doc = "DMA address for full transfer"]
pub mod tim1_af2;
#[doc = "TIM1_TISEL (rw) register accessor: TIM1 timer input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_tisel`]
module"]
#[doc(alias = "TIM1_TISEL")]
pub type Tim1Tisel = crate::Reg<tim1_tisel::Tim1TiselSpec>;
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
