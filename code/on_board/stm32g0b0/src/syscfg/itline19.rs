#[doc = "Register `ITLINE19` reader"]
pub type R = crate::R<Itline19Spec>;
#[doc = "Field `TIM14` reader - TIM14"]
pub type Tim14R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM14"]
    #[inline(always)]
    pub fn tim14(&self) -> Tim14R {
        Tim14R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 19 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline19Spec;
impl crate::RegisterSpec for Itline19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline19::R`](R) reader structure"]
impl crate::Readable for Itline19Spec {}
#[doc = "`reset()` method sets ITLINE19 to value 0"]
impl crate::Resettable for Itline19Spec {
    const RESET_VALUE: u32 = 0;
}
