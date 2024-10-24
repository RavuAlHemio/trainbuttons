#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c0cr: C0cr,
    c1cr: C1cr,
    c2cr: C2cr,
    c3cr: C3cr,
    c4cr: C4cr,
    c5cr: C5cr,
    c6cr: C6cr,
    _reserved7: [u8; 0x64],
    csr: Csr,
    cfr: Cfr,
    _reserved9: [u8; 0x78],
    rg0cr: Rg0cr,
    rg1cr: Rg1cr,
    rg2cr: Rg2cr,
    rg3cr: Rg3cr,
    _reserved13: [u8; 0x30],
    rgsr: Rgsr,
    rgcfr: Rgcfr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0cr {
        &self.c0cr
    }
    #[doc = "0x04 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1cr {
        &self.c1cr
    }
    #[doc = "0x08 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2cr {
        &self.c2cr
    }
    #[doc = "0x0c - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3cr {
        &self.c3cr
    }
    #[doc = "0x10 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4cr {
        &self.c4cr
    }
    #[doc = "0x14 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5cr {
        &self.c5cr
    }
    #[doc = "0x18 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6cr {
        &self.c6cr
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn cfr(&self) -> &Cfr {
        &self.cfr
    }
    #[doc = "0x100 - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn rg0cr(&self) -> &Rg0cr {
        &self.rg0cr
    }
    #[doc = "0x104 - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn rg1cr(&self) -> &Rg1cr {
        &self.rg1cr
    }
    #[doc = "0x108 - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn rg2cr(&self) -> &Rg2cr {
        &self.rg2cr
    }
    #[doc = "0x10c - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn rg3cr(&self) -> &Rg3cr {
        &self.rg3cr
    }
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    #[inline(always)]
    pub const fn rgsr(&self) -> &Rgsr {
        &self.rgsr
    }
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    #[inline(always)]
    pub const fn rgcfr(&self) -> &Rgcfr {
        &self.rgcfr
    }
}
#[doc = "C0CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0cr`]
module"]
#[doc(alias = "C0CR")]
pub type C0cr = crate::Reg<c0cr::C0crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c0cr;
#[doc = "C1CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cr`]
module"]
#[doc(alias = "C1CR")]
pub type C1cr = crate::Reg<c1cr::C1crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c1cr;
#[doc = "C2CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cr`]
module"]
#[doc(alias = "C2CR")]
pub type C2cr = crate::Reg<c2cr::C2crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c2cr;
#[doc = "C3CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3cr`]
module"]
#[doc(alias = "C3CR")]
pub type C3cr = crate::Reg<c3cr::C3crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c3cr;
#[doc = "C4CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4cr`]
module"]
#[doc(alias = "C4CR")]
pub type C4cr = crate::Reg<c4cr::C4crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c4cr;
#[doc = "C5CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5cr`]
module"]
#[doc(alias = "C5CR")]
pub type C5cr = crate::Reg<c5cr::C5crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c5cr;
#[doc = "C6CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6cr`]
module"]
#[doc(alias = "C6CR")]
pub type C6cr = crate::Reg<c6cr::C6crSpec>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c6cr;
#[doc = "CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`]
module"]
#[doc(alias = "CFR")]
pub type Cfr = crate::Reg<cfr::CfrSpec>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
#[doc = "RG0CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg0cr`]
module"]
#[doc(alias = "RG0CR")]
pub type Rg0cr = crate::Reg<rg0cr::Rg0crSpec>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg0cr;
#[doc = "RG1CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg1cr`]
module"]
#[doc(alias = "RG1CR")]
pub type Rg1cr = crate::Reg<rg1cr::Rg1crSpec>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg1cr;
#[doc = "RG2CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg2cr`]
module"]
#[doc(alias = "RG2CR")]
pub type Rg2cr = crate::Reg<rg2cr::Rg2crSpec>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg2cr;
#[doc = "RG3CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg3cr`]
module"]
#[doc(alias = "RG3CR")]
pub type Rg3cr = crate::Reg<rg3cr::Rg3crSpec>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg3cr;
#[doc = "RGSR (r) register accessor: DMAMUX request generator interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`]
module"]
#[doc(alias = "RGSR")]
pub type Rgsr = crate::Reg<rgsr::RgsrSpec>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcfr`]
module"]
#[doc(alias = "RGCFR")]
pub type Rgcfr = crate::Reg<rgcfr::RgcfrSpec>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod rgcfr;
