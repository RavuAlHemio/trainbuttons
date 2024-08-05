#[doc = "Register `RTC_MISR` reader"]
pub type R = crate::R<RtcMisrSpec>;
#[doc = "Field `ALRAMF` reader - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs."]
pub type AlramfR = crate::BitReader;
#[doc = "Field `ALRBMF` reader - Alarm B masked flag This flag is set by hardware when the alarm B interrupt occurs."]
pub type AlrbmfR = crate::BitReader;
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag This flag is set by hardware when the wakeup timer interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
pub type WutmfR = crate::BitReader;
#[doc = "Field `TSMF` reader - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
pub type TsmfR = crate::BitReader;
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type TsovmfR = crate::BitReader;
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestampinterrupt is raised."]
pub type ItsmfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs."]
    #[inline(always)]
    pub fn alramf(&self) -> AlramfR {
        AlramfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag This flag is set by hardware when the alarm B interrupt occurs."]
    #[inline(always)]
    pub fn alrbmf(&self) -> AlrbmfR {
        AlrbmfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag This flag is set by hardware when the wakeup timer interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
    #[inline(always)]
    pub fn wutmf(&self) -> WutmfR {
        WutmfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
    #[inline(always)]
    pub fn tsmf(&self) -> TsmfR {
        TsmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn tsovmf(&self) -> TsovmfR {
        TsovmfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestampinterrupt is raised."]
    #[inline(always)]
    pub fn itsmf(&self) -> ItsmfR {
        ItsmfR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "RTC masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcMisrSpec;
impl crate::RegisterSpec for RtcMisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_misr::R`](R) reader structure"]
impl crate::Readable for RtcMisrSpec {}
#[doc = "`reset()` method sets RTC_MISR to value 0"]
impl crate::Resettable for RtcMisrSpec {
    const RESET_VALUE: u32 = 0;
}
