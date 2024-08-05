#[doc = "Register `PLLSYSCFGR` reader"]
pub type R = crate::R<PllsyscfgrSpec>;
#[doc = "Register `PLLSYSCFGR` writer"]
pub type W = crate::W<PllsyscfgrSpec>;
#[doc = "Field `PLLSRC` reader - PLL input clock source"]
pub type PllsrcR = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - PLL input clock source"]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLM` reader - Division factor M of the PLL input clock divider"]
pub type PllmR = crate::FieldReader;
#[doc = "Field `PLLM` writer - Division factor M of the PLL input clock divider"]
pub type PllmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLN` reader - PLL frequency multiplication factor N"]
pub type PllnR = crate::FieldReader;
#[doc = "Field `PLLN` writer - PLL frequency multiplication factor N"]
pub type PllnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLPEN` reader - PLLPCLK clock output enable"]
pub type PllpenR = crate::BitReader;
#[doc = "Field `PLLPEN` writer - PLLPCLK clock output enable"]
pub type PllpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output"]
pub type PllpR = crate::FieldReader;
#[doc = "Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output"]
pub type PllpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLLQEN` reader - PLLQCLK clock output enable"]
pub type PllqenR = crate::BitReader;
#[doc = "Field `PLLQEN` writer - PLLQCLK clock output enable"]
pub type PllqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output"]
pub type PllqR = crate::FieldReader;
#[doc = "Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output"]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLREN` reader - PLLRCLK clock output enable"]
pub type PllrenR = crate::BitReader;
#[doc = "Field `PLLREN` writer - PLLRCLK clock output enable"]
pub type PllrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output"]
pub type PllrR = crate::FieldReader;
#[doc = "Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output"]
pub type PllrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - PLL input clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider"]
    #[inline(always)]
    pub fn pllm(&self) -> PllmR {
        PllmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - PLL frequency multiplication factor N"]
    #[inline(always)]
    pub fn plln(&self) -> PllnR {
        PllnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PllpenR {
        PllpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output"]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PllqenR {
        PllqenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output"]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PllrenR {
        PllrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output"]
    #[inline(always)]
    pub fn pllr(&self) -> PllrR {
        PllrR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL input clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PllsrcW<PllsyscfgrSpec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PllmW<PllsyscfgrSpec> {
        PllmW::new(self, 4)
    }
    #[doc = "Bits 8:15 - PLL frequency multiplication factor N"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PllnW<PllsyscfgrSpec> {
        PllnW::new(self, 8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PllpenW<PllsyscfgrSpec> {
        PllpenW::new(self, 16)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PllpW<PllsyscfgrSpec> {
        PllpW::new(self, 17)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PllqenW<PllsyscfgrSpec> {
        PllqenW::new(self, 24)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PllqW<PllsyscfgrSpec> {
        PllqW::new(self, 25)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PllrenW<PllsyscfgrSpec> {
        PllrenW::new(self, 28)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PllrW<PllsyscfgrSpec> {
        PllrW::new(self, 29)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsyscfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsyscfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllsyscfgrSpec;
impl crate::RegisterSpec for PllsyscfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsyscfgr::R`](R) reader structure"]
impl crate::Readable for PllsyscfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllsyscfgr::W`](W) writer structure"]
impl crate::Writable for PllsyscfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSYSCFGR to value 0x1000"]
impl crate::Resettable for PllsyscfgrSpec {
    const RESET_VALUE: u32 = 0x1000;
}
