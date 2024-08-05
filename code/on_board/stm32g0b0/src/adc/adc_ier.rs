#[doc = "Register `ADC_IER` reader"]
pub type R = crate::R<AdcIerSpec>;
#[doc = "Register `ADC_IER` writer"]
pub type W = crate::W<AdcIerSpec>;
#[doc = "ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrdyie {
    #[doc = "0: ADRDY interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    B0x1 = 1,
}
impl From<Adrdyie> for bool {
    #[inline(always)]
    fn from(variant: Adrdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AdrdyieR = crate::BitReader<Adrdyie>;
impl AdrdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrdyie {
        match self.bits {
            false => Adrdyie::B0x0,
            true => Adrdyie::B0x1,
        }
    }
    #[doc = "ADRDY interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adrdyie::B0x0
    }
    #[doc = "ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adrdyie::B0x1
    }
}
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AdrdyieW<'a, REG> = crate::BitWriter<'a, REG, Adrdyie>;
impl<'a, REG> AdrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADRDY interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrdyie::B0x0)
    }
    #[doc = "ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adrdyie::B0x1)
    }
}
#[doc = "End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosmpie {
    #[doc = "0: EOSMP interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    B0x1 = 1,
}
impl From<Eosmpie> for bool {
    #[inline(always)]
    fn from(variant: Eosmpie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMPIE` reader - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EosmpieR = crate::BitReader<Eosmpie>;
impl EosmpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosmpie {
        match self.bits {
            false => Eosmpie::B0x0,
            true => Eosmpie::B0x1,
        }
    }
    #[doc = "EOSMP interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eosmpie::B0x0
    }
    #[doc = "EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eosmpie::B0x1
    }
}
#[doc = "Field `EOSMPIE` writer - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EosmpieW<'a, REG> = crate::BitWriter<'a, REG, Eosmpie>;
impl<'a, REG> EosmpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOSMP interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosmpie::B0x0)
    }
    #[doc = "EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosmpie::B0x1)
    }
}
#[doc = "End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocie {
    #[doc = "0: EOC interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    B0x1 = 1,
}
impl From<Eocie> for bool {
    #[inline(always)]
    fn from(variant: Eocie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EocieR = crate::BitReader<Eocie>;
impl EocieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocie {
        match self.bits {
            false => Eocie::B0x0,
            true => Eocie::B0x1,
        }
    }
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eocie::B0x0
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eocie::B0x1
    }
}
#[doc = "Field `EOCIE` writer - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG, Eocie>;
impl<'a, REG> EocieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocie::B0x0)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocie::B0x1)
    }
}
#[doc = "End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosie {
    #[doc = "0: EOS interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    B0x1 = 1,
}
impl From<Eosie> for bool {
    #[inline(always)]
    fn from(variant: Eosie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSIE` reader - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EosieR = crate::BitReader<Eosie>;
impl EosieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosie {
        match self.bits {
            false => Eosie::B0x0,
            true => Eosie::B0x1,
        }
    }
    #[doc = "EOS interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eosie::B0x0
    }
    #[doc = "EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eosie::B0x1
    }
}
#[doc = "Field `EOSIE` writer - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EosieW<'a, REG> = crate::BitWriter<'a, REG, Eosie>;
impl<'a, REG> EosieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOS interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosie::B0x0)
    }
    #[doc = "EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosie::B0x1)
    }
}
#[doc = "Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrie {
    #[doc = "0: Overrun interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    B0x1 = 1,
}
impl From<Ovrie> for bool {
    #[inline(always)]
    fn from(variant: Ovrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRIE` reader - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OvrieR = crate::BitReader<Ovrie>;
impl OvrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrie {
        match self.bits {
            false => Ovrie::B0x0,
            true => Ovrie::B0x1,
        }
    }
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovrie::B0x0
    }
    #[doc = "Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovrie::B0x1
    }
}
#[doc = "Field `OVRIE` writer - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG, Ovrie>;
impl<'a, REG> OvrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrie::B0x0)
    }
    #[doc = "Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrie::B0x1)
    }
}
#[doc = "Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd1ie {
    #[doc = "0: Analog watchdog interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    B0x1 = 1,
}
impl From<Awd1ie> for bool {
    #[inline(always)]
    fn from(variant: Awd1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1IE` reader - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1ieR = crate::BitReader<Awd1ie>;
impl Awd1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd1ie {
        match self.bits {
            false => Awd1ie::B0x0,
            true => Awd1ie::B0x1,
        }
    }
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd1ie::B0x0
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd1ie::B0x1
    }
}
#[doc = "Field `AWD1IE` writer - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1ieW<'a, REG> = crate::BitWriter<'a, REG, Awd1ie>;
impl<'a, REG> Awd1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1ie::B0x0)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1ie::B0x1)
    }
}
#[doc = "Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ie {
    #[doc = "0: Analog watchdog interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    B0x1 = 1,
}
impl From<Awd2ie> for bool {
    #[inline(always)]
    fn from(variant: Awd2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2IE` reader - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd2ieR = crate::BitReader<Awd2ie>;
impl Awd2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ie {
        match self.bits {
            false => Awd2ie::B0x0,
            true => Awd2ie::B0x1,
        }
    }
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ie::B0x0
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ie::B0x1
    }
}
#[doc = "Field `AWD2IE` writer - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd2ieW<'a, REG> = crate::BitWriter<'a, REG, Awd2ie>;
impl<'a, REG> Awd2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ie::B0x0)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ie::B0x1)
    }
}
#[doc = "Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ie {
    #[doc = "0: Analog watchdog interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    B0x1 = 1,
}
impl From<Awd3ie> for bool {
    #[inline(always)]
    fn from(variant: Awd3ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3IE` reader - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd3ieR = crate::BitReader<Awd3ie>;
impl Awd3ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ie {
        match self.bits {
            false => Awd3ie::B0x0,
            true => Awd3ie::B0x1,
        }
    }
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ie::B0x0
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ie::B0x1
    }
}
#[doc = "Field `AWD3IE` writer - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd3ieW<'a, REG> = crate::BitWriter<'a, REG, Awd3ie>;
impl<'a, REG> Awd3ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ie::B0x0)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ie::B0x1)
    }
}
#[doc = "End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocalie {
    #[doc = "0: End of calibration interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: End of calibration interrupt enabled"]
    B0x1 = 1,
}
impl From<Eocalie> for bool {
    #[inline(always)]
    fn from(variant: Eocalie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCALIE` reader - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EocalieR = crate::BitReader<Eocalie>;
impl EocalieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocalie {
        match self.bits {
            false => Eocalie::B0x0,
            true => Eocalie::B0x1,
        }
    }
    #[doc = "End of calibration interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eocalie::B0x0
    }
    #[doc = "End of calibration interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eocalie::B0x1
    }
}
#[doc = "Field `EOCALIE` writer - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EocalieW<'a, REG> = crate::BitWriter<'a, REG, Eocalie>;
impl<'a, REG> EocalieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of calibration interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocalie::B0x0)
    }
    #[doc = "End of calibration interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocalie::B0x1)
    }
}
#[doc = "Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccrdyie {
    #[doc = "0: Channel configuration ready interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Channel configuration ready interrupt enabled"]
    B0x1 = 1,
}
impl From<Ccrdyie> for bool {
    #[inline(always)]
    fn from(variant: Ccrdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDYIE` reader - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CcrdyieR = crate::BitReader<Ccrdyie>;
impl CcrdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccrdyie {
        match self.bits {
            false => Ccrdyie::B0x0,
            true => Ccrdyie::B0x1,
        }
    }
    #[doc = "Channel configuration ready interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccrdyie::B0x0
    }
    #[doc = "Channel configuration ready interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccrdyie::B0x1
    }
}
#[doc = "Field `CCRDYIE` writer - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CcrdyieW<'a, REG> = crate::BitWriter<'a, REG, Ccrdyie>;
impl<'a, REG> CcrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel configuration ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrdyie::B0x0)
    }
    #[doc = "Channel configuration ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrdyie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn adrdyie(&self) -> AdrdyieR {
        AdrdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eosmpie(&self) -> EosmpieR {
        EosmpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eosie(&self) -> EosieR {
        EosieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ie(&self) -> Awd1ieR {
        Awd1ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ie(&self) -> Awd2ieR {
        Awd2ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ie(&self) -> Awd3ieR {
        Awd3ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eocalie(&self) -> EocalieR {
        EocalieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ccrdyie(&self) -> CcrdyieR {
        CcrdyieR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> AdrdyieW<AdcIerSpec> {
        AdrdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EosmpieW<AdcIerSpec> {
        EosmpieW::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EocieW<AdcIerSpec> {
        EocieW::new(self, 2)
    }
    #[doc = "Bit 3 - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EosieW<AdcIerSpec> {
        EosieW::new(self, 3)
    }
    #[doc = "Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OvrieW<AdcIerSpec> {
        OvrieW::new(self, 4)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> Awd1ieW<AdcIerSpec> {
        Awd1ieW::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> Awd2ieW<AdcIerSpec> {
        Awd2ieW::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> Awd3ieW<AdcIerSpec> {
        Awd3ieW::new(self, 9)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eocalie(&mut self) -> EocalieW<AdcIerSpec> {
        EocalieW::new(self, 11)
    }
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ccrdyie(&mut self) -> CcrdyieW<AdcIerSpec> {
        CcrdyieW::new(self, 13)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcIerSpec;
impl crate::RegisterSpec for AdcIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ier::R`](R) reader structure"]
impl crate::Readable for AdcIerSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_ier::W`](W) writer structure"]
impl crate::Writable for AdcIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_IER to value 0"]
impl crate::Resettable for AdcIerSpec {
    const RESET_VALUE: u32 = 0;
}
