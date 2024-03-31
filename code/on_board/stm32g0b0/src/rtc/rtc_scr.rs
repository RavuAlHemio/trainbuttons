#[doc = "Register `RTC_SCR` writer"]
pub type W = crate::W<RtcScrSpec>;
#[doc = "Field `CALRAF` writer - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register."]
pub type CalrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` writer - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register."]
pub type CalrbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register."]
pub type CwutfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` writer - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF."]
pub type CtsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type CtsovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register."]
pub type CitsfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CalrafW<RtcScrSpec> {
        CalrafW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CalrbfW<RtcScrSpec> {
        CalrbfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CwutfW<RtcScrSpec> {
        CwutfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF."]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CtsfW<RtcScrSpec> {
        CtsfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CtsovfW<RtcScrSpec> {
        CtsovfW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CitsfW<RtcScrSpec> {
        CitsfW::new(self, 5)
    }
}
#[doc = "RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcScrSpec;
impl crate::RegisterSpec for RtcScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_scr::W`](W) writer structure"]
impl crate::Writable for RtcScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_SCR to value 0"]
impl crate::Resettable for RtcScrSpec {
    const RESET_VALUE: u32 = 0;
}
