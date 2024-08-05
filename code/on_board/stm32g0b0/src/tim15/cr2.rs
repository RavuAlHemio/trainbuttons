#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
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
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only."]
    B0x0 = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI."]
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
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccus::B0x0
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI."]
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
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccus::B0x0)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI."]
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
#[doc = "Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows:\n\nValue on reset: 0"]
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
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO)."]
    B0x4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO)."]
    B0x5 = 5,
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
#[doc = "Field `MMS` reader - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows:"]
pub type MmsR = crate::FieldReader<Mms>;
impl MmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mms> {
        match self.bits {
            0 => Some(Mms::B0x0),
            1 => Some(Mms::B0x1),
            2 => Some(Mms::B0x2),
            3 => Some(Mms::B0x3),
            4 => Some(Mms::B0x4),
            5 => Some(Mms::B0x5),
            _ => None,
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
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mms::B0x4
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mms::B0x5
    }
}
#[doc = "Field `MMS` writer - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows:"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mms>;
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
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::B0x5)
    }
}
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ti1s {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    B0x0 = 0,
    #[doc = "1: The TIMx_CH1, CH2 pins are connected to the TI1 input (XOR combination)"]
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
    #[doc = "The TIMx_CH1, CH2 pins are connected to the TI1 input (XOR combination)"]
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
    #[doc = "The TIMx_CH1, CH2 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1s::B0x1)
    }
}
#[doc = "Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register).\n\nValue on reset: 0"]
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
#[doc = "Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
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
#[doc = "Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
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
#[doc = "Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register).\n\nValue on reset: 0"]
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
#[doc = "Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
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
#[doc = "Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
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
#[doc = "Output idle state 2 (OC2 output) Note: This bit cannot be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in the TIM15_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ois2 {
    #[doc = "0: OC2=0 when MOE=0"]
    B0x0 = 0,
    #[doc = "1: OC2=1 when MOE=0"]
    B0x1 = 1,
}
impl From<Ois2> for bool {
    #[inline(always)]
    fn from(variant: Ois2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS2` reader - Output idle state 2 (OC2 output) Note: This bit cannot be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in the TIM15_BDTR register)."]
pub type Ois2R = crate::BitReader<Ois2>;
impl Ois2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ois2 {
        match self.bits {
            false => Ois2::B0x0,
            true => Ois2::B0x1,
        }
    }
    #[doc = "OC2=0 when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ois2::B0x0
    }
    #[doc = "OC2=1 when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ois2::B0x1
    }
}
#[doc = "Field `OIS2` writer - Output idle state 2 (OC2 output) Note: This bit cannot be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in the TIM15_BDTR register)."]
pub type Ois2W<'a, REG> = crate::BitWriter<'a, REG, Ois2>;
impl<'a, REG> Ois2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC2=0 when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ois2::B0x0)
    }
    #[doc = "OC2=1 when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ois2::B0x1)
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
    #[doc = "Bits 4:6 - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows:"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> Ti1sR {
        Ti1sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output idle state 2 (OC2 output) Note: This bit cannot be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in the TIM15_BDTR register)."]
    #[inline(always)]
    pub fn ois2(&self) -> Ois2R {
        Ois2R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CcpcW<Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CcusW<Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CcdsW<Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MmsW<Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> Ti1sW<Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> Ois1W<Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> Ois1nW<Cr2Spec> {
        Ois1nW::new(self, 9)
    }
    #[doc = "Bit 10 - Output idle state 2 (OC2 output) Note: This bit cannot be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in the TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> Ois2W<Cr2Spec> {
        Ois2W::new(self, 10)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
