#[doc = "Register `RTC_PRER` reader"]
pub type R = crate::R<RtcPrerSpec>;
#[doc = "Register `RTC_PRER` writer"]
pub type W = crate::W<RtcPrerSpec>;
#[doc = "Field `PREDIV_S` reader - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
pub type PredivSR = crate::FieldReader<u16>;
#[doc = "Field `PREDIV_S` writer - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
pub type PredivSW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `PREDIV_A` reader - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
pub type PredivAR = crate::FieldReader;
#[doc = "Field `PREDIV_A` writer - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
pub type PredivAW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:14 - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
    #[inline(always)]
    pub fn prediv_s(&self) -> PredivSR {
        PredivSR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
    #[inline(always)]
    pub fn prediv_a(&self) -> PredivAR {
        PredivAR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
    #[inline(always)]
    #[must_use]
    pub fn prediv_s(&mut self) -> PredivSW<RtcPrerSpec> {
        PredivSW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
    #[inline(always)]
    #[must_use]
    pub fn prediv_a(&mut self) -> PredivAW<RtcPrerSpec> {
        PredivAW::new(self, 16)
    }
}
#[doc = "RTC prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_prer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_prer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcPrerSpec;
impl crate::RegisterSpec for RtcPrerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_prer::R`](R) reader structure"]
impl crate::Readable for RtcPrerSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_prer::W`](W) writer structure"]
impl crate::Writable for RtcPrerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_PRER to value 0x007f_00ff"]
impl crate::Resettable for RtcPrerSpec {
    const RESET_VALUE: u32 = 0x007f_00ff;
}
