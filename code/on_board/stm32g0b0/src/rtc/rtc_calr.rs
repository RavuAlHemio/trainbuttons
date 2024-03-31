#[doc = "Register `RTC_CALR` reader"]
pub type R = crate::R<RtcCalrSpec>;
#[doc = "Register `RTC_CALR` writer"]
pub type W = crate::W<RtcCalrSpec>;
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
pub type CalmR = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
pub type CalmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
pub type Calw16R = crate::BitReader;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
pub type Calw16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
pub type Calw8R = crate::BitReader;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
pub type Calw8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Increase frequency of RTC by 488\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calp {
    #[doc = "0: No RTCCLK pulses are added."]
    B0x0 = 0,
    #[doc = "1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5ppm)."]
    B0x1 = 1,
}
impl From<Calp> for bool {
    #[inline(always)]
    fn from(variant: Calp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488"]
pub type CalpR = crate::BitReader<Calp>;
impl CalpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calp {
        match self.bits {
            false => Calp::B0x0,
            true => Calp::B0x1,
        }
    }
    #[doc = "No RTCCLK pulses are added."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Calp::B0x0
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5ppm)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Calp::B0x1
    }
}
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488"]
pub type CalpW<'a, REG> = crate::BitWriter<'a, REG, Calp>;
impl<'a, REG> CalpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RTCCLK pulses are added."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Calp::B0x0)
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5ppm)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Calp::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
    #[inline(always)]
    pub fn calm(&self) -> CalmR {
        CalmR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
    #[inline(always)]
    pub fn calw16(&self) -> Calw16R {
        Calw16R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
    #[inline(always)]
    pub fn calw8(&self) -> Calw8R {
        Calw8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488"]
    #[inline(always)]
    pub fn calp(&self) -> CalpR {
        CalpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CalmW<RtcCalrSpec> {
        CalmW::new(self, 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> Calw16W<RtcCalrSpec> {
        Calw16W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> Calw8W<RtcCalrSpec> {
        Calw8W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488"]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CalpW<RtcCalrSpec> {
        CalpW::new(self, 15)
    }
}
#[doc = "RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCalrSpec;
impl crate::RegisterSpec for RtcCalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_calr::R`](R) reader structure"]
impl crate::Readable for RtcCalrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_calr::W`](W) writer structure"]
impl crate::Writable for RtcCalrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_CALR to value 0"]
impl crate::Resettable for RtcCalrSpec {
    const RESET_VALUE: u32 = 0;
}
