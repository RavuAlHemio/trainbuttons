#[doc = "Register `RTC_SR` reader"]
pub type R = crate::R<RtcSrSpec>;
#[doc = "Field `ALRAF` reader - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm A register (RTC_ALRMAR)."]
pub type AlrafR = crate::BitReader;
#[doc = "Field `ALRBF` reader - Alarm B flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm B register (RTC_ALRMBR)."]
pub type AlrbfR = crate::BitReader;
#[doc = "Field `WUTF` reader - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
pub type WutfR = crate::BitReader;
#[doc = "Field `TSF` reader - Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSOVF` reader - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type TsovfR = crate::BitReader;
#[doc = "Field `ITSF` reader - Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs."]
pub type ItsfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm A register (RTC_ALRMAR)."]
    #[inline(always)]
    pub fn alraf(&self) -> AlrafR {
        AlrafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm B register (RTC_ALRMBR)."]
    #[inline(always)]
    pub fn alrbf(&self) -> AlrbfR {
        AlrbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
    #[inline(always)]
    pub fn wutf(&self) -> WutfR {
        WutfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn tsovf(&self) -> TsovfR {
        TsovfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs."]
    #[inline(always)]
    pub fn itsf(&self) -> ItsfR {
        ItsfR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "RTC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcSrSpec;
impl crate::RegisterSpec for RtcSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_sr::R`](R) reader structure"]
impl crate::Readable for RtcSrSpec {}
#[doc = "`reset()` method sets RTC_SR to value 0"]
impl crate::Resettable for RtcSrSpec {
    const RESET_VALUE: u32 = 0;
}
