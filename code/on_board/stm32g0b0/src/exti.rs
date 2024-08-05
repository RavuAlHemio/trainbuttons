#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: Rtsr1,
    ftsr1: Ftsr1,
    swier1: Swier1,
    rpr1: Rpr1,
    fpr1: Fpr1,
    _reserved5: [u8; 0x4c],
    exticr1: Exticr1,
    exticr2: Exticr2,
    exticr3: Exticr3,
    exticr4: Exticr4,
    _reserved9: [u8; 0x10],
    imr1: Imr1,
    emr1: Emr1,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &Rtsr1 {
        &self.rtsr1
    }
    #[doc = "0x04 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &Ftsr1 {
        &self.ftsr1
    }
    #[doc = "0x08 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &Swier1 {
        &self.swier1
    }
    #[doc = "0x0c - EXTI rising edge pending register"]
    #[inline(always)]
    pub const fn rpr1(&self) -> &Rpr1 {
        &self.rpr1
    }
    #[doc = "0x10 - EXTI falling edge pending register"]
    #[inline(always)]
    pub const fn fpr1(&self) -> &Fpr1 {
        &self.fpr1
    }
    #[doc = "0x60 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &Exticr1 {
        &self.exticr1
    }
    #[doc = "0x64 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &Exticr2 {
        &self.exticr2
    }
    #[doc = "0x68 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &Exticr3 {
        &self.exticr3
    }
    #[doc = "0x6c - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr4(&self) -> &Exticr4 {
        &self.exticr4
    }
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr1(&self) -> &Imr1 {
        &self.imr1
    }
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr1(&self) -> &Emr1 {
        &self.emr1
    }
}
#[doc = "RTSR1 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`]
module"]
#[doc(alias = "RTSR1")]
pub type Rtsr1 = crate::Reg<rtsr1::Rtsr1Spec>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`]
module"]
#[doc(alias = "FTSR1")]
pub type Ftsr1 = crate::Reg<ftsr1::Ftsr1Spec>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`]
module"]
#[doc(alias = "SWIER1")]
pub type Swier1 = crate::Reg<swier1::Swier1Spec>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "RPR1 (rw) register accessor: EXTI rising edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr1`]
module"]
#[doc(alias = "RPR1")]
pub type Rpr1 = crate::Reg<rpr1::Rpr1Spec>;
#[doc = "EXTI rising edge pending register"]
pub mod rpr1;
#[doc = "FPR1 (rw) register accessor: EXTI falling edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpr1`]
module"]
#[doc(alias = "FPR1")]
pub type Fpr1 = crate::Reg<fpr1::Fpr1Spec>;
#[doc = "EXTI falling edge pending register"]
pub mod fpr1;
#[doc = "EXTICR1 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`]
module"]
#[doc(alias = "EXTICR1")]
pub type Exticr1 = crate::Reg<exticr1::Exticr1Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`]
module"]
#[doc(alias = "EXTICR2")]
pub type Exticr2 = crate::Reg<exticr2::Exticr2Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`]
module"]
#[doc(alias = "EXTICR3")]
pub type Exticr3 = crate::Reg<exticr3::Exticr3Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`]
module"]
#[doc(alias = "EXTICR4")]
pub type Exticr4 = crate::Reg<exticr4::Exticr4Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr4;
#[doc = "IMR1 (rw) register accessor: EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
#[doc(alias = "IMR1")]
pub type Imr1 = crate::Reg<imr1::Imr1Spec>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "EMR1 (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr1`]
module"]
#[doc(alias = "EMR1")]
pub type Emr1 = crate::Reg<emr1::Emr1Spec>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr1;
