#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sms1 {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B0x0 = 0,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    B0x4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    B0x5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    B0x6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    B0x7 = 7,
}
impl From<Sms1> for u8 {
    #[inline(always)]
    fn from(variant: Sms1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sms1 {
    type Ux = u8;
}
impl crate::IsEnum for Sms1 {}
#[doc = "Field `SMS1` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms1R = crate::FieldReader<Sms1>;
impl Sms1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sms1> {
        match self.bits {
            0 => Some(Sms1::B0x0),
            4 => Some(Sms1::B0x4),
            5 => Some(Sms1::B0x5),
            6 => Some(Sms1::B0x6),
            7 => Some(Sms1::B0x7),
            _ => None,
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sms1::B0x0
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Sms1::B0x4
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Sms1::B0x5
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Sms1::B0x6
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Sms1::B0x7
    }
}
#[doc = "Field `SMS1` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sms1>;
impl<'a, REG> Sms1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x0)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x4)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x5)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x6)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x7)
    }
}
#[doc = "Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts1 {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    B0x0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    B0x1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    B0x2 = 2,
    #[doc = "3: Internal Trigger 3 (ITR3)"]
    B0x3 = 3,
    #[doc = "4: TI1 Edge Detector (TI1F_ED)"]
    B0x4 = 4,
    #[doc = "5: Filtered Timer Input 1 (TI1FP1)"]
    B0x5 = 5,
    #[doc = "6: Filtered Timer Input 2 (TI2FP2)"]
    B0x6 = 6,
}
impl From<Ts1> for u8 {
    #[inline(always)]
    fn from(variant: Ts1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ts1 {
    type Ux = u8;
}
impl crate::IsEnum for Ts1 {}
#[doc = "Field `TS1` reader - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts1R = crate::FieldReader<Ts1>;
impl Ts1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ts1> {
        match self.bits {
            0 => Some(Ts1::B0x0),
            1 => Some(Ts1::B0x1),
            2 => Some(Ts1::B0x2),
            3 => Some(Ts1::B0x3),
            4 => Some(Ts1::B0x4),
            5 => Some(Ts1::B0x5),
            6 => Some(Ts1::B0x6),
            _ => None,
        }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ts1::B0x0
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ts1::B0x1
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ts1::B0x2
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ts1::B0x3
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ts1::B0x4
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ts1::B0x5
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ts1::B0x6
    }
}
#[doc = "Field `TS1` writer - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Ts1>;
impl<'a, REG> Ts1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x3)
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x4)
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x5)
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x6)
    }
}
#[doc = "Master/slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msm {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    B0x1 = 1,
}
impl From<Msm> for bool {
    #[inline(always)]
    fn from(variant: Msm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSM` reader - Master/slave mode"]
pub type MsmR = crate::BitReader<Msm>;
impl MsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msm {
        match self.bits {
            false => Msm::B0x0,
            true => Msm::B0x1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msm::B0x0
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msm::B0x1
    }
}
#[doc = "Field `MSM` writer - Master/slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG, Msm>;
impl<'a, REG> MsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msm::B0x0)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msm::B0x1)
    }
}
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sms2 {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B0x0 = 0,
}
impl From<Sms2> for bool {
    #[inline(always)]
    fn from(variant: Sms2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS2` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms2R = crate::BitReader<Sms2>;
impl Sms2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sms2> {
        match self.bits {
            false => Some(Sms2::B0x0),
            _ => None,
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sms2::B0x0
    }
}
#[doc = "Field `SMS2` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms2W<'a, REG> = crate::BitWriter<'a, REG, Sms2>;
impl<'a, REG> Sms2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sms2::B0x0)
    }
}
#[doc = "Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts2 {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    B0x0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    B0x1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    B0x2 = 2,
    #[doc = "3: Internal Trigger 3 (ITR3)"]
    B0x3 = 3,
}
impl From<Ts2> for u8 {
    #[inline(always)]
    fn from(variant: Ts2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ts2 {
    type Ux = u8;
}
impl crate::IsEnum for Ts2 {}
#[doc = "Field `TS2` reader - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts2R = crate::FieldReader<Ts2>;
impl Ts2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts2 {
        match self.bits {
            0 => Ts2::B0x0,
            1 => Ts2::B0x1,
            2 => Ts2::B0x2,
            3 => Ts2::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ts2::B0x0
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ts2::B0x1
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ts2::B0x2
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ts2::B0x3
    }
}
#[doc = "Field `TS2` writer - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ts2, crate::Safe>;
impl<'a, REG> Ts2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ts2::B0x0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts2::B0x1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ts2::B0x2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ts2::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> Sms1R {
        Sms1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> Ts1R {
        Ts1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> Sms2R {
        Sms2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> Ts2R {
        Ts2R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms1(&mut self) -> Sms1W<SmcrSpec> {
        Sms1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> Ts1W<SmcrSpec> {
        Ts1W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MsmW<SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='00100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms2(&mut self) -> Sms2W<SmcrSpec> {
        Sms2W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> Ts2W<SmcrSpec> {
        Ts2W::new(self, 20)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {
    const RESET_VALUE: u32 = 0;
}
