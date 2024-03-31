#[doc = "Register `ITLINE14` reader"]
pub type R = crate::R<Itline14Spec>;
#[doc = "Field `TIM1_CC` reader - TIM1_CC"]
pub type Tim1CcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM1_CC"]
    #[inline(always)]
    pub fn tim1_cc(&self) -> Tim1CcR {
        Tim1CcR::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline14Spec;
impl crate::RegisterSpec for Itline14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline14::R`](R) reader structure"]
impl crate::Readable for Itline14Spec {}
#[doc = "`reset()` method sets ITLINE14 to value 0"]
impl crate::Resettable for Itline14Spec {
    const RESET_VALUE: u32 = 0;
}
