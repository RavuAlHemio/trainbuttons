#[doc = "Register `TIM1_CCR3` reader"]
pub type R = crate::R<Tim1Ccr3Spec>;
#[doc = "Register `TIM1_CCR3` writer"]
pub type W = crate::W<Tim1Ccr3Spec>;
#[doc = "Field `CCR3` reader - Capture/Compare value"]
pub type Ccr3R = crate::FieldReader<u16>;
#[doc = "Field `CCR3` writer - Capture/Compare value"]
pub type Ccr3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3(&self) -> Ccr3R {
        Ccr3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> Ccr3W<Tim1Ccr3Spec> {
        Ccr3W::new(self, 0)
    }
}
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr3Spec;
impl crate::RegisterSpec for Tim1Ccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccr3::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr3::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR3 to value 0"]
impl crate::Resettable for Tim1Ccr3Spec {
    const RESET_VALUE: u32 = 0;
}
