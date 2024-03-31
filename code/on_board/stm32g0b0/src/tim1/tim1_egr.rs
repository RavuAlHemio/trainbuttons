#[doc = "Register `TIM1_EGR` writer"]
pub type W = crate::W<Tim1EgrSpec>;
#[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ug {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: Reinitialize the counter and generates an update of the registers. The prescaler internal counter is also cleared (the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting)."]
    B0x1 = 1,
}
impl From<Ug> for bool {
    #[inline(always)]
    fn from(variant: Ug) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware."]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG, Ug>;
impl<'a, REG> UgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ug::B0x0)
    }
    #[doc = "Reinitialize the counter and generates an update of the registers. The prescaler internal counter is also cleared (the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ug::B0x1)
    }
}
#[doc = "Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1g {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: A capture/compare event is generated on channel 1:"]
    B0x1 = 1,
}
impl From<Cc1g> for bool {
    #[inline(always)]
    fn from(variant: Cc1g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1G` writer - Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG, Cc1g>;
impl<'a, REG> Cc1gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1g::B0x0)
    }
    #[doc = "A capture/compare event is generated on channel 1:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1g::B0x1)
    }
}
#[doc = "Field `CC2G` writer - Capture/Compare 2 generation Refer to CC1G description"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - Capture/Compare 3 generation Refer to CC1G description"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - Capture/Compare 4 generation Refer to CC1G description"]
pub type Cc4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comg {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated."]
    B0x1 = 1,
}
impl From<Comg> for bool {
    #[inline(always)]
    fn from(variant: Comg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMG` writer - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output."]
pub type ComgW<'a, REG> = crate::BitWriter<'a, REG, Comg>;
impl<'a, REG> ComgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Comg::B0x0)
    }
    #[doc = "When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Comg::B0x1)
    }
}
#[doc = "Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tg {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    B0x1 = 1,
}
impl From<Tg> for bool {
    #[inline(always)]
    fn from(variant: Tg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG, Tg>;
impl<'a, REG> TgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tg::B0x0)
    }
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tg::B0x1)
    }
}
#[doc = "Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bg {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    B0x1 = 1,
}
impl From<Bg> for bool {
    #[inline(always)]
    fn from(variant: Bg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type BgW<'a, REG> = crate::BitWriter<'a, REG, Bg>;
impl<'a, REG> BgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bg::B0x0)
    }
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bg::B0x1)
    }
}
#[doc = "Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2g {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled."]
    B0x1 = 1,
}
impl From<B2g> for bool {
    #[inline(always)]
    fn from(variant: B2g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type B2gW<'a, REG> = crate::BitWriter<'a, REG, B2g>;
impl<'a, REG> B2gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(B2g::B0x0)
    }
    #[doc = "A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(B2g::B0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UgW<Tim1EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> Cc1gW<Tim1EgrSpec> {
        Cc1gW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 generation Refer to CC1G description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> Cc2gW<Tim1EgrSpec> {
        Cc2gW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 generation Refer to CC1G description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> Cc3gW<Tim1EgrSpec> {
        Cc3gW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 generation Refer to CC1G description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> Cc4gW<Tim1EgrSpec> {
        Cc4gW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> ComgW<Tim1EgrSpec> {
        ComgW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TgW<Tim1EgrSpec> {
        TgW::new(self, 6)
    }
    #[doc = "Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BgW<Tim1EgrSpec> {
        BgW::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn b2g(&mut self) -> B2gW<Tim1EgrSpec> {
        B2gW::new(self, 8)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1EgrSpec;
impl crate::RegisterSpec for Tim1EgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tim1_egr::W`](W) writer structure"]
impl crate::Writable for Tim1EgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_EGR to value 0"]
impl crate::Resettable for Tim1EgrSpec {
    const RESET_VALUE: u32 = 0;
}
