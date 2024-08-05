#[doc = "Register `ITLINE17` reader"]
pub type R = crate::R<Itline17Spec>;
#[doc = "Field `TIM6` reader - TIM6"]
pub type Tim6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> Tim6R {
        Tim6R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 17 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline17Spec;
impl crate::RegisterSpec for Itline17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline17::R`](R) reader structure"]
impl crate::Readable for Itline17Spec {}
#[doc = "`reset()` method sets ITLINE17 to value 0"]
impl crate::Resettable for Itline17Spec {
    const RESET_VALUE: u32 = 0;
}
