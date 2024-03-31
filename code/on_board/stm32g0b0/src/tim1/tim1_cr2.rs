#[doc = "Register `TIM1_CR2` reader"]
pub type R = crate::R<Tim1Cr2Spec>;
#[doc = "Register `TIM1_CR2` writer"]
pub type W = crate::W<Tim1Cr2Spec>;
#[doc = "Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpc {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    B0x0 = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    B0x1 = 1,
}
impl From<Ccpc> for bool {
    #[inline(always)]
    fn from(variant: Ccpc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CcpcR = crate::BitReader<Ccpc>;
impl CcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpc {
        match self.bits {
            false => Ccpc::B0x0,
            true => Ccpc::B0x1,
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccpc::B0x0
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccpc::B0x1
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG, Ccpc>;
impl<'a, REG> CcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpc::B0x0)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpc::B0x1)
    }
}
#[doc = "Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccus {
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    B0x0 = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    B0x1 = 1,
}
impl From<Ccus> for bool {
    #[inline(always)]
    fn from(variant: Ccus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CcusR = crate::BitReader<Ccus>;
impl CcusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccus {
        match self.bits {
            false => Ccus::B0x0,
            true => Ccus::B0x1,
        }
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccus::B0x0
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccus::B0x1
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG, Ccus>;
impl<'a, REG> CcusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccus::B0x0)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccus::B0x1)
    }
}
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccds {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    B0x0 = 0,
    #[doc = "1: CCx DMA requests sent when update event occurs"]
    B0x1 = 1,
}
impl From<Ccds> for bool {
    #[inline(always)]
    fn from(variant: Ccds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CcdsR = crate::BitReader<Ccds>;
impl CcdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccds {
        match self.bits {
            false => Ccds::B0x0,
            true => Ccds::B0x1,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccds::B0x0
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccds::B0x1
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG, Ccds>;
impl<'a, REG> CcdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccds::B0x0)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccds::B0x1)
    }
}
#[doc = "Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mms {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    B0x0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B0x1 = 1,
    #[doc = "2: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    B0x2 = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    B0x3 = 3,
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO)"]
    B0x4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO)"]
    B0x5 = 5,
    #[doc = "6: Compare - OC3REFC signal is used as trigger output (TRGO)"]
    B0x6 = 6,
    #[doc = "7: Compare - OC4REFC signal is used as trigger output (TRGO)"]
    B0x7 = 7,
}
impl From<Mms> for u8 {
    #[inline(always)]
    fn from(variant: Mms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mms {
    type Ux = u8;
}
impl crate::IsEnum for Mms {}
#[doc = "Field `MMS` reader - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MmsR = crate::FieldReader<Mms>;
impl MmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mms {
        match self.bits {
            0 => Mms::B0x0,
            1 => Mms::B0x1,
            2 => Mms::B0x2,
            3 => Mms::B0x3,
            4 => Mms::B0x4,
            5 => Mms::B0x5,
            6 => Mms::B0x6,
            7 => Mms::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mms::B0x0
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mms::B0x1
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mms::B0x2
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mms::B0x3
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mms::B0x4
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mms::B0x5
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mms::B0x6
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mms::B0x7
    }
}
#[doc = "Field `MMS` writer - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mms, crate::Safe>;
impl<'a, REG> MmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x1)
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x2)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x3)
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x5)
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x6)
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x7)
    }
}
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ti1s {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    B0x0 = 0,
    #[doc = "1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    B0x1 = 1,
}
impl From<Ti1s> for bool {
    #[inline(always)]
    fn from(variant: Ti1s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type Ti1sR = crate::BitReader<Ti1s>;
impl Ti1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ti1s {
        match self.bits {
            false => Ti1s::B0x0,
            true => Ti1s::B0x1,
        }
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti1s::B0x0
    }
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ti1s::B0x1
    }
}
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type Ti1sW<'a, REG> = crate::BitWriter<'a, REG, Ti1s>;
impl<'a, REG> Ti1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1s::B0x0)
    }
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1s::B0x1)
    }
}
#[doc = "Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ois1 {
    #[doc = "0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    B0x0 = 0,
    #[doc = "1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    B0x1 = 1,
}
impl From<Ois1> for bool {
    #[inline(always)]
    fn from(variant: Ois1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1R = crate::BitReader<Ois1>;
impl Ois1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ois1 {
        match self.bits {
            false => Ois1::B0x0,
            true => Ois1::B0x1,
        }
    }
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ois1::B0x0
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ois1::B0x1
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG, Ois1>;
impl<'a, REG> Ois1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1::B0x0)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1::B0x1)
    }
}
#[doc = "Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ois1n {
    #[doc = "0: OC1N=0 after a dead-time when MOE=0"]
    B0x0 = 0,
    #[doc = "1: OC1N=1 after a dead-time when MOE=0"]
    B0x1 = 1,
}
impl From<Ois1n> for bool {
    #[inline(always)]
    fn from(variant: Ois1n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1nR = crate::BitReader<Ois1n>;
impl Ois1nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ois1n {
        match self.bits {
            false => Ois1n::B0x0,
            true => Ois1n::B0x1,
        }
    }
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ois1n::B0x0
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ois1n::B0x1
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG, Ois1n>;
impl<'a, REG> Ois1nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1n::B0x0)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1n::B0x1)
    }
}
#[doc = "Field `OIS2` reader - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub type Ois2R = crate::BitReader;
#[doc = "Field `OIS2` writer - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub type Ois2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub type Ois2nR = crate::BitReader;
#[doc = "Field `OIS2N` writer - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub type Ois2nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub type Ois3R = crate::BitReader;
#[doc = "Field `OIS3` writer - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub type Ois3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub type Ois3nR = crate::BitReader;
#[doc = "Field `OIS3N` writer - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub type Ois3nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub type Ois4R = crate::BitReader;
#[doc = "Field `OIS4` writer - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub type Ois4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub type Ois5R = crate::BitReader;
#[doc = "Field `OIS5` writer - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub type Ois5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub type Ois6R = crate::BitReader;
#[doc = "Field `OIS6` writer - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub type Ois6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mms2 {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    B0x0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B0x1 = 1,
    #[doc = "2: Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    B0x2 = 2,
    #[doc = "3: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    B0x3 = 3,
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    B0x4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    B0x5 = 5,
    #[doc = "6: Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    B0x6 = 6,
    #[doc = "7: Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    B0x7 = 7,
    #[doc = "8: Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    B0x8 = 8,
    #[doc = "9: Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    B0x9 = 9,
    #[doc = "10: Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    B0xA = 10,
    #[doc = "11: Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    B0xB = 11,
    #[doc = "12: Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    B0xC = 12,
    #[doc = "13: Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    B0xD = 13,
    #[doc = "14: Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    B0xE = 14,
    #[doc = "15: Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    B0xF = 15,
}
impl From<Mms2> for u8 {
    #[inline(always)]
    fn from(variant: Mms2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mms2 {
    type Ux = u8;
}
impl crate::IsEnum for Mms2 {}
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type Mms2R = crate::FieldReader<Mms2>;
impl Mms2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mms2 {
        match self.bits {
            0 => Mms2::B0x0,
            1 => Mms2::B0x1,
            2 => Mms2::B0x2,
            3 => Mms2::B0x3,
            4 => Mms2::B0x4,
            5 => Mms2::B0x5,
            6 => Mms2::B0x6,
            7 => Mms2::B0x7,
            8 => Mms2::B0x8,
            9 => Mms2::B0x9,
            10 => Mms2::B0xA,
            11 => Mms2::B0xB,
            12 => Mms2::B0xC,
            13 => Mms2::B0xD,
            14 => Mms2::B0xE,
            15 => Mms2::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mms2::B0x0
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mms2::B0x1
    }
    #[doc = "Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mms2::B0x2
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mms2::B0x3
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mms2::B0x4
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mms2::B0x5
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mms2::B0x6
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mms2::B0x7
    }
    #[doc = "Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mms2::B0x8
    }
    #[doc = "Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Mms2::B0x9
    }
    #[doc = "Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Mms2::B0xA
    }
    #[doc = "Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Mms2::B0xB
    }
    #[doc = "Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Mms2::B0xC
    }
    #[doc = "Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Mms2::B0xD
    }
    #[doc = "Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Mms2::B0xE
    }
    #[doc = "Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Mms2::B0xF
    }
}
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type Mms2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mms2, crate::Safe>;
impl<'a, REG> Mms2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x1)
    }
    #[doc = "Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x2)
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x3)
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x5)
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x6)
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x7)
    }
    #[doc = "Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x8)
    }
    #[doc = "Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0x9)
    }
    #[doc = "Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0xA)
    }
    #[doc = "Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0xB)
    }
    #[doc = "Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0xC)
    }
    #[doc = "Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0xD)
    }
    #[doc = "Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0xE)
    }
    #[doc = "Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Mms2::B0xF)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> Ti1sR {
        Ti1sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&self) -> Ois2R {
        Ois2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&self) -> Ois2nR {
        Ois2nR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&self) -> Ois3R {
        Ois3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&self) -> Ois3nR {
        Ois3nR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&self) -> Ois4R {
        Ois4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&self) -> Ois5R {
        Ois5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&self) -> Ois6R {
        Ois6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> Mms2R {
        Mms2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CcpcW<Tim1Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CcusW<Tim1Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CcdsW<Tim1Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MmsW<Tim1Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> Ti1sW<Tim1Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> Ois1W<Tim1Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> Ois1nW<Tim1Cr2Spec> {
        Ois1nW::new(self, 9)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> Ois2W<Tim1Cr2Spec> {
        Ois2W::new(self, 10)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> Ois2nW<Tim1Cr2Spec> {
        Ois2nW::new(self, 11)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> Ois3W<Tim1Cr2Spec> {
        Ois3W::new(self, 12)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> Ois3nW<Tim1Cr2Spec> {
        Ois3nW::new(self, 13)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> Ois4W<Tim1Cr2Spec> {
        Ois4W::new(self, 14)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> Ois5W<Tim1Cr2Spec> {
        Ois5W::new(self, 16)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> Ois6W<Tim1Cr2Spec> {
        Ois6W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> Mms2W<Tim1Cr2Spec> {
        Mms2W::new(self, 20)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Cr2Spec;
impl crate::RegisterSpec for Tim1Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_cr2::R`](R) reader structure"]
impl crate::Readable for Tim1Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_cr2::W`](W) writer structure"]
impl crate::Writable for Tim1Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CR2 to value 0"]
impl crate::Resettable for Tim1Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
