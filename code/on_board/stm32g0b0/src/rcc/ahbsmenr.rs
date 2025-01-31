#[doc = "Register `AHBSMENR` reader"]
pub type R = crate::R<AhbsmenrSpec>;
#[doc = "Register `AHBSMENR` writer"]
pub type W = crate::W<AhbsmenrSpec>;
#[doc = "Field `DMA1SMEN` reader - DMA1 clock enable during Sleep mode"]
pub type Dma1smenR = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - DMA1 clock enable during Sleep mode"]
pub type Dma1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - DMA2 clock enable during Sleep mode"]
pub type Dma2smenR = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - DMA2 clock enable during Sleep mode"]
pub type Dma2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode"]
pub type FlashsmenR = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode"]
pub type FlashsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode"]
pub type SramsmenR = crate::BitReader;
#[doc = "Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode"]
pub type SramsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRC clock enable during Sleep mode"]
pub type CrcsmenR = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRC clock enable during Sleep mode"]
pub type CrcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1smen(&self) -> Dma1smenR {
        Dma1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2smen(&self) -> Dma2smenR {
        Dma2smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FlashsmenR {
        FlashsmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SramsmenR {
        SramsmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CrcsmenR {
        CrcsmenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> Dma1smenW<AhbsmenrSpec> {
        Dma1smenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> Dma2smenW<AhbsmenrSpec> {
        Dma2smenW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FlashsmenW<AhbsmenrSpec> {
        FlashsmenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SramsmenW<AhbsmenrSpec> {
        SramsmenW::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CrcsmenW<AhbsmenrSpec> {
        CrcsmenW::new(self, 12)
    }
}
#[doc = "AHB peripheral clock enable in Sleep mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbsmenrSpec;
impl crate::RegisterSpec for AhbsmenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbsmenr::R`](R) reader structure"]
impl crate::Readable for AhbsmenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure"]
impl crate::Writable for AhbsmenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBSMENR to value 0x0005_1303"]
impl crate::Resettable for AhbsmenrSpec {
    const RESET_VALUE: u32 = 0x0005_1303;
}
