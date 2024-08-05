#[doc = "Register `TIM1_CCR2` reader"]
pub type R = crate::R<Tim1Ccr2Spec>;
#[doc = "Register `TIM1_CCR2` writer"]
pub type W = crate::W<Tim1Ccr2Spec>;
#[doc = "Field `CCR2` reader - Capture/Compare 2 value"]
pub type Ccr2R = crate::FieldReader<u16>;
#[doc = "Field `CCR2` writer - Capture/Compare 2 value"]
pub type Ccr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ccr2(&self) -> Ccr2R {
        Ccr2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> Ccr2W<Tim1Ccr2Spec> {
        Ccr2W::new(self, 0)
    }
}
#[doc = "capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr2Spec;
impl crate::RegisterSpec for Tim1Ccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccr2::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr2::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR2 to value 0"]
impl crate::Resettable for Tim1Ccr2Spec {
    const RESET_VALUE: u32 = 0;
}
