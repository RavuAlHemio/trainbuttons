#[doc = "Register `RTC_SHIFTR` writer"]
pub type W = crate::W<RtcShiftrSpec>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF = 1 to be sure that the shadow registers have been updated with the shifted time."]
pub type SubfsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Add1s {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Add one second to the clock/calendar"]
    B0x1 = 1,
}
impl From<Add1s> for bool {
    #[inline(always)]
    fn from(variant: Add1s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1S` writer - Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
pub type Add1sW<'a, REG> = crate::BitWriter<'a, REG, Add1s>;
impl<'a, REG> Add1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Add1s::B0x0)
    }
    #[doc = "Add one second to the clock/calendar"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Add1s::B0x1)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF = 1 to be sure that the shadow registers have been updated with the shifted time."]
    #[inline(always)]
    #[must_use]
    pub fn subfs(&mut self) -> SubfsW<RtcShiftrSpec> {
        SubfsW::new(self, 0)
    }
    #[doc = "Bit 31 - Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> Add1sW<RtcShiftrSpec> {
        Add1sW::new(self, 31)
    }
}
#[doc = "RTC shift control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcShiftrSpec;
impl crate::RegisterSpec for RtcShiftrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_shiftr::W`](W) writer structure"]
impl crate::Writable for RtcShiftrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_SHIFTR to value 0"]
impl crate::Resettable for RtcShiftrSpec {
    const RESET_VALUE: u32 = 0;
}
