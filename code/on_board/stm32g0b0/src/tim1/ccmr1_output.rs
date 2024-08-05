#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<Ccmr1OutputSpec>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<Ccmr1OutputSpec>;
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc1s {
    #[doc = "0: CC1 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1"]
    B0x1 = 1,
    #[doc = "2: CC1 channel is configured as input, IC1 is mapped on TI2"]
    B0x2 = 2,
    #[doc = "3: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
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
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
pub type Cc1sR = crate::FieldReader<Cc1s>;
impl Cc1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1s {
        match self.bits {
            0 => Cc1s::B0x0,
            1 => Cc1s::B0x1,
            2 => Cc1s::B0x2,
            3 => Cc1s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1s::B0x0
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1s::B0x1
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc1s::B0x2
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc1s::B0x3
    }
}
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc1s, crate::Safe>;
impl<'a, REG> Cc1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x0)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x3)
    }
}
#[doc = "Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oc1fe {
    #[doc = "0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    B0x0 = 0,
    #[doc = "1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    B0x1 = 1,
}
impl From<Oc1fe> for bool {
    #[inline(always)]
    fn from(variant: Oc1fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1FE` reader - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
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
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1fe::B0x1
    }
}
#[doc = "Field `OC1FE` writer - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
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
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1fe::B0x1)
    }
}
#[doc = "Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.\n\nValue on reset: 0"]
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
#[doc = "Field `OC1PE` reader - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
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
#[doc = "Field `OC1PE` writer - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
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
#[doc = "Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oc1m1 {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    B0x0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B0x1 = 1,
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
#[doc = "Field `OC1M1` reader - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m1R = crate::FieldReader<Oc1m1>;
impl Oc1m1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Oc1m1> {
        match self.bits {
            0 => Some(Oc1m1::B0x0),
            1 => Some(Oc1m1::B0x1),
            _ => None,
        }
    }
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oc1m1::B0x0
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1m1::B0x1
    }
}
#[doc = "Field `OC1M1` writer - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Oc1m1>;
impl<'a, REG> Oc1m1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x0)
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m1::B0x1)
    }
}
#[doc = "Output Compare 1 clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oc1ce {
    #[doc = "0: OC1Ref is not affected by the ocref_clr_int signal"]
    B0x0 = 0,
    #[doc = "1: OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)"]
    B0x1 = 1,
}
impl From<Oc1ce> for bool {
    #[inline(always)]
    fn from(variant: Oc1ce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1CE` reader - Output Compare 1 clear enable"]
pub type Oc1ceR = crate::BitReader<Oc1ce>;
impl Oc1ceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc1ce {
        match self.bits {
            false => Oc1ce::B0x0,
            true => Oc1ce::B0x1,
        }
    }
    #[doc = "OC1Ref is not affected by the ocref_clr_int signal"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oc1ce::B0x0
    }
    #[doc = "OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oc1ce::B0x1
    }
}
#[doc = "Field `OC1CE` writer - Output Compare 1 clear enable"]
pub type Oc1ceW<'a, REG> = crate::BitWriter<'a, REG, Oc1ce>;
impl<'a, REG> Oc1ceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1Ref is not affected by the ocref_clr_int signal"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1ce::B0x0)
    }
    #[doc = "OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1ce::B0x1)
    }
}
#[doc = "Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0' in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2s {
    #[doc = "0: CC2 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC2 channel is configured as input, IC2 is mapped on TI2"]
    B0x1 = 1,
    #[doc = "2: CC2 channel is configured as input, IC2 is mapped on TI1"]
    B0x2 = 2,
    #[doc = "3: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc2s> for u8 {
    #[inline(always)]
    fn from(variant: Cc2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2s {
    type Ux = u8;
}
impl crate::IsEnum for Cc2s {}
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0' in TIMx_CCER)."]
pub type Cc2sR = crate::FieldReader<Cc2s>;
impl Cc2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2s {
        match self.bits {
            0 => Cc2s::B0x0,
            1 => Cc2s::B0x1,
            2 => Cc2s::B0x2,
            3 => Cc2s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2s::B0x0
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2s::B0x1
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc2s::B0x2
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc2s::B0x3
    }
}
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0' in TIMx_CCER)."]
pub type Cc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2s, crate::Safe>;
impl<'a, REG> Cc2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x0)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x3)
    }
}
#[doc = "Field `OC2FE` reader - Output Compare 2 fast enable Refer to OC1FE description."]
pub type Oc2feR = crate::BitReader;
#[doc = "Field `OC2FE` writer - Output Compare 2 fast enable Refer to OC1FE description."]
pub type Oc2feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Output Compare 2 preload enable Refer to OC1PE description."]
pub type Oc2peR = crate::BitReader;
#[doc = "Field `OC2PE` writer - Output Compare 2 preload enable Refer to OC1PE description."]
pub type Oc2peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M1` reader - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc2m1R = crate::FieldReader;
#[doc = "Field `OC2M1` writer - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc2m1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CE` reader - Output Compare 2 clear enable Refer to OC1CE description."]
pub type Oc2ceR = crate::BitReader;
#[doc = "Field `OC2CE` writer - Output Compare 2 clear enable Refer to OC1CE description."]
pub type Oc2ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oc1m2 {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
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
#[doc = "Field `OC1M2` reader - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
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
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
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
#[doc = "Field `OC1M2` writer - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type Oc1m2W<'a, REG> = crate::BitWriter<'a, REG, Oc1m2>;
impl<'a, REG> Oc1m2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
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
#[doc = "Field `OC2M2` reader - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc2m2R = crate::BitReader;
#[doc = "Field `OC2M2` writer - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc2m2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&self) -> Oc1m1R {
        Oc1m1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> Oc1ceR {
        Oc1ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> Cc2sR {
        Cc2sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc2fe(&self) -> Oc2feR {
        Oc2feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc2pe(&self) -> Oc2peR {
        Oc2peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc2m1(&self) -> Oc2m1R {
        Oc2m1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc2ce(&self) -> Oc2ceR {
        Oc2ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&self) -> Oc1m2R {
        Oc1m2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc2m2(&self) -> Oc2m2R {
        Oc2m2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> Cc1sW<Ccmr1OutputSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> Oc1feW<Ccmr1OutputSpec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> Oc1peW<Ccmr1OutputSpec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> Oc1m1W<Ccmr1OutputSpec> {
        Oc1m1W::new(self, 4)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> Oc1ceW<Ccmr1OutputSpec> {
        Oc1ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0' in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> Cc2sW<Ccmr1OutputSpec> {
        Cc2sW::new(self, 8)
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> Oc2feW<Ccmr1OutputSpec> {
        Oc2feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> Oc2peW<Ccmr1OutputSpec> {
        Oc2peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc2m1(&mut self) -> Oc2m1W<Ccmr1OutputSpec> {
        Oc2m1W::new(self, 12)
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> Oc2ceW<Ccmr1OutputSpec> {
        Oc2ceW::new(self, 15)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m2(&mut self) -> Oc1m2W<Ccmr1OutputSpec> {
        Oc1m2W::new(self, 16)
    }
    #[doc = "Bit 24 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc2m2(&mut self) -> Oc2m2W<Ccmr1OutputSpec> {
        Oc2m2W::new(self, 24)
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
