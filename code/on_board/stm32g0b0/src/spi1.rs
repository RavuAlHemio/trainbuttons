#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_cr1: SpiCr1,
    _reserved1: [u8; 0x02],
    spi_cr2: SpiCr2,
    _reserved2: [u8; 0x02],
    spi_sr: SpiSr,
    _reserved3: [u8; 0x02],
    spi_dr: SpiDr,
    _reserved4: [u8; 0x02],
    spi_crcpr: SpiCrcpr,
    _reserved5: [u8; 0x02],
    spi_rxcrcr: SpiRxcrcr,
    _reserved6: [u8; 0x02],
    spi_txcrcr: SpiTxcrcr,
    _reserved7: [u8; 0x02],
    spi_i2scfgr: SpiI2scfgr,
    _reserved8: [u8; 0x02],
    spi_i2spr: SpiI2spr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn spi_cr1(&self) -> &SpiCr1 {
        &self.spi_cr1
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn spi_cr2(&self) -> &SpiCr2 {
        &self.spi_cr2
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn spi_sr(&self) -> &SpiSr {
        &self.spi_sr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn spi_dr(&self) -> &SpiDr {
        &self.spi_dr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn spi_crcpr(&self) -> &SpiCrcpr {
        &self.spi_crcpr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn spi_rxcrcr(&self) -> &SpiRxcrcr {
        &self.spi_rxcrcr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn spi_txcrcr(&self) -> &SpiTxcrcr {
        &self.spi_txcrcr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn spi_i2scfgr(&self) -> &SpiI2scfgr {
        &self.spi_i2scfgr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn spi_i2spr(&self) -> &SpiI2spr {
        &self.spi_i2spr
    }
}
#[doc = "SPI_CR1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cr1`]
module"]
#[doc(alias = "SPI_CR1")]
pub type SpiCr1 = crate::Reg<spi_cr1::SpiCr1Spec>;
#[doc = ""]
pub mod spi_cr1;
#[doc = "SPI_CR2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cr2`]
module"]
#[doc(alias = "SPI_CR2")]
pub type SpiCr2 = crate::Reg<spi_cr2::SpiCr2Spec>;
#[doc = ""]
pub mod spi_cr2;
#[doc = "SPI_SR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sr`]
module"]
#[doc(alias = "SPI_SR")]
pub type SpiSr = crate::Reg<spi_sr::SpiSrSpec>;
#[doc = ""]
pub mod spi_sr;
#[doc = "SPI_DR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dr`]
module"]
#[doc(alias = "SPI_DR")]
pub type SpiDr = crate::Reg<spi_dr::SpiDrSpec>;
#[doc = ""]
pub mod spi_dr;
#[doc = "SPI_CRCPR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_crcpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_crcpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_crcpr`]
module"]
#[doc(alias = "SPI_CRCPR")]
pub type SpiCrcpr = crate::Reg<spi_crcpr::SpiCrcprSpec>;
#[doc = ""]
pub mod spi_crcpr;
#[doc = "SPI_RXCRCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxcrcr`]
module"]
#[doc(alias = "SPI_RXCRCR")]
pub type SpiRxcrcr = crate::Reg<spi_rxcrcr::SpiRxcrcrSpec>;
#[doc = ""]
pub mod spi_rxcrcr;
#[doc = "SPI_TXCRCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txcrcr`]
module"]
#[doc(alias = "SPI_TXCRCR")]
pub type SpiTxcrcr = crate::Reg<spi_txcrcr::SpiTxcrcrSpec>;
#[doc = ""]
pub mod spi_txcrcr;
#[doc = "SPI_I2SCFGR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2scfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2scfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2scfgr`]
module"]
#[doc(alias = "SPI_I2SCFGR")]
pub type SpiI2scfgr = crate::Reg<spi_i2scfgr::SpiI2scfgrSpec>;
#[doc = ""]
pub mod spi_i2scfgr;
#[doc = "SPI_I2SPR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2spr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2spr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2spr`]
module"]
#[doc(alias = "SPI_I2SPR")]
pub type SpiI2spr = crate::Reg<spi_i2spr::SpiI2sprSpec>;
#[doc = ""]
pub mod spi_i2spr;
