#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ifcr: Ifcr,
    ccr1: Ccr1,
    cndtr1: Cndtr1,
    cpar1: Cpar1,
    cmar1: Cmar1,
    _reserved6: [u8; 0x04],
    ccr2: Ccr2,
    cndtr2: Cndtr2,
    cpar2: Cpar2,
    cmar2: Cmar2,
    _reserved10: [u8; 0x04],
    ccr3: Ccr3,
    cndtr3: Cndtr3,
    cpar3: Cpar3,
    cmar3: Cmar3,
    _reserved14: [u8; 0x04],
    ccr4: Ccr4,
    cndtr4: Cndtr4,
    cpar4: Cpar4,
    cmar4: Cmar4,
    _reserved18: [u8; 0x04],
    ccr5: Ccr5,
    cndtr5: Cndtr5,
    cpar5: Cpar5,
    cmar5: Cmar5,
    _reserved22: [u8; 0x04],
    ccr6: Ccr6,
    cndtr6: Cndtr6,
    cpar6: Cpar6,
    cmar6: Cmar6,
    _reserved26: [u8; 0x04],
    ccr7: Ccr7,
    cndtr7: Cndtr7,
    cpar7: Cpar7,
    cmar7: Cmar7,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - DMA interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x08 - DMA channel 1 configuration register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x0c - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr1(&self) -> &Cndtr1 {
        &self.cndtr1
    }
    #[doc = "0x10 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar1(&self) -> &Cpar1 {
        &self.cpar1
    }
    #[doc = "0x14 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar1(&self) -> &Cmar1 {
        &self.cmar1
    }
    #[doc = "0x1c - DMA channel 2 configuration register"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x20 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr2(&self) -> &Cndtr2 {
        &self.cndtr2
    }
    #[doc = "0x24 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar2(&self) -> &Cpar2 {
        &self.cpar2
    }
    #[doc = "0x28 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar2(&self) -> &Cmar2 {
        &self.cmar2
    }
    #[doc = "0x30 - DMA channel 3 configuration register"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x34 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr3(&self) -> &Cndtr3 {
        &self.cndtr3
    }
    #[doc = "0x38 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar3(&self) -> &Cpar3 {
        &self.cpar3
    }
    #[doc = "0x3c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar3(&self) -> &Cmar3 {
        &self.cmar3
    }
    #[doc = "0x44 - DMA channel 4 configuration register"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x48 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr4(&self) -> &Cndtr4 {
        &self.cndtr4
    }
    #[doc = "0x4c - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar4(&self) -> &Cpar4 {
        &self.cpar4
    }
    #[doc = "0x50 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar4(&self) -> &Cmar4 {
        &self.cmar4
    }
    #[doc = "0x58 - DMA channel 5 configuration register"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    #[doc = "0x5c - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr5(&self) -> &Cndtr5 {
        &self.cndtr5
    }
    #[doc = "0x60 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar5(&self) -> &Cpar5 {
        &self.cpar5
    }
    #[doc = "0x64 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar5(&self) -> &Cmar5 {
        &self.cmar5
    }
    #[doc = "0x6c - DMA channel 6 configuration register"]
    #[inline(always)]
    pub const fn ccr6(&self) -> &Ccr6 {
        &self.ccr6
    }
    #[doc = "0x70 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr6(&self) -> &Cndtr6 {
        &self.cndtr6
    }
    #[doc = "0x74 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar6(&self) -> &Cpar6 {
        &self.cpar6
    }
    #[doc = "0x78 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar6(&self) -> &Cmar6 {
        &self.cmar6
    }
    #[doc = "0x80 - DMA channel 7 configuration register"]
    #[inline(always)]
    pub const fn ccr7(&self) -> &Ccr7 {
        &self.ccr7
    }
    #[doc = "0x84 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn cndtr7(&self) -> &Cndtr7 {
        &self.cndtr7
    }
    #[doc = "0x88 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar7(&self) -> &Cpar7 {
        &self.cpar7
    }
    #[doc = "0x8c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar7(&self) -> &Cmar7 {
        &self.cmar7
    }
}
#[doc = "ISR (r) register accessor: DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: DMA channel 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "DMA channel 1 configuration register"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`]
module"]
#[doc(alias = "CNDTR1")]
pub type Cndtr1 = crate::Reg<cndtr1::Cndtr1Spec>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`]
module"]
#[doc(alias = "CPAR1")]
pub type Cpar1 = crate::Reg<cpar1::Cpar1Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`]
module"]
#[doc(alias = "CMAR1")]
pub type Cmar1 = crate::Reg<cmar1::Cmar1Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar1;
pub use ccr1 as ccr2;
pub use ccr1 as ccr3;
pub use ccr1 as ccr4;
pub use ccr1 as ccr5;
pub use ccr1 as ccr6;
pub use ccr1 as ccr7;
pub use cmar1 as cmar2;
pub use cmar1 as cmar3;
pub use cmar1 as cmar4;
pub use cmar1 as cmar5;
pub use cmar1 as cmar6;
pub use cmar1 as cmar7;
pub use cndtr1 as cndtr2;
pub use cndtr1 as cndtr3;
pub use cndtr1 as cndtr4;
pub use cndtr1 as cndtr5;
pub use cndtr1 as cndtr6;
pub use cndtr1 as cndtr7;
pub use cpar1 as cpar2;
pub use cpar1 as cpar3;
pub use cpar1 as cpar4;
pub use cpar1 as cpar5;
pub use cpar1 as cpar6;
pub use cpar1 as cpar7;
pub use Ccr1 as Ccr2;
pub use Ccr1 as Ccr3;
pub use Ccr1 as Ccr4;
pub use Ccr1 as Ccr5;
pub use Ccr1 as Ccr6;
pub use Ccr1 as Ccr7;
pub use Cmar1 as Cmar2;
pub use Cmar1 as Cmar3;
pub use Cmar1 as Cmar4;
pub use Cmar1 as Cmar5;
pub use Cmar1 as Cmar6;
pub use Cmar1 as Cmar7;
pub use Cndtr1 as Cndtr2;
pub use Cndtr1 as Cndtr3;
pub use Cndtr1 as Cndtr4;
pub use Cndtr1 as Cndtr5;
pub use Cndtr1 as Cndtr6;
pub use Cndtr1 as Cndtr7;
pub use Cpar1 as Cpar2;
pub use Cpar1 as Cpar3;
pub use Cpar1 as Cpar4;
pub use Cpar1 as Cpar5;
pub use Cpar1 as Cpar6;
pub use Cpar1 as Cpar7;
