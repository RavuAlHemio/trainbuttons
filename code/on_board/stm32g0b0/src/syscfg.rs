#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr1: Cfgr1,
    _reserved1: [u8; 0x14],
    cfgr2: Cfgr2,
    _reserved2: [u8; 0x64],
    itline0: Itline0,
    _reserved3: [u8; 0x04],
    itline2: Itline2,
    itline3: Itline3,
    itline4: Itline4,
    itline5: Itline5,
    itline6: Itline6,
    itline7: Itline7,
    itline8: Itline8,
    itline9: Itline9,
    itline10: Itline10,
    itline11: Itline11,
    itline12: Itline12,
    itline13: Itline13,
    itline14: Itline14,
    _reserved16: [u8; 0x04],
    itline16: Itline16,
    itline17: Itline17,
    itline18: Itline18,
    itline19: Itline19,
    itline20: Itline20,
    itline21: Itline21,
    itline22: Itline22,
    itline23: Itline23,
    itline24: Itline24,
    itline25: Itline25,
    itline26: Itline26,
    itline27: Itline27,
    itline28: Itline28,
    itline29: Itline29,
}
impl RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x18 - SYSCFG configuration register 1"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x80 - interrupt line 0 status register"]
    #[inline(always)]
    pub const fn itline0(&self) -> &Itline0 {
        &self.itline0
    }
    #[doc = "0x88 - interrupt line 2 status register"]
    #[inline(always)]
    pub const fn itline2(&self) -> &Itline2 {
        &self.itline2
    }
    #[doc = "0x8c - interrupt line 3 status register"]
    #[inline(always)]
    pub const fn itline3(&self) -> &Itline3 {
        &self.itline3
    }
    #[doc = "0x90 - interrupt line 4 status register"]
    #[inline(always)]
    pub const fn itline4(&self) -> &Itline4 {
        &self.itline4
    }
    #[doc = "0x94 - interrupt line 5 status register"]
    #[inline(always)]
    pub const fn itline5(&self) -> &Itline5 {
        &self.itline5
    }
    #[doc = "0x98 - interrupt line 6 status register"]
    #[inline(always)]
    pub const fn itline6(&self) -> &Itline6 {
        &self.itline6
    }
    #[doc = "0x9c - interrupt line 7 status register"]
    #[inline(always)]
    pub const fn itline7(&self) -> &Itline7 {
        &self.itline7
    }
    #[doc = "0xa0 - interrupt line 8 status register"]
    #[inline(always)]
    pub const fn itline8(&self) -> &Itline8 {
        &self.itline8
    }
    #[doc = "0xa4 - interrupt line 9 status register"]
    #[inline(always)]
    pub const fn itline9(&self) -> &Itline9 {
        &self.itline9
    }
    #[doc = "0xa8 - interrupt line 10 status register"]
    #[inline(always)]
    pub const fn itline10(&self) -> &Itline10 {
        &self.itline10
    }
    #[doc = "0xac - interrupt line 11 status register"]
    #[inline(always)]
    pub const fn itline11(&self) -> &Itline11 {
        &self.itline11
    }
    #[doc = "0xb0 - interrupt line 12 status register"]
    #[inline(always)]
    pub const fn itline12(&self) -> &Itline12 {
        &self.itline12
    }
    #[doc = "0xb4 - interrupt line 13 status register"]
    #[inline(always)]
    pub const fn itline13(&self) -> &Itline13 {
        &self.itline13
    }
    #[doc = "0xb8 - interrupt line 14 status register"]
    #[inline(always)]
    pub const fn itline14(&self) -> &Itline14 {
        &self.itline14
    }
    #[doc = "0xc0 - interrupt line 16 status register"]
    #[inline(always)]
    pub const fn itline16(&self) -> &Itline16 {
        &self.itline16
    }
    #[doc = "0xc4 - interrupt line 17 status register"]
    #[inline(always)]
    pub const fn itline17(&self) -> &Itline17 {
        &self.itline17
    }
    #[doc = "0xc8 - interrupt line 18 status register"]
    #[inline(always)]
    pub const fn itline18(&self) -> &Itline18 {
        &self.itline18
    }
    #[doc = "0xcc - interrupt line 19 status register"]
    #[inline(always)]
    pub const fn itline19(&self) -> &Itline19 {
        &self.itline19
    }
    #[doc = "0xd0 - interrupt line 20 status register"]
    #[inline(always)]
    pub const fn itline20(&self) -> &Itline20 {
        &self.itline20
    }
    #[doc = "0xd4 - interrupt line 21 status register"]
    #[inline(always)]
    pub const fn itline21(&self) -> &Itline21 {
        &self.itline21
    }
    #[doc = "0xd8 - interrupt line 22 status register"]
    #[inline(always)]
    pub const fn itline22(&self) -> &Itline22 {
        &self.itline22
    }
    #[doc = "0xdc - interrupt line 23 status register"]
    #[inline(always)]
    pub const fn itline23(&self) -> &Itline23 {
        &self.itline23
    }
    #[doc = "0xe0 - interrupt line 24 status register"]
    #[inline(always)]
    pub const fn itline24(&self) -> &Itline24 {
        &self.itline24
    }
    #[doc = "0xe4 - interrupt line 25 status register"]
    #[inline(always)]
    pub const fn itline25(&self) -> &Itline25 {
        &self.itline25
    }
    #[doc = "0xe8 - interrupt line 26 status register"]
    #[inline(always)]
    pub const fn itline26(&self) -> &Itline26 {
        &self.itline26
    }
    #[doc = "0xec - interrupt line 27 status register"]
    #[inline(always)]
    pub const fn itline27(&self) -> &Itline27 {
        &self.itline27
    }
    #[doc = "0xf0 - interrupt line 28 status register"]
    #[inline(always)]
    pub const fn itline28(&self) -> &Itline28 {
        &self.itline28
    }
    #[doc = "0xf4 - interrupt line 29 status register"]
    #[inline(always)]
    pub const fn itline29(&self) -> &Itline29 {
        &self.itline29
    }
}
#[doc = "CFGR1 (rw) register accessor: SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr2;
#[doc = "ITLINE0 (r) register accessor: interrupt line 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline0`]
module"]
#[doc(alias = "ITLINE0")]
pub type Itline0 = crate::Reg<itline0::Itline0Spec>;
#[doc = "interrupt line 0 status register"]
pub mod itline0;
#[doc = "ITLINE2 (r) register accessor: interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline2`]
module"]
#[doc(alias = "ITLINE2")]
pub type Itline2 = crate::Reg<itline2::Itline2Spec>;
#[doc = "interrupt line 2 status register"]
pub mod itline2;
#[doc = "ITLINE3 (r) register accessor: interrupt line 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline3`]
module"]
#[doc(alias = "ITLINE3")]
pub type Itline3 = crate::Reg<itline3::Itline3Spec>;
#[doc = "interrupt line 3 status register"]
pub mod itline3;
#[doc = "ITLINE4 (r) register accessor: interrupt line 4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline4`]
module"]
#[doc(alias = "ITLINE4")]
pub type Itline4 = crate::Reg<itline4::Itline4Spec>;
#[doc = "interrupt line 4 status register"]
pub mod itline4;
#[doc = "ITLINE5 (r) register accessor: interrupt line 5 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline5`]
module"]
#[doc(alias = "ITLINE5")]
pub type Itline5 = crate::Reg<itline5::Itline5Spec>;
#[doc = "interrupt line 5 status register"]
pub mod itline5;
#[doc = "ITLINE6 (r) register accessor: interrupt line 6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline6`]
module"]
#[doc(alias = "ITLINE6")]
pub type Itline6 = crate::Reg<itline6::Itline6Spec>;
#[doc = "interrupt line 6 status register"]
pub mod itline6;
#[doc = "ITLINE7 (r) register accessor: interrupt line 7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline7`]
module"]
#[doc(alias = "ITLINE7")]
pub type Itline7 = crate::Reg<itline7::Itline7Spec>;
#[doc = "interrupt line 7 status register"]
pub mod itline7;
#[doc = "ITLINE8 (r) register accessor: interrupt line 8 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline8`]
module"]
#[doc(alias = "ITLINE8")]
pub type Itline8 = crate::Reg<itline8::Itline8Spec>;
#[doc = "interrupt line 8 status register"]
pub mod itline8;
#[doc = "ITLINE9 (r) register accessor: interrupt line 9 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline9`]
module"]
#[doc(alias = "ITLINE9")]
pub type Itline9 = crate::Reg<itline9::Itline9Spec>;
#[doc = "interrupt line 9 status register"]
pub mod itline9;
#[doc = "ITLINE10 (r) register accessor: interrupt line 10 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline10`]
module"]
#[doc(alias = "ITLINE10")]
pub type Itline10 = crate::Reg<itline10::Itline10Spec>;
#[doc = "interrupt line 10 status register"]
pub mod itline10;
#[doc = "ITLINE11 (r) register accessor: interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline11`]
module"]
#[doc(alias = "ITLINE11")]
pub type Itline11 = crate::Reg<itline11::Itline11Spec>;
#[doc = "interrupt line 11 status register"]
pub mod itline11;
#[doc = "ITLINE12 (r) register accessor: interrupt line 12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline12`]
module"]
#[doc(alias = "ITLINE12")]
pub type Itline12 = crate::Reg<itline12::Itline12Spec>;
#[doc = "interrupt line 12 status register"]
pub mod itline12;
#[doc = "ITLINE13 (r) register accessor: interrupt line 13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline13`]
module"]
#[doc(alias = "ITLINE13")]
pub type Itline13 = crate::Reg<itline13::Itline13Spec>;
#[doc = "interrupt line 13 status register"]
pub mod itline13;
#[doc = "ITLINE14 (r) register accessor: interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline14`]
module"]
#[doc(alias = "ITLINE14")]
pub type Itline14 = crate::Reg<itline14::Itline14Spec>;
#[doc = "interrupt line 14 status register"]
pub mod itline14;
#[doc = "ITLINE16 (r) register accessor: interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline16`]
module"]
#[doc(alias = "ITLINE16")]
pub type Itline16 = crate::Reg<itline16::Itline16Spec>;
#[doc = "interrupt line 16 status register"]
pub mod itline16;
#[doc = "ITLINE17 (r) register accessor: interrupt line 17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline17`]
module"]
#[doc(alias = "ITLINE17")]
pub type Itline17 = crate::Reg<itline17::Itline17Spec>;
#[doc = "interrupt line 17 status register"]
pub mod itline17;
#[doc = "ITLINE18 (r) register accessor: interrupt line 18 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline18`]
module"]
#[doc(alias = "ITLINE18")]
pub type Itline18 = crate::Reg<itline18::Itline18Spec>;
#[doc = "interrupt line 18 status register"]
pub mod itline18;
#[doc = "ITLINE19 (r) register accessor: interrupt line 19 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline19`]
module"]
#[doc(alias = "ITLINE19")]
pub type Itline19 = crate::Reg<itline19::Itline19Spec>;
#[doc = "interrupt line 19 status register"]
pub mod itline19;
#[doc = "ITLINE20 (r) register accessor: interrupt line 20 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline20`]
module"]
#[doc(alias = "ITLINE20")]
pub type Itline20 = crate::Reg<itline20::Itline20Spec>;
#[doc = "interrupt line 20 status register"]
pub mod itline20;
#[doc = "ITLINE21 (r) register accessor: interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline21`]
module"]
#[doc(alias = "ITLINE21")]
pub type Itline21 = crate::Reg<itline21::Itline21Spec>;
#[doc = "interrupt line 21 status register"]
pub mod itline21;
#[doc = "ITLINE22 (r) register accessor: interrupt line 22 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline22`]
module"]
#[doc(alias = "ITLINE22")]
pub type Itline22 = crate::Reg<itline22::Itline22Spec>;
#[doc = "interrupt line 22 status register"]
pub mod itline22;
#[doc = "ITLINE23 (r) register accessor: interrupt line 23 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline23`]
module"]
#[doc(alias = "ITLINE23")]
pub type Itline23 = crate::Reg<itline23::Itline23Spec>;
#[doc = "interrupt line 23 status register"]
pub mod itline23;
#[doc = "ITLINE24 (r) register accessor: interrupt line 24 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline24`]
module"]
#[doc(alias = "ITLINE24")]
pub type Itline24 = crate::Reg<itline24::Itline24Spec>;
#[doc = "interrupt line 24 status register"]
pub mod itline24;
#[doc = "ITLINE25 (r) register accessor: interrupt line 25 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline25`]
module"]
#[doc(alias = "ITLINE25")]
pub type Itline25 = crate::Reg<itline25::Itline25Spec>;
#[doc = "interrupt line 25 status register"]
pub mod itline25;
#[doc = "ITLINE26 (r) register accessor: interrupt line 26 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline26::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline26`]
module"]
#[doc(alias = "ITLINE26")]
pub type Itline26 = crate::Reg<itline26::Itline26Spec>;
#[doc = "interrupt line 26 status register"]
pub mod itline26;
#[doc = "ITLINE27 (r) register accessor: interrupt line 27 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline27::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline27`]
module"]
#[doc(alias = "ITLINE27")]
pub type Itline27 = crate::Reg<itline27::Itline27Spec>;
#[doc = "interrupt line 27 status register"]
pub mod itline27;
#[doc = "ITLINE28 (r) register accessor: interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline28::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline28`]
module"]
#[doc(alias = "ITLINE28")]
pub type Itline28 = crate::Reg<itline28::Itline28Spec>;
#[doc = "interrupt line 28 status register"]
pub mod itline28;
#[doc = "ITLINE29 (r) register accessor: interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline29::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline29`]
module"]
#[doc(alias = "ITLINE29")]
pub type Itline29 = crate::Reg<itline29::Itline29Spec>;
#[doc = "interrupt line 29 status register"]
pub mod itline29;
