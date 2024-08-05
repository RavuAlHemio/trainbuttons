#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `HSION` reader - HSI16 clock enable"]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - HSI16 clock enable"]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernels"]
pub type HsikeronR = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernels"]
pub type HsikeronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIRDY` writer - HSI16 clock ready flag"]
pub type HsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDIV` reader - HSI16 clock division factor"]
pub type HsidivR = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - HSI16 clock division factor"]
pub type HsidivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSERDY` writer - HSE clock ready flag"]
pub type HserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HsebypR = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` reader - Clock security system enable"]
pub type CssonR = crate::BitReader;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - PLL enable"]
pub type PllonR = crate::BitReader;
#[doc = "Field `PLLON` writer - PLL enable"]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLRDY` writer - PLL clock ready flag"]
pub type PllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HsikeronR {
        HsikeronR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - HSI16 clock division factor"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HsidivR {
        HsidivR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CssonR {
        CssonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HsionW<CrSpec> {
        HsionW::new(self, 8)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HsikeronW<CrSpec> {
        HsikeronW::new(self, 9)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HsirdyW<CrSpec> {
        HsirdyW::new(self, 10)
    }
    #[doc = "Bits 11:13 - HSI16 clock division factor"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HsidivW<CrSpec> {
        HsidivW::new(self, 11)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HseonW<CrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HserdyW<CrSpec> {
        HserdyW::new(self, 17)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HsebypW<CrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CssonW<CrSpec> {
        CssonW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PllonW<CrSpec> {
        PllonW::new(self, 24)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdy(&mut self) -> PllrdyW<CrSpec> {
        PllrdyW::new(self, 25)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x63;
}
