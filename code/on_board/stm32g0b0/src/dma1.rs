#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_isr: DmaIsr,
    dma_ifcr: DmaIfcr,
    dma_ccr1: DmaCcr1,
    dma_cndtr1: DmaCndtr1,
    dma_cpar1: DmaCpar1,
    dma_cmar1: DmaCmar1,
    _reserved6: [u8; 0x04],
    dma_ccr2: DmaCcr2,
    dma_cndtr2: DmaCndtr2,
    dma_cpar2: DmaCpar2,
    dma_cmar2: DmaCmar2,
    _reserved10: [u8; 0x04],
    dma_ccr3: DmaCcr3,
    dma_cndtr3: DmaCndtr3,
    dma_cpar3: DmaCpar3,
    dma_cmar3: DmaCmar3,
    _reserved14: [u8; 0x04],
    dma_ccr4: DmaCcr4,
    dma_cndtr4: DmaCndtr4,
    dma_cpar4: DmaCpar4,
    dma_cmar4: DmaCmar4,
    _reserved18: [u8; 0x04],
    dma_ccr5: DmaCcr5,
    dma_cndtr5: DmaCndtr5,
    dma_cpar5: DmaCpar5,
    dma_cmar5: DmaCmar5,
    _reserved22: [u8; 0x04],
    dma_ccr6: DmaCcr6,
    dma_cndtr6: DmaCndtr6,
    dma_cpar6: DmaCpar6,
    dma_cmar6: DmaCmar6,
    _reserved26: [u8; 0x04],
    dma_ccr7: DmaCcr7,
    dma_cndtr7: DmaCndtr7,
    dma_cpar7: DmaCpar7,
    dma_cmar7: DmaCmar7,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    #[inline(always)]
    pub const fn dma_isr(&self) -> &DmaIsr {
        &self.dma_isr
    }
    #[doc = "0x04 - DMA interrupt flag clear register"]
    #[inline(always)]
    pub const fn dma_ifcr(&self) -> &DmaIfcr {
        &self.dma_ifcr
    }
    #[doc = "0x08 - DMA channel 1 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr1(&self) -> &DmaCcr1 {
        &self.dma_ccr1
    }
    #[doc = "0x0c - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr1(&self) -> &DmaCndtr1 {
        &self.dma_cndtr1
    }
    #[doc = "0x10 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar1(&self) -> &DmaCpar1 {
        &self.dma_cpar1
    }
    #[doc = "0x14 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar1(&self) -> &DmaCmar1 {
        &self.dma_cmar1
    }
    #[doc = "0x1c - DMA channel 2 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr2(&self) -> &DmaCcr2 {
        &self.dma_ccr2
    }
    #[doc = "0x20 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr2(&self) -> &DmaCndtr2 {
        &self.dma_cndtr2
    }
    #[doc = "0x24 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar2(&self) -> &DmaCpar2 {
        &self.dma_cpar2
    }
    #[doc = "0x28 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar2(&self) -> &DmaCmar2 {
        &self.dma_cmar2
    }
    #[doc = "0x30 - DMA channel 3 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr3(&self) -> &DmaCcr3 {
        &self.dma_ccr3
    }
    #[doc = "0x34 - DMA channel x configuration register"]
    #[inline(always)]
    pub const fn dma_cndtr3(&self) -> &DmaCndtr3 {
        &self.dma_cndtr3
    }
    #[doc = "0x38 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar3(&self) -> &DmaCpar3 {
        &self.dma_cpar3
    }
    #[doc = "0x3c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar3(&self) -> &DmaCmar3 {
        &self.dma_cmar3
    }
    #[doc = "0x44 - DMA channel 4 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr4(&self) -> &DmaCcr4 {
        &self.dma_ccr4
    }
    #[doc = "0x48 - DMA channel x configuration register"]
    #[inline(always)]
    pub const fn dma_cndtr4(&self) -> &DmaCndtr4 {
        &self.dma_cndtr4
    }
    #[doc = "0x4c - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar4(&self) -> &DmaCpar4 {
        &self.dma_cpar4
    }
    #[doc = "0x50 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar4(&self) -> &DmaCmar4 {
        &self.dma_cmar4
    }
    #[doc = "0x58 - DMA channel 5 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr5(&self) -> &DmaCcr5 {
        &self.dma_ccr5
    }
    #[doc = "0x5c - DMA channel x configuration register"]
    #[inline(always)]
    pub const fn dma_cndtr5(&self) -> &DmaCndtr5 {
        &self.dma_cndtr5
    }
    #[doc = "0x60 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar5(&self) -> &DmaCpar5 {
        &self.dma_cpar5
    }
    #[doc = "0x64 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar5(&self) -> &DmaCmar5 {
        &self.dma_cmar5
    }
    #[doc = "0x6c - DMA channel 6 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr6(&self) -> &DmaCcr6 {
        &self.dma_ccr6
    }
    #[doc = "0x70 - DMA channel x configuration register"]
    #[inline(always)]
    pub const fn dma_cndtr6(&self) -> &DmaCndtr6 {
        &self.dma_cndtr6
    }
    #[doc = "0x74 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar6(&self) -> &DmaCpar6 {
        &self.dma_cpar6
    }
    #[doc = "0x78 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar6(&self) -> &DmaCmar6 {
        &self.dma_cmar6
    }
    #[doc = "0x80 - DMA channel 7 configuration register"]
    #[inline(always)]
    pub const fn dma_ccr7(&self) -> &DmaCcr7 {
        &self.dma_ccr7
    }
    #[doc = "0x84 - DMA channel x configuration register"]
    #[inline(always)]
    pub const fn dma_cndtr7(&self) -> &DmaCndtr7 {
        &self.dma_cndtr7
    }
    #[doc = "0x88 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar7(&self) -> &DmaCpar7 {
        &self.dma_cpar7
    }
    #[doc = "0x8c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn dma_cmar7(&self) -> &DmaCmar7 {
        &self.dma_cmar7
    }
}
#[doc = "DMA_ISR (r) register accessor: DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_isr`]
module"]
#[doc(alias = "DMA_ISR")]
pub type DmaIsr = crate::Reg<dma_isr::DmaIsrSpec>;
#[doc = "DMA interrupt status register"]
pub mod dma_isr;
#[doc = "DMA_IFCR (w) register accessor: DMA interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ifcr`]
module"]
#[doc(alias = "DMA_IFCR")]
pub type DmaIfcr = crate::Reg<dma_ifcr::DmaIfcrSpec>;
#[doc = "DMA interrupt flag clear register"]
pub mod dma_ifcr;
#[doc = "DMA_CCR1 (rw) register accessor: DMA channel 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr1`]
module"]
#[doc(alias = "DMA_CCR1")]
pub type DmaCcr1 = crate::Reg<dma_ccr1::DmaCcr1Spec>;
#[doc = "DMA channel 1 configuration register"]
pub mod dma_ccr1;
#[doc = "DMA_CNDTR1 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr1`]
module"]
#[doc(alias = "DMA_CNDTR1")]
pub type DmaCndtr1 = crate::Reg<dma_cndtr1::DmaCndtr1Spec>;
#[doc = "DMA channel x number of data register"]
pub mod dma_cndtr1;
#[doc = "DMA_CPAR1 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar1`]
module"]
#[doc(alias = "DMA_CPAR1")]
pub type DmaCpar1 = crate::Reg<dma_cpar1::DmaCpar1Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar1;
#[doc = "DMA_CMAR1 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar1`]
module"]
#[doc(alias = "DMA_CMAR1")]
pub type DmaCmar1 = crate::Reg<dma_cmar1::DmaCmar1Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar1;
#[doc = "DMA_CCR2 (rw) register accessor: DMA channel 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr2`]
module"]
#[doc(alias = "DMA_CCR2")]
pub type DmaCcr2 = crate::Reg<dma_ccr2::DmaCcr2Spec>;
#[doc = "DMA channel 2 configuration register"]
pub mod dma_ccr2;
#[doc = "DMA_CNDTR2 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr2`]
module"]
#[doc(alias = "DMA_CNDTR2")]
pub type DmaCndtr2 = crate::Reg<dma_cndtr2::DmaCndtr2Spec>;
#[doc = "DMA channel x number of data register"]
pub mod dma_cndtr2;
#[doc = "DMA_CPAR2 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar2`]
module"]
#[doc(alias = "DMA_CPAR2")]
pub type DmaCpar2 = crate::Reg<dma_cpar2::DmaCpar2Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar2;
#[doc = "DMA_CMAR2 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar2`]
module"]
#[doc(alias = "DMA_CMAR2")]
pub type DmaCmar2 = crate::Reg<dma_cmar2::DmaCmar2Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar2;
#[doc = "DMA_CCR3 (rw) register accessor: DMA channel 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr3`]
module"]
#[doc(alias = "DMA_CCR3")]
pub type DmaCcr3 = crate::Reg<dma_ccr3::DmaCcr3Spec>;
#[doc = "DMA channel 3 configuration register"]
pub mod dma_ccr3;
#[doc = "DMA_CNDTR3 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr3`]
module"]
#[doc(alias = "DMA_CNDTR3")]
pub type DmaCndtr3 = crate::Reg<dma_cndtr3::DmaCndtr3Spec>;
#[doc = "DMA channel x configuration register"]
pub mod dma_cndtr3;
#[doc = "DMA_CPAR3 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar3`]
module"]
#[doc(alias = "DMA_CPAR3")]
pub type DmaCpar3 = crate::Reg<dma_cpar3::DmaCpar3Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar3;
#[doc = "DMA_CMAR3 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar3`]
module"]
#[doc(alias = "DMA_CMAR3")]
pub type DmaCmar3 = crate::Reg<dma_cmar3::DmaCmar3Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar3;
#[doc = "DMA_CCR4 (rw) register accessor: DMA channel 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr4`]
module"]
#[doc(alias = "DMA_CCR4")]
pub type DmaCcr4 = crate::Reg<dma_ccr4::DmaCcr4Spec>;
#[doc = "DMA channel 4 configuration register"]
pub mod dma_ccr4;
#[doc = "DMA_CNDTR4 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr4`]
module"]
#[doc(alias = "DMA_CNDTR4")]
pub type DmaCndtr4 = crate::Reg<dma_cndtr4::DmaCndtr4Spec>;
#[doc = "DMA channel x configuration register"]
pub mod dma_cndtr4;
#[doc = "DMA_CPAR4 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar4`]
module"]
#[doc(alias = "DMA_CPAR4")]
pub type DmaCpar4 = crate::Reg<dma_cpar4::DmaCpar4Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar4;
#[doc = "DMA_CMAR4 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar4`]
module"]
#[doc(alias = "DMA_CMAR4")]
pub type DmaCmar4 = crate::Reg<dma_cmar4::DmaCmar4Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar4;
#[doc = "DMA_CCR5 (rw) register accessor: DMA channel 5 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr5`]
module"]
#[doc(alias = "DMA_CCR5")]
pub type DmaCcr5 = crate::Reg<dma_ccr5::DmaCcr5Spec>;
#[doc = "DMA channel 5 configuration register"]
pub mod dma_ccr5;
#[doc = "DMA_CNDTR5 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr5`]
module"]
#[doc(alias = "DMA_CNDTR5")]
pub type DmaCndtr5 = crate::Reg<dma_cndtr5::DmaCndtr5Spec>;
#[doc = "DMA channel x configuration register"]
pub mod dma_cndtr5;
#[doc = "DMA_CPAR5 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar5`]
module"]
#[doc(alias = "DMA_CPAR5")]
pub type DmaCpar5 = crate::Reg<dma_cpar5::DmaCpar5Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar5;
#[doc = "DMA_CMAR5 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar5`]
module"]
#[doc(alias = "DMA_CMAR5")]
pub type DmaCmar5 = crate::Reg<dma_cmar5::DmaCmar5Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar5;
#[doc = "DMA_CCR6 (rw) register accessor: DMA channel 6 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr6`]
module"]
#[doc(alias = "DMA_CCR6")]
pub type DmaCcr6 = crate::Reg<dma_ccr6::DmaCcr6Spec>;
#[doc = "DMA channel 6 configuration register"]
pub mod dma_ccr6;
#[doc = "DMA_CNDTR6 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr6`]
module"]
#[doc(alias = "DMA_CNDTR6")]
pub type DmaCndtr6 = crate::Reg<dma_cndtr6::DmaCndtr6Spec>;
#[doc = "DMA channel x configuration register"]
pub mod dma_cndtr6;
#[doc = "DMA_CPAR6 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar6`]
module"]
#[doc(alias = "DMA_CPAR6")]
pub type DmaCpar6 = crate::Reg<dma_cpar6::DmaCpar6Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar6;
#[doc = "DMA_CMAR6 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar6`]
module"]
#[doc(alias = "DMA_CMAR6")]
pub type DmaCmar6 = crate::Reg<dma_cmar6::DmaCmar6Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar6;
#[doc = "DMA_CCR7 (rw) register accessor: DMA channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr7`]
module"]
#[doc(alias = "DMA_CCR7")]
pub type DmaCcr7 = crate::Reg<dma_ccr7::DmaCcr7Spec>;
#[doc = "DMA channel 7 configuration register"]
pub mod dma_ccr7;
#[doc = "DMA_CNDTR7 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr7`]
module"]
#[doc(alias = "DMA_CNDTR7")]
pub type DmaCndtr7 = crate::Reg<dma_cndtr7::DmaCndtr7Spec>;
#[doc = "DMA channel x configuration register"]
pub mod dma_cndtr7;
#[doc = "DMA_CPAR7 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar7`]
module"]
#[doc(alias = "DMA_CPAR7")]
pub type DmaCpar7 = crate::Reg<dma_cpar7::DmaCpar7Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod dma_cpar7;
#[doc = "DMA_CMAR7 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cmar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cmar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar7`]
module"]
#[doc(alias = "DMA_CMAR7")]
pub type DmaCmar7 = crate::Reg<dma_cmar7::DmaCmar7Spec>;
#[doc = "DMA channel x memory address register"]
pub mod dma_cmar7;
