#[doc = "Register `ITLINE18` reader"]
pub type R = crate::R<Itline18Spec>;
#[doc = "Field `TIM7` reader - TIM7"]
pub type Tim7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM7"]
    #[inline(always)]
    pub fn tim7(&self) -> Tim7R {
        Tim7R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 18 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline18Spec;
impl crate::RegisterSpec for Itline18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline18::R`](R) reader structure"]
impl crate::Readable for Itline18Spec {}
#[doc = "`reset()` method sets ITLINE18 to value 0"]
impl crate::Resettable for Itline18Spec {
    const RESET_VALUE: u32 = 0;
}
