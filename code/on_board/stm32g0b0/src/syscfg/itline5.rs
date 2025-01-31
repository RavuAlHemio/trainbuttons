#[doc = "Register `ITLINE5` reader"]
pub type R = crate::R<Itline5Spec>;
#[doc = "Field `EXTI0` reader - EXTI0"]
pub type Exti0R = crate::BitReader;
#[doc = "Field `EXTI1` reader - EXTI1"]
pub type Exti1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&self) -> Exti0R {
        Exti0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&self) -> Exti1R {
        Exti1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 5 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline5Spec;
impl crate::RegisterSpec for Itline5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline5::R`](R) reader structure"]
impl crate::Readable for Itline5Spec {}
#[doc = "`reset()` method sets ITLINE5 to value 0"]
impl crate::Resettable for Itline5Spec {
    const RESET_VALUE: u32 = 0;
}
