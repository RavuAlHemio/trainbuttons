#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<Ccmr1OutputSpec>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<Ccmr1OutputSpec>;
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc1s {
    #[doc = "0: CC1 channel is configured as output."]
    B0x0 = 0,
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1."]
    B0x1 = 1,
}
impl From<Cc1s> for u8 {
    #[inline(always)]
    fn from(variant: Cc1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc1s {
    type Ux = u8;
}
impl crate::IsEnum for Cc1s {}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type Cc1sR = crate::FieldReader<Cc1s>;
impl Cc1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cc1s> {
        match self.bits {
            0 => Some(Cc1s::B0x0),
            1 => Some(Cc1s::B0x1),
            _ => None,
        }
    }
    #[doc = "CC1 channel is configured as output."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1s::B0x0
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1s::B0x1
    }
}
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc1s>;
impl<'a, REG> Cc1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC1 channel is configured as output."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x0)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x1)
    }
}
#[doc = "Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oc1fe {
    #[doc = "0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    B0x0 = 0,
    #[doc = "1: An active edge on the trigger input acts like a compare match on CC1 output. OC is then set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    B0x1 = 1,
}
impl From<Oc1fe> for bool {
    #[inline(always)]
    fn from(variant: Oc1fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type Oc1feR = crate::BitReader<Oc1fe>;
impl Oc1feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc1fe {
        match self.bits {
            false => Oc1fe::B0x0,
            true => Oc1fe::B0x1,
        }
    }
    #[doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oc1fe::B0x0
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. OC is then set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1fe::B0x1
    }
}
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG, Oc1fe>;
impl<'a, REG> Oc1feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1fe::B0x0)
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. OC is then set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1fe::B0x1)
    }
}
#[doc = "Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oc1pe {
    #[doc = "0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    B0x0 = 0,
    #[doc = "1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    B0x1 = 1,
}
impl From<Oc1pe> for bool {
    #[inline(always)]
    fn from(variant: Oc1pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type Oc1peR = crate::BitReader<Oc1pe>;
impl Oc1peR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc1pe {
        match self.bits {
            false => Oc1pe::B0x0,
            true => Oc1pe::B0x1,
        }
    }
    #[doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oc1pe::B0x0
    }
    #[doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1pe::B0x1
    }
}
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG, Oc1pe>;
impl<'a, REG> Oc1peW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1pe::B0x0)
    }
    #[doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1pe::B0x1)
    }
}
#[doc = "Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oc1m1 {
    #[doc = "0: Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    B0x0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B0x1 = 1,
    #[doc = "2: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B0x2 = 2,
    #[doc = "3: Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1."]
    B0x3 = 3,
    #[doc = "4: Force inactive level - OC1REF is forced low."]
    B0x4 = 4,
    #[doc = "5: Force active level - OC1REF is forced high."]
    B0x5 = 5,
    #[doc = "6: PWM mode 1 - Channel 1 is active as long as TIMx_CNT TIMx_CCR1 else inactive."]
    B0x6 = 6,
    #[doc = "7: PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT TIMx_CCR1 else active"]
    B0x7 = 7,
}
impl From<Oc1m1> for u8 {
    #[inline(always)]
    fn from(variant: Oc1m1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oc1m1 {
    type Ux = u8;
}
impl crate::IsEnum for Oc1m1 {}
#[doc = "Field `OC1M1` reader - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m1R = crate::FieldReader<Oc1m1>;
impl Oc1m1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc1m1 {
        match self.bits {
            0 => Oc1m1::B0x0,
            1 => Oc1m1::B0x1,
            2 => Oc1m1::B0x2,
            3 => Oc1m1::B0x3,
            4 => Oc1m1::B0x4,
            5 => Oc1m1::B0x5,
            6 => Oc1m1::B0x6,
            7 => Oc1m1::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oc1m1::B0x0
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1m1::B0x1
    }
    #[doc = "Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Oc1m1::B0x2
    }
    #[doc = "Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Oc1m1::B0x3
    }
    #[doc = "Force inactive level - OC1REF is forced low."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Oc1m1::B0x4
    }
    #[doc = "Force active level - OC1REF is forced high."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Oc1m1::B0x5
    }
    #[doc = "PWM mode 1 - Channel 1 is active as long as TIMx_CNT TIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Oc1m1::B0x6
    }
    #[doc = "PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT TIMx_CCR1 else active"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Oc1m1::B0x7
    }
}
#[doc = "Field `OC1M1` writer - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Oc1m1, crate::Safe>;
impl<'a, REG> Oc1m1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x0)
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x1)
    }
    #[doc = "Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x2)
    }
    #[doc = "Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x3)
    }
    #[doc = "Force inactive level - OC1REF is forced low."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x4)
    }
    #[doc = "Force active level - OC1REF is forced high."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x5)
    }
    #[doc = "PWM mode 1 - Channel 1 is active as long as TIMx_CNT TIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x6)
    }
    #[doc = "PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT TIMx_CCR1 else active"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x7)
    }
}
#[doc = "Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oc1m2 {
    #[doc = "0: Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    B0x0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B0x1 = 1,
}
impl From<Oc1m2> for bool {
    #[inline(always)]
    fn from(variant: Oc1m2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1M2` reader - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m2R = crate::BitReader<Oc1m2>;
impl Oc1m2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc1m2 {
        match self.bits {
            false => Oc1m2::B0x0,
            true => Oc1m2::B0x1,
        }
    }
    #[doc = "Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oc1m2::B0x0
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1m2::B0x1
    }
}
#[doc = "Field `OC1M2` writer - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m2W<'a, REG> = crate::BitWriter<'a, REG, Oc1m2>;
impl<'a, REG> Oc1m2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m2::B0x0)
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m2::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&self) -> Oc1m1R {
        Oc1m1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&self) -> Oc1m2R {
        Oc1m2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> Cc1sW<Ccmr1OutputSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> Oc1feW<Ccmr1OutputSpec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> Oc1peW<Ccmr1OutputSpec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> Oc1m1W<Ccmr1OutputSpec> {
        Oc1m1W::new(self, 4)
    }
    #[doc = "Bit 16 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m2(&mut self) -> Oc1m2W<Ccmr1OutputSpec> {
        Oc1m2W::new(self, 16)
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1OutputSpec;
impl crate::RegisterSpec for Ccmr1OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for Ccmr1OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for Ccmr1OutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for Ccmr1OutputSpec {
    const RESET_VALUE: u32 = 0;
}
