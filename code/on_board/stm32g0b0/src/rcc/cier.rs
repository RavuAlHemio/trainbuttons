#[doc = "Register `CIER` reader"]
pub type R = crate::R<CierSpec>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CierSpec>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSYSRDYIE` reader - PLL ready interrupt enable"]
pub type PllsysrdyieR = crate::BitReader;
#[doc = "Field `PLLSYSRDYIE` writer - PLL ready interrupt enable"]
pub type PllsysrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllsysrdyie(&self) -> PllsysrdyieR {
        PllsysrdyieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LsirdyieW<CierSpec> {
        LsirdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LserdyieW<CierSpec> {
        LserdyieW::new(self, 1)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HsirdyieW<CierSpec> {
        HsirdyieW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HserdyieW<CierSpec> {
        HserdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsysrdyie(&mut self) -> PllsysrdyieW<CierSpec> {
        PllsysrdyieW::new(self, 5)
    }
}
#[doc = "Clock interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CierSpec;
impl crate::RegisterSpec for CierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CierSpec {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CierSpec {
    const RESET_VALUE: u32 = 0;
}
