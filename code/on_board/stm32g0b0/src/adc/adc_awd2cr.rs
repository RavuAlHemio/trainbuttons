#[doc = "Register `ADC_AWD2CR` reader"]
pub type R = crate::R<AdcAwd2crSpec>;
#[doc = "Register `ADC_AWD2CR` writer"]
pub type W = crate::W<AdcAwd2crSpec>;
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch0 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch0> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH0` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch0R = crate::BitReader<Awd2ch0>;
impl Awd2ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch0 {
        match self.bits {
            false => Awd2ch0::B0x0,
            true => Awd2ch0::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch0::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch0::B0x1
    }
}
#[doc = "Field `AWD2CH0` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch0W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch0>;
impl<'a, REG> Awd2ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch0::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch0::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch1 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch1> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH1` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch1R = crate::BitReader<Awd2ch1>;
impl Awd2ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch1 {
        match self.bits {
            false => Awd2ch1::B0x0,
            true => Awd2ch1::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch1::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch1::B0x1
    }
}
#[doc = "Field `AWD2CH1` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch1W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch1>;
impl<'a, REG> Awd2ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch1::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch1::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch2 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch2> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH2` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch2R = crate::BitReader<Awd2ch2>;
impl Awd2ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch2 {
        match self.bits {
            false => Awd2ch2::B0x0,
            true => Awd2ch2::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch2::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch2::B0x1
    }
}
#[doc = "Field `AWD2CH2` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch2W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch2>;
impl<'a, REG> Awd2ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch2::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch2::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch3 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch3> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH3` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch3R = crate::BitReader<Awd2ch3>;
impl Awd2ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch3 {
        match self.bits {
            false => Awd2ch3::B0x0,
            true => Awd2ch3::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch3::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch3::B0x1
    }
}
#[doc = "Field `AWD2CH3` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch3W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch3>;
impl<'a, REG> Awd2ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch3::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch3::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch4 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch4> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH4` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch4R = crate::BitReader<Awd2ch4>;
impl Awd2ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch4 {
        match self.bits {
            false => Awd2ch4::B0x0,
            true => Awd2ch4::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch4::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch4::B0x1
    }
}
#[doc = "Field `AWD2CH4` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch4W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch4>;
impl<'a, REG> Awd2ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch4::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch4::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch5 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch5> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH5` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch5R = crate::BitReader<Awd2ch5>;
impl Awd2ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch5 {
        match self.bits {
            false => Awd2ch5::B0x0,
            true => Awd2ch5::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch5::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch5::B0x1
    }
}
#[doc = "Field `AWD2CH5` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch5W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch5>;
impl<'a, REG> Awd2ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch5::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch5::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch6 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch6> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH6` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch6R = crate::BitReader<Awd2ch6>;
impl Awd2ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch6 {
        match self.bits {
            false => Awd2ch6::B0x0,
            true => Awd2ch6::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch6::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch6::B0x1
    }
}
#[doc = "Field `AWD2CH6` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch6W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch6>;
impl<'a, REG> Awd2ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch6::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch6::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch7 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch7> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH7` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch7R = crate::BitReader<Awd2ch7>;
impl Awd2ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch7 {
        match self.bits {
            false => Awd2ch7::B0x0,
            true => Awd2ch7::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch7::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch7::B0x1
    }
}
#[doc = "Field `AWD2CH7` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch7W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch7>;
impl<'a, REG> Awd2ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch7::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch7::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch8 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch8> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH8` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch8R = crate::BitReader<Awd2ch8>;
impl Awd2ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch8 {
        match self.bits {
            false => Awd2ch8::B0x0,
            true => Awd2ch8::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch8::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch8::B0x1
    }
}
#[doc = "Field `AWD2CH8` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch8W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch8>;
impl<'a, REG> Awd2ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch8::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch8::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch9 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch9> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH9` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch9R = crate::BitReader<Awd2ch9>;
impl Awd2ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch9 {
        match self.bits {
            false => Awd2ch9::B0x0,
            true => Awd2ch9::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch9::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch9::B0x1
    }
}
#[doc = "Field `AWD2CH9` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch9W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch9>;
impl<'a, REG> Awd2ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch9::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch9::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch10 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch10> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH10` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch10R = crate::BitReader<Awd2ch10>;
impl Awd2ch10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch10 {
        match self.bits {
            false => Awd2ch10::B0x0,
            true => Awd2ch10::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch10::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch10::B0x1
    }
}
#[doc = "Field `AWD2CH10` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch10W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch10>;
impl<'a, REG> Awd2ch10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch10::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch10::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch11 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch11> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH11` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch11R = crate::BitReader<Awd2ch11>;
impl Awd2ch11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch11 {
        match self.bits {
            false => Awd2ch11::B0x0,
            true => Awd2ch11::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch11::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch11::B0x1
    }
}
#[doc = "Field `AWD2CH11` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch11W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch11>;
impl<'a, REG> Awd2ch11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch11::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch11::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch12 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch12> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH12` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch12R = crate::BitReader<Awd2ch12>;
impl Awd2ch12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch12 {
        match self.bits {
            false => Awd2ch12::B0x0,
            true => Awd2ch12::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch12::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch12::B0x1
    }
}
#[doc = "Field `AWD2CH12` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch12W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch12>;
impl<'a, REG> Awd2ch12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch12::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch12::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch13 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch13> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH13` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch13R = crate::BitReader<Awd2ch13>;
impl Awd2ch13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch13 {
        match self.bits {
            false => Awd2ch13::B0x0,
            true => Awd2ch13::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch13::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch13::B0x1
    }
}
#[doc = "Field `AWD2CH13` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch13W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch13>;
impl<'a, REG> Awd2ch13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch13::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch13::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch14 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch14> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH14` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch14R = crate::BitReader<Awd2ch14>;
impl Awd2ch14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch14 {
        match self.bits {
            false => Awd2ch14::B0x0,
            true => Awd2ch14::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch14::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch14::B0x1
    }
}
#[doc = "Field `AWD2CH14` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch14W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch14>;
impl<'a, REG> Awd2ch14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch14::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch14::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch15 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch15> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH15` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch15R = crate::BitReader<Awd2ch15>;
impl Awd2ch15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch15 {
        match self.bits {
            false => Awd2ch15::B0x0,
            true => Awd2ch15::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch15::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch15::B0x1
    }
}
#[doc = "Field `AWD2CH15` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch15W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch15>;
impl<'a, REG> Awd2ch15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch15::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch15::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch16 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch16> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH16` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch16R = crate::BitReader<Awd2ch16>;
impl Awd2ch16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch16 {
        match self.bits {
            false => Awd2ch16::B0x0,
            true => Awd2ch16::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch16::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch16::B0x1
    }
}
#[doc = "Field `AWD2CH16` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch16W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch16>;
impl<'a, REG> Awd2ch16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch16::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch16::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch17 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch17> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH17` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch17R = crate::BitReader<Awd2ch17>;
impl Awd2ch17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch17 {
        match self.bits {
            false => Awd2ch17::B0x0,
            true => Awd2ch17::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch17::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch17::B0x1
    }
}
#[doc = "Field `AWD2CH17` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch17W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch17>;
impl<'a, REG> Awd2ch17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch17::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch17::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2ch18 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    B0x1 = 1,
}
impl From<Awd2ch18> for bool {
    #[inline(always)]
    fn from(variant: Awd2ch18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH18` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch18R = crate::BitReader<Awd2ch18>;
impl Awd2ch18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2ch18 {
        match self.bits {
            false => Awd2ch18::B0x0,
            true => Awd2ch18::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2ch18::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2ch18::B0x1
    }
}
#[doc = "Field `AWD2CH18` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd2ch18W<'a, REG> = crate::BitWriter<'a, REG, Awd2ch18>;
impl<'a, REG> Awd2ch18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch18::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2ch18::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch0(&self) -> Awd2ch0R {
        Awd2ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch1(&self) -> Awd2ch1R {
        Awd2ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch2(&self) -> Awd2ch2R {
        Awd2ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch3(&self) -> Awd2ch3R {
        Awd2ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch4(&self) -> Awd2ch4R {
        Awd2ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch5(&self) -> Awd2ch5R {
        Awd2ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch6(&self) -> Awd2ch6R {
        Awd2ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch7(&self) -> Awd2ch7R {
        Awd2ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch8(&self) -> Awd2ch8R {
        Awd2ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch9(&self) -> Awd2ch9R {
        Awd2ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch10(&self) -> Awd2ch10R {
        Awd2ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch11(&self) -> Awd2ch11R {
        Awd2ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch12(&self) -> Awd2ch12R {
        Awd2ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch13(&self) -> Awd2ch13R {
        Awd2ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch14(&self) -> Awd2ch14R {
        Awd2ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch15(&self) -> Awd2ch15R {
        Awd2ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch16(&self) -> Awd2ch16R {
        Awd2ch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch17(&self) -> Awd2ch17R {
        Awd2ch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ch18(&self) -> Awd2ch18R {
        Awd2ch18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch0(&mut self) -> Awd2ch0W<AdcAwd2crSpec> {
        Awd2ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch1(&mut self) -> Awd2ch1W<AdcAwd2crSpec> {
        Awd2ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch2(&mut self) -> Awd2ch2W<AdcAwd2crSpec> {
        Awd2ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch3(&mut self) -> Awd2ch3W<AdcAwd2crSpec> {
        Awd2ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch4(&mut self) -> Awd2ch4W<AdcAwd2crSpec> {
        Awd2ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch5(&mut self) -> Awd2ch5W<AdcAwd2crSpec> {
        Awd2ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch6(&mut self) -> Awd2ch6W<AdcAwd2crSpec> {
        Awd2ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch7(&mut self) -> Awd2ch7W<AdcAwd2crSpec> {
        Awd2ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch8(&mut self) -> Awd2ch8W<AdcAwd2crSpec> {
        Awd2ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch9(&mut self) -> Awd2ch9W<AdcAwd2crSpec> {
        Awd2ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch10(&mut self) -> Awd2ch10W<AdcAwd2crSpec> {
        Awd2ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch11(&mut self) -> Awd2ch11W<AdcAwd2crSpec> {
        Awd2ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch12(&mut self) -> Awd2ch12W<AdcAwd2crSpec> {
        Awd2ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch13(&mut self) -> Awd2ch13W<AdcAwd2crSpec> {
        Awd2ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch14(&mut self) -> Awd2ch14W<AdcAwd2crSpec> {
        Awd2ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch15(&mut self) -> Awd2ch15W<AdcAwd2crSpec> {
        Awd2ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch16(&mut self) -> Awd2ch16W<AdcAwd2crSpec> {
        Awd2ch16W::new(self, 16)
    }
    #[doc = "Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch17(&mut self) -> Awd2ch17W<AdcAwd2crSpec> {
        Awd2ch17W::new(self, 17)
    }
    #[doc = "Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch18(&mut self) -> Awd2ch18W<AdcAwd2crSpec> {
        Awd2ch18W::new(self, 18)
    }
}
#[doc = "ADC Analog Watchdog 2 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAwd2crSpec;
impl crate::RegisterSpec for AdcAwd2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_awd2cr::R`](R) reader structure"]
impl crate::Readable for AdcAwd2crSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_awd2cr::W`](W) writer structure"]
impl crate::Writable for AdcAwd2crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_AWD2CR to value 0"]
impl crate::Resettable for AdcAwd2crSpec {
    const RESET_VALUE: u32 = 0;
}
