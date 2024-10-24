#[doc = "Register `AWD3CR` reader"]
pub type R = crate::R<Awd3crSpec>;
#[doc = "Register `AWD3CR` writer"]
pub type W = crate::W<Awd3crSpec>;
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch0 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch0> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH0` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch0R = crate::BitReader<Awd3ch0>;
impl Awd3ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch0 {
        match self.bits {
            false => Awd3ch0::B0x0,
            true => Awd3ch0::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch0::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch0::B0x1
    }
}
#[doc = "Field `AWD3CH0` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch0W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch0>;
impl<'a, REG> Awd3ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch0::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch0::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch1 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch1> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH1` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch1R = crate::BitReader<Awd3ch1>;
impl Awd3ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch1 {
        match self.bits {
            false => Awd3ch1::B0x0,
            true => Awd3ch1::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch1::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch1::B0x1
    }
}
#[doc = "Field `AWD3CH1` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch1W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch1>;
impl<'a, REG> Awd3ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch1::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch1::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch2 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch2> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH2` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch2R = crate::BitReader<Awd3ch2>;
impl Awd3ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch2 {
        match self.bits {
            false => Awd3ch2::B0x0,
            true => Awd3ch2::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch2::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch2::B0x1
    }
}
#[doc = "Field `AWD3CH2` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch2W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch2>;
impl<'a, REG> Awd3ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch2::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch2::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch3 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch3> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH3` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch3R = crate::BitReader<Awd3ch3>;
impl Awd3ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch3 {
        match self.bits {
            false => Awd3ch3::B0x0,
            true => Awd3ch3::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch3::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch3::B0x1
    }
}
#[doc = "Field `AWD3CH3` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch3W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch3>;
impl<'a, REG> Awd3ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch3::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch3::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch4 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch4> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH4` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch4R = crate::BitReader<Awd3ch4>;
impl Awd3ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch4 {
        match self.bits {
            false => Awd3ch4::B0x0,
            true => Awd3ch4::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch4::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch4::B0x1
    }
}
#[doc = "Field `AWD3CH4` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch4W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch4>;
impl<'a, REG> Awd3ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch4::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch4::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch5 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch5> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH5` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch5R = crate::BitReader<Awd3ch5>;
impl Awd3ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch5 {
        match self.bits {
            false => Awd3ch5::B0x0,
            true => Awd3ch5::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch5::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch5::B0x1
    }
}
#[doc = "Field `AWD3CH5` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch5W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch5>;
impl<'a, REG> Awd3ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch5::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch5::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch6 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch6> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH6` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch6R = crate::BitReader<Awd3ch6>;
impl Awd3ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch6 {
        match self.bits {
            false => Awd3ch6::B0x0,
            true => Awd3ch6::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch6::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch6::B0x1
    }
}
#[doc = "Field `AWD3CH6` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch6W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch6>;
impl<'a, REG> Awd3ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch6::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch6::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch7 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch7> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH7` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch7R = crate::BitReader<Awd3ch7>;
impl Awd3ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch7 {
        match self.bits {
            false => Awd3ch7::B0x0,
            true => Awd3ch7::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch7::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch7::B0x1
    }
}
#[doc = "Field `AWD3CH7` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch7W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch7>;
impl<'a, REG> Awd3ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch7::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch7::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch8 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch8> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH8` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch8R = crate::BitReader<Awd3ch8>;
impl Awd3ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch8 {
        match self.bits {
            false => Awd3ch8::B0x0,
            true => Awd3ch8::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch8::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch8::B0x1
    }
}
#[doc = "Field `AWD3CH8` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch8W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch8>;
impl<'a, REG> Awd3ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch8::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch8::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch9 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch9> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH9` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch9R = crate::BitReader<Awd3ch9>;
impl Awd3ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch9 {
        match self.bits {
            false => Awd3ch9::B0x0,
            true => Awd3ch9::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch9::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch9::B0x1
    }
}
#[doc = "Field `AWD3CH9` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch9W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch9>;
impl<'a, REG> Awd3ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch9::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch9::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch10 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch10> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH10` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch10R = crate::BitReader<Awd3ch10>;
impl Awd3ch10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch10 {
        match self.bits {
            false => Awd3ch10::B0x0,
            true => Awd3ch10::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch10::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch10::B0x1
    }
}
#[doc = "Field `AWD3CH10` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch10W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch10>;
impl<'a, REG> Awd3ch10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch10::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch10::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch11 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch11> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH11` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch11R = crate::BitReader<Awd3ch11>;
impl Awd3ch11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch11 {
        match self.bits {
            false => Awd3ch11::B0x0,
            true => Awd3ch11::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch11::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch11::B0x1
    }
}
#[doc = "Field `AWD3CH11` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch11W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch11>;
impl<'a, REG> Awd3ch11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch11::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch11::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch12 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch12> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH12` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch12R = crate::BitReader<Awd3ch12>;
impl Awd3ch12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch12 {
        match self.bits {
            false => Awd3ch12::B0x0,
            true => Awd3ch12::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch12::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch12::B0x1
    }
}
#[doc = "Field `AWD3CH12` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch12W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch12>;
impl<'a, REG> Awd3ch12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch12::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch12::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch13 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch13> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH13` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch13R = crate::BitReader<Awd3ch13>;
impl Awd3ch13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch13 {
        match self.bits {
            false => Awd3ch13::B0x0,
            true => Awd3ch13::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch13::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch13::B0x1
    }
}
#[doc = "Field `AWD3CH13` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch13W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch13>;
impl<'a, REG> Awd3ch13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch13::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch13::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch14 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch14> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH14` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch14R = crate::BitReader<Awd3ch14>;
impl Awd3ch14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch14 {
        match self.bits {
            false => Awd3ch14::B0x0,
            true => Awd3ch14::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch14::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch14::B0x1
    }
}
#[doc = "Field `AWD3CH14` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch14W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch14>;
impl<'a, REG> Awd3ch14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch14::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch14::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch15 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch15> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH15` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch15R = crate::BitReader<Awd3ch15>;
impl Awd3ch15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch15 {
        match self.bits {
            false => Awd3ch15::B0x0,
            true => Awd3ch15::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch15::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch15::B0x1
    }
}
#[doc = "Field `AWD3CH15` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch15W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch15>;
impl<'a, REG> Awd3ch15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch15::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch15::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch16 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch16> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH16` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch16R = crate::BitReader<Awd3ch16>;
impl Awd3ch16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch16 {
        match self.bits {
            false => Awd3ch16::B0x0,
            true => Awd3ch16::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch16::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch16::B0x1
    }
}
#[doc = "Field `AWD3CH16` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch16W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch16>;
impl<'a, REG> Awd3ch16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch16::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch16::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch17 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch17> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH17` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch17R = crate::BitReader<Awd3ch17>;
impl Awd3ch17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch17 {
        match self.bits {
            false => Awd3ch17::B0x0,
            true => Awd3ch17::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch17::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch17::B0x1
    }
}
#[doc = "Field `AWD3CH17` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch17W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch17>;
impl<'a, REG> Awd3ch17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch17::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch17::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3ch18 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    B0x0 = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    B0x1 = 1,
}
impl From<Awd3ch18> for bool {
    #[inline(always)]
    fn from(variant: Awd3ch18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH18` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch18R = crate::BitReader<Awd3ch18>;
impl Awd3ch18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3ch18 {
        match self.bits {
            false => Awd3ch18::B0x0,
            true => Awd3ch18::B0x1,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3ch18::B0x0
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3ch18::B0x1
    }
}
#[doc = "Field `AWD3CH18` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Awd3ch18W<'a, REG> = crate::BitWriter<'a, REG, Awd3ch18>;
impl<'a, REG> Awd3ch18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch18::B0x0)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3ch18::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch0(&self) -> Awd3ch0R {
        Awd3ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch1(&self) -> Awd3ch1R {
        Awd3ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch2(&self) -> Awd3ch2R {
        Awd3ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch3(&self) -> Awd3ch3R {
        Awd3ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch4(&self) -> Awd3ch4R {
        Awd3ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch5(&self) -> Awd3ch5R {
        Awd3ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch6(&self) -> Awd3ch6R {
        Awd3ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch7(&self) -> Awd3ch7R {
        Awd3ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch8(&self) -> Awd3ch8R {
        Awd3ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch9(&self) -> Awd3ch9R {
        Awd3ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch10(&self) -> Awd3ch10R {
        Awd3ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch11(&self) -> Awd3ch11R {
        Awd3ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch12(&self) -> Awd3ch12R {
        Awd3ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch13(&self) -> Awd3ch13R {
        Awd3ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch14(&self) -> Awd3ch14R {
        Awd3ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch15(&self) -> Awd3ch15R {
        Awd3ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch16(&self) -> Awd3ch16R {
        Awd3ch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch17(&self) -> Awd3ch17R {
        Awd3ch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ch18(&self) -> Awd3ch18R {
        Awd3ch18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch0(&mut self) -> Awd3ch0W<Awd3crSpec> {
        Awd3ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch1(&mut self) -> Awd3ch1W<Awd3crSpec> {
        Awd3ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch2(&mut self) -> Awd3ch2W<Awd3crSpec> {
        Awd3ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch3(&mut self) -> Awd3ch3W<Awd3crSpec> {
        Awd3ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch4(&mut self) -> Awd3ch4W<Awd3crSpec> {
        Awd3ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch5(&mut self) -> Awd3ch5W<Awd3crSpec> {
        Awd3ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch6(&mut self) -> Awd3ch6W<Awd3crSpec> {
        Awd3ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch7(&mut self) -> Awd3ch7W<Awd3crSpec> {
        Awd3ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch8(&mut self) -> Awd3ch8W<Awd3crSpec> {
        Awd3ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch9(&mut self) -> Awd3ch9W<Awd3crSpec> {
        Awd3ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch10(&mut self) -> Awd3ch10W<Awd3crSpec> {
        Awd3ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch11(&mut self) -> Awd3ch11W<Awd3crSpec> {
        Awd3ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch12(&mut self) -> Awd3ch12W<Awd3crSpec> {
        Awd3ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch13(&mut self) -> Awd3ch13W<Awd3crSpec> {
        Awd3ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch14(&mut self) -> Awd3ch14W<Awd3crSpec> {
        Awd3ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch15(&mut self) -> Awd3ch15W<Awd3crSpec> {
        Awd3ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch16(&mut self) -> Awd3ch16W<Awd3crSpec> {
        Awd3ch16W::new(self, 16)
    }
    #[doc = "Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch17(&mut self) -> Awd3ch17W<Awd3crSpec> {
        Awd3ch17W::new(self, 17)
    }
    #[doc = "Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch18(&mut self) -> Awd3ch18W<Awd3crSpec> {
        Awd3ch18W::new(self, 18)
    }
}
#[doc = "ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd3crSpec;
impl crate::RegisterSpec for Awd3crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3cr::R`](R) reader structure"]
impl crate::Readable for Awd3crSpec {}
#[doc = "`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure"]
impl crate::Writable for Awd3crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for Awd3crSpec {
    const RESET_VALUE: u32 = 0;
}
