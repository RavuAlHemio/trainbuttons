#[doc = "Register `ITLINE6` reader"]
pub type R = crate::R<Itline6Spec>;
#[doc = "Field `EXTI2` reader - EXTI2"]
pub type Exti2R = crate::BitReader;
#[doc = "Field `EXTI3` reader - EXTI3"]
pub type Exti3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI2"]
    #[inline(always)]
    pub fn exti2(&self) -> Exti2R {
        Exti2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI3"]
    #[inline(always)]
    pub fn exti3(&self) -> Exti3R {
        Exti3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline6Spec;
impl crate::RegisterSpec for Itline6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline6::R`](R) reader structure"]
impl crate::Readable for Itline6Spec {}
#[doc = "`reset()` method sets ITLINE6 to value 0"]
impl crate::Resettable for Itline6Spec {
    const RESET_VALUE: u32 = 0;
}
