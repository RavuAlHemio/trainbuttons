#[doc = "Register `TIM1_SMCR` reader"]
pub type R = crate::R<Tim1SmcrSpec>;
#[doc = "Register `TIM1_SMCR` writer"]
pub type W = crate::W<Tim1SmcrSpec>;
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sms1 {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B0x0 = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    B0x1 = 1,
    #[doc = "2: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    B0x2 = 2,
    #[doc = "3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    B0x3 = 3,
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
#[doc = "Field `SMS1` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms1R = crate::FieldReader<Sms1>;
impl Sms1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sms1 {
        match self.bits {
            0 => Sms1::B0x0,
            1 => Sms1::B0x1,
            2 => Sms1::B0x2,
            3 => Sms1::B0x3,
            4 => Sms1::B0x4,
            5 => Sms1::B0x5,
            6 => Sms1::B0x6,
            7 => Sms1::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sms1::B0x0
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sms1::B0x1
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Sms1::B0x2
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Sms1::B0x3
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
#[doc = "Field `SMS1` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sms1, crate::Safe>;
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
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x1)
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x2)
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sms1::B0x3)
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
#[doc = "OCREF clear selection This bit is used to select the OCREF clear source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Occs {
    #[doc = "0: OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIM1_OR1.OCREF_CLR"]
    B0x0 = 0,
    #[doc = "1: OCREF_CLR_INT is connected to ETRF"]
    B0x1 = 1,
}
impl From<Occs> for bool {
    #[inline(always)]
    fn from(variant: Occs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCCS` reader - OCREF clear selection This bit is used to select the OCREF clear source."]
pub type OccsR = crate::BitReader<Occs>;
impl OccsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Occs {
        match self.bits {
            false => Occs::B0x0,
            true => Occs::B0x1,
        }
    }
    #[doc = "OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIM1_OR1.OCREF_CLR"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Occs::B0x0
    }
    #[doc = "OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Occs::B0x1
    }
}
#[doc = "Field `OCCS` writer - OCREF clear selection This bit is used to select the OCREF clear source."]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG, Occs>;
impl<'a, REG> OccsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIM1_OR1.OCREF_CLR"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Occs::B0x0)
    }
    #[doc = "OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Occs::B0x1)
    }
}
#[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
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
    #[doc = "7: External Trigger input (ETRF)"]
    B0x7 = 7,
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
#[doc = "Field `TS1` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts1R = crate::FieldReader<Ts1>;
impl Ts1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts1 {
        match self.bits {
            0 => Ts1::B0x0,
            1 => Ts1::B0x1,
            2 => Ts1::B0x2,
            3 => Ts1::B0x3,
            4 => Ts1::B0x4,
            5 => Ts1::B0x5,
            6 => Ts1::B0x6,
            7 => Ts1::B0x7,
            _ => unreachable!(),
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
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ts1::B0x7
    }
}
#[doc = "Field `TS1` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Ts1, crate::Safe>;
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
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1::B0x7)
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
#[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etf {
    #[doc = "0: No filter, sampling is done at fDTS"]
    B0x0 = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    B0x1 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    B0x2 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    B0x3 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    B0x4 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    B0x5 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    B0x6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    B0x7 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    B0x8 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    B0x9 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    B0xA = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    B0xB = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    B0xC = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    B0xD = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    B0xE = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    B0xF = 15,
}
impl From<Etf> for u8 {
    #[inline(always)]
    fn from(variant: Etf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etf {
    type Ux = u8;
}
impl crate::IsEnum for Etf {}
#[doc = "Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type EtfR = crate::FieldReader<Etf>;
impl EtfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etf {
        match self.bits {
            0 => Etf::B0x0,
            1 => Etf::B0x1,
            2 => Etf::B0x2,
            3 => Etf::B0x3,
            4 => Etf::B0x4,
            5 => Etf::B0x5,
            6 => Etf::B0x6,
            7 => Etf::B0x7,
            8 => Etf::B0x8,
            9 => Etf::B0x9,
            10 => Etf::B0xA,
            11 => Etf::B0xB,
            12 => Etf::B0xC,
            13 => Etf::B0xD,
            14 => Etf::B0xE,
            15 => Etf::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Etf::B0x0
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Etf::B0x1
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Etf::B0x2
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Etf::B0x3
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Etf::B0x4
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Etf::B0x5
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Etf::B0x6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Etf::B0x7
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Etf::B0x8
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Etf::B0x9
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Etf::B0xA
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Etf::B0xB
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Etf::B0xC
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Etf::B0xD
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Etf::B0xE
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Etf::B0xF
    }
}
#[doc = "Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type EtfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Etf, crate::Safe>;
impl<'a, REG> EtfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x0)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x1)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0x9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0xA)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0xB)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0xC)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0xD)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0xE)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Etf::B0xF)
    }
}
#[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etps {
    #[doc = "0: Prescaler OFF"]
    B0x0 = 0,
    #[doc = "1: ETRP frequency divided by 2"]
    B0x1 = 1,
    #[doc = "2: ETRP frequency divided by 4"]
    B0x2 = 2,
    #[doc = "3: ETRP frequency divided by 8"]
    B0x3 = 3,
}
impl From<Etps> for u8 {
    #[inline(always)]
    fn from(variant: Etps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etps {
    type Ux = u8;
}
impl crate::IsEnum for Etps {}
#[doc = "Field `ETPS` reader - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
pub type EtpsR = crate::FieldReader<Etps>;
impl EtpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etps {
        match self.bits {
            0 => Etps::B0x0,
            1 => Etps::B0x1,
            2 => Etps::B0x2,
            3 => Etps::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Etps::B0x0
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Etps::B0x1
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Etps::B0x2
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Etps::B0x3
    }
}
#[doc = "Field `ETPS` writer - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
pub type EtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Etps, crate::Safe>;
impl<'a, REG> EtpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::B0x0)
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::B0x1)
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::B0x2)
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::B0x3)
    }
}
#[doc = "External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ece {
    #[doc = "0: External clock mode 2 disabled"]
    B0x0 = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    B0x1 = 1,
}
impl From<Ece> for bool {
    #[inline(always)]
    fn from(variant: Ece) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type EceR = crate::BitReader<Ece>;
impl EceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ece {
        match self.bits {
            false => Ece::B0x0,
            true => Ece::B0x1,
        }
    }
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ece::B0x0
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ece::B0x1
    }
}
#[doc = "Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type EceW<'a, REG> = crate::BitWriter<'a, REG, Ece>;
impl<'a, REG> EceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ece::B0x0)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ece::B0x1)
    }
}
#[doc = "External trigger polarity This bit selects whether ETR or ETR is used for trigger operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etp {
    #[doc = "0: ETR is non-inverted, active at high level or rising edge."]
    B0x0 = 0,
    #[doc = "1: ETR is inverted, active at low level or falling edge."]
    B0x1 = 1,
}
impl From<Etp> for bool {
    #[inline(always)]
    fn from(variant: Etp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETP` reader - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
pub type EtpR = crate::BitReader<Etp>;
impl EtpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etp {
        match self.bits {
            false => Etp::B0x0,
            true => Etp::B0x1,
        }
    }
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Etp::B0x0
    }
    #[doc = "ETR is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Etp::B0x1
    }
}
#[doc = "Field `ETP` writer - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG, Etp>;
impl<'a, REG> EtpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Etp::B0x0)
    }
    #[doc = "ETR is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Etp::B0x1)
    }
}
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sms2 {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B0x0 = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    B0x1 = 1,
}
impl From<Sms2> for bool {
    #[inline(always)]
    fn from(variant: Sms2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS2` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type Sms2R = crate::BitReader<Sms2>;
impl Sms2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sms2 {
        match self.bits {
            false => Sms2::B0x0,
            true => Sms2::B0x1,
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sms2::B0x0
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sms2::B0x1
    }
}
#[doc = "Field `SMS2` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
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
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sms2::B0x1)
    }
}
#[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
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
#[doc = "Field `TS2` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
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
#[doc = "Field `TS2` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
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
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> Sms1R {
        Sms1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source."]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> Ts1R {
        Ts1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&self) -> EtfR {
        EtfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    pub fn etps(&self) -> EtpsR {
        EtpsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&self) -> EceR {
        EceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> Sms2R {
        Sms2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> Ts2R {
        Ts2R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms1(&mut self) -> Sms1W<Tim1SmcrSpec> {
        Sms1W::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source."]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OccsW<Tim1SmcrSpec> {
        OccsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> Ts1W<Tim1SmcrSpec> {
        Ts1W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MsmW<Tim1SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> EtfW<Tim1SmcrSpec> {
        EtfW::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> EtpsW<Tim1SmcrSpec> {
        EtpsW::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> EceW<Tim1SmcrSpec> {
        EceW::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> EtpW<Tim1SmcrSpec> {
        EtpW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms2(&mut self) -> Sms2W<Tim1SmcrSpec> {
        Sms2W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> Ts2W<Tim1SmcrSpec> {
        Ts2W::new(self, 20)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1SmcrSpec;
impl crate::RegisterSpec for Tim1SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_smcr::R`](R) reader structure"]
impl crate::Readable for Tim1SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_smcr::W`](W) writer structure"]
impl crate::Writable for Tim1SmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_SMCR to value 0"]
impl crate::Resettable for Tim1SmcrSpec {
    const RESET_VALUE: u32 = 0;
}
