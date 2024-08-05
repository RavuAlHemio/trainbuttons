#[doc = "Register `ITLINE4` reader"]
pub type R = crate::R<Itline4Spec>;
#[doc = "Field `RCC` reader - RCC"]
pub type RccR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RCC"]
    #[inline(always)]
    pub fn rcc(&self) -> RccR {
        RccR::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 4 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline4Spec;
impl crate::RegisterSpec for Itline4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline4::R`](R) reader structure"]
impl crate::Readable for Itline4Spec {}
#[doc = "`reset()` method sets ITLINE4 to value 0"]
impl crate::Resettable for Itline4Spec {
    const RESET_VALUE: u32 = 0;
}
