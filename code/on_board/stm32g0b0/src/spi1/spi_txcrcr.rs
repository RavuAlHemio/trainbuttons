#[doc = "Register `SPI_TXCRCR` reader"]
pub type R = crate::R<SpiTxcrcrSpec>;
#[doc = "Field `TXCRC` reader - Tx CRC register When CRC calculation is enabled, the TXCRC\\[7:0\\]
bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPIx_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPIx_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPIx_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPIx_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value. These bits are not used in I2S mode."]
pub type TxcrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register When CRC calculation is enabled, the TXCRC\\[7:0\\]
bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPIx_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPIx_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPIx_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPIx_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value. These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn txcrc(&self) -> TxcrcR {
        TxcrcR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_txcrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiTxcrcrSpec;
impl crate::RegisterSpec for SpiTxcrcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_txcrcr::R`](R) reader structure"]
impl crate::Readable for SpiTxcrcrSpec {}
#[doc = "`reset()` method sets SPI_TXCRCR to value 0"]
impl crate::Resettable for SpiTxcrcrSpec {
    const RESET_VALUE: u16 = 0;
}
