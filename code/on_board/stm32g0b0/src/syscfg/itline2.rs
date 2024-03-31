#[doc = "Register `ITLINE2` reader"]
pub type R = crate::R<Itline2Spec>;
#[doc = "Field `TAMP` reader - TAMP"]
pub type TampR = crate::BitReader;
#[doc = "Field `RTC` reader - RTC"]
pub type RtcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP"]
    #[inline(always)]
    pub fn tamp(&self) -> TampR {
        TampR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline2Spec;
impl crate::RegisterSpec for Itline2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline2::R`](R) reader structure"]
impl crate::Readable for Itline2Spec {}
#[doc = "`reset()` method sets ITLINE2 to value 0"]
impl crate::Resettable for Itline2Spec {
    const RESET_VALUE: u32 = 0;
}
