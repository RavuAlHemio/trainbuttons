#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA1 reset"]
pub type Dma2rstR = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA1 reset"]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - FLITF reset"]
pub type FlashrstR = crate::BitReader;
#[doc = "Field `FLASHRST` writer - FLITF reset"]
pub type FlashrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FLITF reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FlashrstR {
        FlashrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> Dma1rstW<AhbrstrSpec> {
        Dma1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> Dma2rstW<AhbrstrSpec> {
        Dma2rstW::new(self, 1)
    }
    #[doc = "Bit 8 - FLITF reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FlashrstW<AhbrstrSpec> {
        FlashrstW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<AhbrstrSpec> {
        CrcrstW::new(self, 12)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstrSpec;
impl crate::RegisterSpec for AhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AhbrstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AhbrstrSpec {
    const RESET_VALUE: u32 = 0;
}
