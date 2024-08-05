#[doc = "Register `TIM1_AF1` reader"]
pub type R = crate::R<Tim1Af1Spec>;
#[doc = "Register `TIM1_AF1` writer"]
pub type W = crate::W<Tim1Af1Spec>;
#[doc = "BRK BKIN input enable This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkine {
    #[doc = "0: BKIN input disabled"]
    B0x0 = 0,
    #[doc = "1: BKIN input enabled"]
    B0x1 = 1,
}
impl From<Bkine> for bool {
    #[inline(always)]
    fn from(variant: Bkine) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINE` reader - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BkineR = crate::BitReader<Bkine>;
impl BkineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkine {
        match self.bits {
            false => Bkine::B0x0,
            true => Bkine::B0x1,
        }
    }
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkine::B0x0
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkine::B0x1
    }
}
#[doc = "Field `BKINE` writer - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG, Bkine>;
impl<'a, REG> BkineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkine::B0x0)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkine::B0x1)
    }
}
#[doc = "BRK COMP1 enable This bit enables the COMP1 for the timer's BRK input. COMP1 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkcmp1e {
    #[doc = "0: COMP1 input disabled"]
    B0x0 = 0,
    #[doc = "1: COMP1 input enabled"]
    B0x1 = 1,
}
impl From<Bkcmp1e> for bool {
    #[inline(always)]
    fn from(variant: Bkcmp1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable This bit enables the COMP1 for the timer's BRK input. COMP1 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp1eR = crate::BitReader<Bkcmp1e>;
impl Bkcmp1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkcmp1e {
        match self.bits {
            false => Bkcmp1e::B0x0,
            true => Bkcmp1e::B0x1,
        }
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkcmp1e::B0x0
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkcmp1e::B0x1
    }
}
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable This bit enables the COMP1 for the timer's BRK input. COMP1 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp1eW<'a, REG> = crate::BitWriter<'a, REG, Bkcmp1e>;
impl<'a, REG> Bkcmp1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp1e::B0x0)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp1e::B0x1)
    }
}
#[doc = "BRK COMP2 enable This bit enables the COMP2 for the timer's BRK input. COMP2 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkcmp2e {
    #[doc = "0: COMP2 input disabled"]
    B0x0 = 0,
    #[doc = "1: COMP2 input enabled"]
    B0x1 = 1,
}
impl From<Bkcmp2e> for bool {
    #[inline(always)]
    fn from(variant: Bkcmp2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable This bit enables the COMP2 for the timer's BRK input. COMP2 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp2eR = crate::BitReader<Bkcmp2e>;
impl Bkcmp2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkcmp2e {
        match self.bits {
            false => Bkcmp2e::B0x0,
            true => Bkcmp2e::B0x1,
        }
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkcmp2e::B0x0
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkcmp2e::B0x1
    }
}
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable This bit enables the COMP2 for the timer's BRK input. COMP2 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp2eW<'a, REG> = crate::BitWriter<'a, REG, Bkcmp2e>;
impl<'a, REG> Bkcmp2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp2e::B0x0)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp2e::B0x1)
    }
}
#[doc = "BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkinp {
    #[doc = "0: BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    B0x0 = 0,
    #[doc = "1: BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    B0x1 = 1,
}
impl From<Bkinp> for bool {
    #[inline(always)]
    fn from(variant: Bkinp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINP` reader - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BkinpR = crate::BitReader<Bkinp>;
impl BkinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkinp {
        match self.bits {
            false => Bkinp::B0x0,
            true => Bkinp::B0x1,
        }
    }
    #[doc = "BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkinp::B0x0
    }
    #[doc = "BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkinp::B0x1
    }
}
#[doc = "Field `BKINP` writer - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG, Bkinp>;
impl<'a, REG> BkinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkinp::B0x0)
    }
    #[doc = "BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkinp::B0x1)
    }
}
#[doc = "BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkcmp1p {
    #[doc = "0: COMP1 input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    B0x0 = 0,
    #[doc = "1: COMP1 input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    B0x1 = 1,
}
impl From<Bkcmp1p> for bool {
    #[inline(always)]
    fn from(variant: Bkcmp1p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp1pR = crate::BitReader<Bkcmp1p>;
impl Bkcmp1pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkcmp1p {
        match self.bits {
            false => Bkcmp1p::B0x0,
            true => Bkcmp1p::B0x1,
        }
    }
    #[doc = "COMP1 input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkcmp1p::B0x0
    }
    #[doc = "COMP1 input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkcmp1p::B0x1
    }
}
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp1pW<'a, REG> = crate::BitWriter<'a, REG, Bkcmp1p>;
impl<'a, REG> Bkcmp1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp1p::B0x0)
    }
    #[doc = "COMP1 input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp1p::B0x1)
    }
}
#[doc = "BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkcmp2p {
    #[doc = "0: COMP2 input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    B0x0 = 0,
    #[doc = "1: COMP2 input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    B0x1 = 1,
}
impl From<Bkcmp2p> for bool {
    #[inline(always)]
    fn from(variant: Bkcmp2p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp2pR = crate::BitReader<Bkcmp2p>;
impl Bkcmp2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkcmp2p {
        match self.bits {
            false => Bkcmp2p::B0x0,
            true => Bkcmp2p::B0x1,
        }
    }
    #[doc = "COMP2 input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkcmp2p::B0x0
    }
    #[doc = "COMP2 input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkcmp2p::B0x1
    }
}
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bkcmp2pW<'a, REG> = crate::BitWriter<'a, REG, Bkcmp2p>;
impl<'a, REG> Bkcmp2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input polarity is not inverted (active low if BKP=0, active high if BKP=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp2p::B0x0)
    }
    #[doc = "COMP2 input polarity is inverted (active high if BKP=0, active low if BKP=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkcmp2p::B0x1)
    }
}
#[doc = "ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etrsel {
    #[doc = "0: ETR legacy mode"]
    B0x0 = 0,
    #[doc = "1: COMP1 output"]
    B0x1 = 1,
    #[doc = "2: COMP2 output"]
    B0x2 = 2,
    #[doc = "3: ADC1 AWD1"]
    B0x3 = 3,
    #[doc = "4: ADC1 AWD2"]
    B0x4 = 4,
    #[doc = "5: ADC1 AWD3"]
    B0x5 = 5,
}
impl From<Etrsel> for u8 {
    #[inline(always)]
    fn from(variant: Etrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etrsel {
    type Ux = u8;
}
impl crate::IsEnum for Etrsel {}
#[doc = "Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type EtrselR = crate::FieldReader<Etrsel>;
impl EtrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Etrsel> {
        match self.bits {
            0 => Some(Etrsel::B0x0),
            1 => Some(Etrsel::B0x1),
            2 => Some(Etrsel::B0x2),
            3 => Some(Etrsel::B0x3),
            4 => Some(Etrsel::B0x4),
            5 => Some(Etrsel::B0x5),
            _ => None,
        }
    }
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Etrsel::B0x0
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Etrsel::B0x1
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Etrsel::B0x2
    }
    #[doc = "ADC1 AWD1"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Etrsel::B0x3
    }
    #[doc = "ADC1 AWD2"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Etrsel::B0x4
    }
    #[doc = "ADC1 AWD3"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Etrsel::B0x5
    }
}
#[doc = "Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type EtrselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Etrsel>;
impl<'a, REG> EtrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x0)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x1)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x2)
    }
    #[doc = "ADC1 AWD1"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x3)
    }
    #[doc = "ADC1 AWD2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x4)
    }
    #[doc = "ADC1 AWD3"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x5)
    }
}
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timer's BRK input. COMP1 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> Bkcmp1eR {
        Bkcmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timer's BRK input. COMP2 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> Bkcmp2eR {
        Bkcmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> Bkcmp1pR {
        Bkcmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> Bkcmp2pR {
        Bkcmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn etrsel(&self) -> EtrselR {
        EtrselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BkineW<Tim1Af1Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timer's BRK input. COMP1 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> Bkcmp1eW<Tim1Af1Spec> {
        Bkcmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timer's BRK input. COMP2 output is 'ORed' with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> Bkcmp2eW<Tim1Af1Spec> {
        Bkcmp2eW::new(self, 2)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BkinpW<Tim1Af1Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> Bkcmp1pW<Tim1Af1Spec> {
        Bkcmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> Bkcmp2pW<Tim1Af1Spec> {
        Bkcmp2pW::new(self, 11)
    }
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> EtrselW<Tim1Af1Spec> {
        EtrselW::new(self, 14)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Af1Spec;
impl crate::RegisterSpec for Tim1Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_af1::R`](R) reader structure"]
impl crate::Readable for Tim1Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_af1::W`](W) writer structure"]
impl crate::Writable for Tim1Af1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_AF1 to value 0x01"]
impl crate::Resettable for Tim1Af1Spec {
    const RESET_VALUE: u32 = 0x01;
}
