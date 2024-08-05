#[doc = "Register `TIM1_AF2` reader"]
pub type R = crate::R<Tim1Af2Spec>;
#[doc = "Register `TIM1_AF2` writer"]
pub type W = crate::W<Tim1Af2Spec>;
#[doc = "BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2ine {
    #[doc = "0: BKIN2 input disabled"]
    B0x0 = 0,
    #[doc = "1: BKIN2 input enabled"]
    B0x1 = 1,
}
impl From<Bk2ine> for bool {
    #[inline(always)]
    fn from(variant: Bk2ine) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2ineR = crate::BitReader<Bk2ine>;
impl Bk2ineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2ine {
        match self.bits {
            false => Bk2ine::B0x0,
            true => Bk2ine::B0x1,
        }
    }
    #[doc = "BKIN2 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2ine::B0x0
    }
    #[doc = "BKIN2 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2ine::B0x1
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2ineW<'a, REG> = crate::BitWriter<'a, REG, Bk2ine>;
impl<'a, REG> Bk2ineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2ine::B0x0)
    }
    #[doc = "BKIN2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2ine::B0x1)
    }
}
#[doc = "BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2cmp1e {
    #[doc = "0: COMP1 input disabled"]
    B0x0 = 0,
    #[doc = "1: COMP1 input enabled"]
    B0x1 = 1,
}
impl From<Bk2cmp1e> for bool {
    #[inline(always)]
    fn from(variant: Bk2cmp1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp1eR = crate::BitReader<Bk2cmp1e>;
impl Bk2cmp1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2cmp1e {
        match self.bits {
            false => Bk2cmp1e::B0x0,
            true => Bk2cmp1e::B0x1,
        }
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2cmp1e::B0x0
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2cmp1e::B0x1
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp1eW<'a, REG> = crate::BitWriter<'a, REG, Bk2cmp1e>;
impl<'a, REG> Bk2cmp1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp1e::B0x0)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp1e::B0x1)
    }
}
#[doc = "BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2cmp2e {
    #[doc = "0: COMP2 input disabled"]
    B0x0 = 0,
    #[doc = "1: COMP2 input enabled"]
    B0x1 = 1,
}
impl From<Bk2cmp2e> for bool {
    #[inline(always)]
    fn from(variant: Bk2cmp2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp2eR = crate::BitReader<Bk2cmp2e>;
impl Bk2cmp2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2cmp2e {
        match self.bits {
            false => Bk2cmp2e::B0x0,
            true => Bk2cmp2e::B0x1,
        }
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2cmp2e::B0x0
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2cmp2e::B0x1
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp2eW<'a, REG> = crate::BitWriter<'a, REG, Bk2cmp2e>;
impl<'a, REG> Bk2cmp2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp2e::B0x0)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp2e::B0x1)
    }
}
#[doc = "BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2inp {
    #[doc = "0: BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B0x0 = 0,
    #[doc = "1: BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B0x1 = 1,
}
impl From<Bk2inp> for bool {
    #[inline(always)]
    fn from(variant: Bk2inp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2inpR = crate::BitReader<Bk2inp>;
impl Bk2inpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2inp {
        match self.bits {
            false => Bk2inp::B0x0,
            true => Bk2inp::B0x1,
        }
    }
    #[doc = "BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2inp::B0x0
    }
    #[doc = "BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2inp::B0x1
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2inpW<'a, REG> = crate::BitWriter<'a, REG, Bk2inp>;
impl<'a, REG> Bk2inpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2inp::B0x0)
    }
    #[doc = "BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2inp::B0x1)
    }
}
#[doc = "BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2cmp1p {
    #[doc = "0: COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B0x0 = 0,
    #[doc = "1: COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B0x1 = 1,
}
impl From<Bk2cmp1p> for bool {
    #[inline(always)]
    fn from(variant: Bk2cmp1p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp1pR = crate::BitReader<Bk2cmp1p>;
impl Bk2cmp1pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2cmp1p {
        match self.bits {
            false => Bk2cmp1p::B0x0,
            true => Bk2cmp1p::B0x1,
        }
    }
    #[doc = "COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2cmp1p::B0x0
    }
    #[doc = "COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2cmp1p::B0x1
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp1pW<'a, REG> = crate::BitWriter<'a, REG, Bk2cmp1p>;
impl<'a, REG> Bk2cmp1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp1p::B0x0)
    }
    #[doc = "COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp1p::B0x1)
    }
}
#[doc = "BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2cmp2p {
    #[doc = "0: COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B0x0 = 0,
    #[doc = "1: COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B0x1 = 1,
}
impl From<Bk2cmp2p> for bool {
    #[inline(always)]
    fn from(variant: Bk2cmp2p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp2pR = crate::BitReader<Bk2cmp2p>;
impl Bk2cmp2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2cmp2p {
        match self.bits {
            false => Bk2cmp2p::B0x0,
            true => Bk2cmp2p::B0x1,
        }
    }
    #[doc = "COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2cmp2p::B0x0
    }
    #[doc = "COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2cmp2p::B0x1
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2cmp2pW<'a, REG> = crate::BitWriter<'a, REG, Bk2cmp2p>;
impl<'a, REG> Bk2cmp2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp2p::B0x0)
    }
    #[doc = "COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2cmp2p::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2ine(&self) -> Bk2ineR {
        Bk2ineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> Bk2cmp1eR {
        Bk2cmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> Bk2cmp2eR {
        Bk2cmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2inp(&self) -> Bk2inpR {
        Bk2inpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> Bk2cmp1pR {
        Bk2cmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> Bk2cmp2pR {
        Bk2cmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> Bk2ineW<Tim1Af2Spec> {
        Bk2ineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> Bk2cmp1eW<Tim1Af2Spec> {
        Bk2cmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> Bk2cmp2eW<Tim1Af2Spec> {
        Bk2cmp2eW::new(self, 2)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> Bk2inpW<Tim1Af2Spec> {
        Bk2inpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> Bk2cmp1pW<Tim1Af2Spec> {
        Bk2cmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> Bk2cmp2pW<Tim1Af2Spec> {
        Bk2cmp2pW::new(self, 11)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Af2Spec;
impl crate::RegisterSpec for Tim1Af2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_af2::R`](R) reader structure"]
impl crate::Readable for Tim1Af2Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_af2::W`](W) writer structure"]
impl crate::Writable for Tim1Af2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_AF2 to value 0x01"]
impl crate::Resettable for Tim1Af2Spec {
    const RESET_VALUE: u32 = 0x01;
}
