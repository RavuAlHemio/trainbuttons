#[doc = "Register `ITLINE20` reader"]
pub type R = crate::R<Itline20Spec>;
#[doc = "Field `TIM15` reader - TIM15"]
pub type Tim15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM15"]
    #[inline(always)]
    pub fn tim15(&self) -> Tim15R {
        Tim15R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 20 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline20::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline20Spec;
impl crate::RegisterSpec for Itline20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline20::R`](R) reader structure"]
impl crate::Readable for Itline20Spec {}
#[doc = "`reset()` method sets ITLINE20 to value 0"]
impl crate::Resettable for Itline20Spec {
    const RESET_VALUE: u32 = 0;
}
