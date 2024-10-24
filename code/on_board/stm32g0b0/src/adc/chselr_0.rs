#[doc = "Register `CHSELR_0` reader"]
pub type R = crate::R<Chselr0Spec>;
#[doc = "Register `CHSELR_0` writer"]
pub type W = crate::W<Chselr0Spec>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel0 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel0> for bool {
    #[inline(always)]
    fn from(variant: Chsel0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL0` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel0R = crate::BitReader<Chsel0>;
impl Chsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel0 {
        match self.bits {
            false => Chsel0::B0x0,
            true => Chsel0::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel0::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel0::B0x1
    }
}
#[doc = "Field `CHSEL0` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel0W<'a, REG> = crate::BitWriter<'a, REG, Chsel0>;
impl<'a, REG> Chsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel1 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel1> for bool {
    #[inline(always)]
    fn from(variant: Chsel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL1` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel1R = crate::BitReader<Chsel1>;
impl Chsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel1 {
        match self.bits {
            false => Chsel1::B0x0,
            true => Chsel1::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel1::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel1::B0x1
    }
}
#[doc = "Field `CHSEL1` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel1W<'a, REG> = crate::BitWriter<'a, REG, Chsel1>;
impl<'a, REG> Chsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel2 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel2> for bool {
    #[inline(always)]
    fn from(variant: Chsel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL2` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel2R = crate::BitReader<Chsel2>;
impl Chsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel2 {
        match self.bits {
            false => Chsel2::B0x0,
            true => Chsel2::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel2::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel2::B0x1
    }
}
#[doc = "Field `CHSEL2` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel2W<'a, REG> = crate::BitWriter<'a, REG, Chsel2>;
impl<'a, REG> Chsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel3 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel3> for bool {
    #[inline(always)]
    fn from(variant: Chsel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL3` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel3R = crate::BitReader<Chsel3>;
impl Chsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel3 {
        match self.bits {
            false => Chsel3::B0x0,
            true => Chsel3::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel3::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel3::B0x1
    }
}
#[doc = "Field `CHSEL3` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel3W<'a, REG> = crate::BitWriter<'a, REG, Chsel3>;
impl<'a, REG> Chsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel4 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel4> for bool {
    #[inline(always)]
    fn from(variant: Chsel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL4` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel4R = crate::BitReader<Chsel4>;
impl Chsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel4 {
        match self.bits {
            false => Chsel4::B0x0,
            true => Chsel4::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel4::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel4::B0x1
    }
}
#[doc = "Field `CHSEL4` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel4W<'a, REG> = crate::BitWriter<'a, REG, Chsel4>;
impl<'a, REG> Chsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel5 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel5> for bool {
    #[inline(always)]
    fn from(variant: Chsel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL5` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel5R = crate::BitReader<Chsel5>;
impl Chsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel5 {
        match self.bits {
            false => Chsel5::B0x0,
            true => Chsel5::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel5::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel5::B0x1
    }
}
#[doc = "Field `CHSEL5` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel5W<'a, REG> = crate::BitWriter<'a, REG, Chsel5>;
impl<'a, REG> Chsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel6 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel6> for bool {
    #[inline(always)]
    fn from(variant: Chsel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL6` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel6R = crate::BitReader<Chsel6>;
impl Chsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel6 {
        match self.bits {
            false => Chsel6::B0x0,
            true => Chsel6::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel6::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel6::B0x1
    }
}
#[doc = "Field `CHSEL6` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel6W<'a, REG> = crate::BitWriter<'a, REG, Chsel6>;
impl<'a, REG> Chsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel7 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel7> for bool {
    #[inline(always)]
    fn from(variant: Chsel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL7` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel7R = crate::BitReader<Chsel7>;
impl Chsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel7 {
        match self.bits {
            false => Chsel7::B0x0,
            true => Chsel7::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel7::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel7::B0x1
    }
}
#[doc = "Field `CHSEL7` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel7W<'a, REG> = crate::BitWriter<'a, REG, Chsel7>;
impl<'a, REG> Chsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel8 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel8> for bool {
    #[inline(always)]
    fn from(variant: Chsel8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL8` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel8R = crate::BitReader<Chsel8>;
impl Chsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel8 {
        match self.bits {
            false => Chsel8::B0x0,
            true => Chsel8::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel8::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel8::B0x1
    }
}
#[doc = "Field `CHSEL8` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel8W<'a, REG> = crate::BitWriter<'a, REG, Chsel8>;
impl<'a, REG> Chsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel8::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel8::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel9 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel9> for bool {
    #[inline(always)]
    fn from(variant: Chsel9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL9` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel9R = crate::BitReader<Chsel9>;
impl Chsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel9 {
        match self.bits {
            false => Chsel9::B0x0,
            true => Chsel9::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel9::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel9::B0x1
    }
}
#[doc = "Field `CHSEL9` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel9W<'a, REG> = crate::BitWriter<'a, REG, Chsel9>;
impl<'a, REG> Chsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel9::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel9::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel10 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel10> for bool {
    #[inline(always)]
    fn from(variant: Chsel10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL10` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel10R = crate::BitReader<Chsel10>;
impl Chsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel10 {
        match self.bits {
            false => Chsel10::B0x0,
            true => Chsel10::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel10::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel10::B0x1
    }
}
#[doc = "Field `CHSEL10` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel10W<'a, REG> = crate::BitWriter<'a, REG, Chsel10>;
impl<'a, REG> Chsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel10::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel10::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel11 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel11> for bool {
    #[inline(always)]
    fn from(variant: Chsel11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL11` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel11R = crate::BitReader<Chsel11>;
impl Chsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel11 {
        match self.bits {
            false => Chsel11::B0x0,
            true => Chsel11::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel11::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel11::B0x1
    }
}
#[doc = "Field `CHSEL11` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel11W<'a, REG> = crate::BitWriter<'a, REG, Chsel11>;
impl<'a, REG> Chsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel11::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel11::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel12 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel12> for bool {
    #[inline(always)]
    fn from(variant: Chsel12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL12` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel12R = crate::BitReader<Chsel12>;
impl Chsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel12 {
        match self.bits {
            false => Chsel12::B0x0,
            true => Chsel12::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel12::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel12::B0x1
    }
}
#[doc = "Field `CHSEL12` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel12W<'a, REG> = crate::BitWriter<'a, REG, Chsel12>;
impl<'a, REG> Chsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel12::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel12::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel13 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel13> for bool {
    #[inline(always)]
    fn from(variant: Chsel13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL13` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel13R = crate::BitReader<Chsel13>;
impl Chsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel13 {
        match self.bits {
            false => Chsel13::B0x0,
            true => Chsel13::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel13::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel13::B0x1
    }
}
#[doc = "Field `CHSEL13` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel13W<'a, REG> = crate::BitWriter<'a, REG, Chsel13>;
impl<'a, REG> Chsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel13::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel13::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel14 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel14> for bool {
    #[inline(always)]
    fn from(variant: Chsel14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL14` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel14R = crate::BitReader<Chsel14>;
impl Chsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel14 {
        match self.bits {
            false => Chsel14::B0x0,
            true => Chsel14::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel14::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel14::B0x1
    }
}
#[doc = "Field `CHSEL14` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel14W<'a, REG> = crate::BitWriter<'a, REG, Chsel14>;
impl<'a, REG> Chsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel14::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel14::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel15 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel15> for bool {
    #[inline(always)]
    fn from(variant: Chsel15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL15` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel15R = crate::BitReader<Chsel15>;
impl Chsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel15 {
        match self.bits {
            false => Chsel15::B0x0,
            true => Chsel15::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel15::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel15::B0x1
    }
}
#[doc = "Field `CHSEL15` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel15W<'a, REG> = crate::BitWriter<'a, REG, Chsel15>;
impl<'a, REG> Chsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel15::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel15::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel16 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel16> for bool {
    #[inline(always)]
    fn from(variant: Chsel16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL16` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel16R = crate::BitReader<Chsel16>;
impl Chsel16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel16 {
        match self.bits {
            false => Chsel16::B0x0,
            true => Chsel16::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel16::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel16::B0x1
    }
}
#[doc = "Field `CHSEL16` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel16W<'a, REG> = crate::BitWriter<'a, REG, Chsel16>;
impl<'a, REG> Chsel16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel16::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel16::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel17 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel17> for bool {
    #[inline(always)]
    fn from(variant: Chsel17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL17` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel17R = crate::BitReader<Chsel17>;
impl Chsel17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel17 {
        match self.bits {
            false => Chsel17::B0x0,
            true => Chsel17::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel17::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel17::B0x1
    }
}
#[doc = "Field `CHSEL17` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel17W<'a, REG> = crate::BitWriter<'a, REG, Chsel17>;
impl<'a, REG> Chsel17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel17::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel17::B0x1)
    }
}
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel18 {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B0x0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B0x1 = 1,
}
impl From<Chsel18> for bool {
    #[inline(always)]
    fn from(variant: Chsel18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL18` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel18R = crate::BitReader<Chsel18>;
impl Chsel18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel18 {
        match self.bits {
            false => Chsel18::B0x0,
            true => Chsel18::B0x1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chsel18::B0x0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chsel18::B0x1
    }
}
#[doc = "Field `CHSEL18` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type Chsel18W<'a, REG> = crate::BitWriter<'a, REG, Chsel18>;
impl<'a, REG> Chsel18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel18::B0x0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel18::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel0(&self) -> Chsel0R {
        Chsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel1(&self) -> Chsel1R {
        Chsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel2(&self) -> Chsel2R {
        Chsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel3(&self) -> Chsel3R {
        Chsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel4(&self) -> Chsel4R {
        Chsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel5(&self) -> Chsel5R {
        Chsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel6(&self) -> Chsel6R {
        Chsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel7(&self) -> Chsel7R {
        Chsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel8(&self) -> Chsel8R {
        Chsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel9(&self) -> Chsel9R {
        Chsel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel10(&self) -> Chsel10R {
        Chsel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel11(&self) -> Chsel11R {
        Chsel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel12(&self) -> Chsel12R {
        Chsel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel13(&self) -> Chsel13R {
        Chsel13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel14(&self) -> Chsel14R {
        Chsel14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel15(&self) -> Chsel15R {
        Chsel15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel16(&self) -> Chsel16R {
        Chsel16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel17(&self) -> Chsel17R {
        Chsel17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel18(&self) -> Chsel18R {
        Chsel18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> Chsel0W<Chselr0Spec> {
        Chsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> Chsel1W<Chselr0Spec> {
        Chsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> Chsel2W<Chselr0Spec> {
        Chsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> Chsel3W<Chselr0Spec> {
        Chsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> Chsel4W<Chselr0Spec> {
        Chsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> Chsel5W<Chselr0Spec> {
        Chsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> Chsel6W<Chselr0Spec> {
        Chsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> Chsel7W<Chselr0Spec> {
        Chsel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> Chsel8W<Chselr0Spec> {
        Chsel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel9(&mut self) -> Chsel9W<Chselr0Spec> {
        Chsel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel10(&mut self) -> Chsel10W<Chselr0Spec> {
        Chsel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel11(&mut self) -> Chsel11W<Chselr0Spec> {
        Chsel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel12(&mut self) -> Chsel12W<Chselr0Spec> {
        Chsel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel13(&mut self) -> Chsel13W<Chselr0Spec> {
        Chsel13W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel14(&mut self) -> Chsel14W<Chselr0Spec> {
        Chsel14W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel15(&mut self) -> Chsel15W<Chselr0Spec> {
        Chsel15W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel16(&mut self) -> Chsel16W<Chselr0Spec> {
        Chsel16W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel17(&mut self) -> Chsel17W<Chselr0Spec> {
        Chsel17W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel18(&mut self) -> Chsel18W<Chselr0Spec> {
        Chsel18W::new(self, 18)
    }
}
#[doc = "ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`chselr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chselr0Spec;
impl crate::RegisterSpec for Chselr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr_0::R`](R) reader structure"]
impl crate::Readable for Chselr0Spec {}
#[doc = "`write(|w| ..)` method takes [`chselr_0::W`](W) writer structure"]
impl crate::Writable for Chselr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSELR_0 to value 0"]
impl crate::Resettable for Chselr0Spec {
    const RESET_VALUE: u32 = 0;
}
