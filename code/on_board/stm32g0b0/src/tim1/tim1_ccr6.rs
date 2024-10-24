#[doc = "Register `TIM1_CCR6` reader"]
pub type R = crate::R<Tim1Ccr6Spec>;
#[doc = "Register `TIM1_CCR6` writer"]
pub type W = crate::W<Tim1Ccr6Spec>;
#[doc = "Field `CCR6` reader - Capture/Compare value"]
pub type Ccr6R = crate::FieldReader<u16>;
#[doc = "Field `CCR6` writer - Capture/Compare value"]
pub type Ccr6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr6(&self) -> Ccr6R {
        Ccr6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr6(&mut self) -> Ccr6W<Tim1Ccr6Spec> {
        Ccr6W::new(self, 0)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr6Spec;
impl crate::RegisterSpec for Tim1Ccr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccr6::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr6Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr6::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR6 to value 0"]
impl crate::Resettable for Tim1Ccr6Spec {
    const RESET_VALUE: u32 = 0;
}
