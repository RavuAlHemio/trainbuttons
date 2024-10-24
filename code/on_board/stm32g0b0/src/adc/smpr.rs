#[doc = "Register `SMPR` reader"]
pub type R = crate::R<SmprSpec>;
#[doc = "Register `SMPR` writer"]
pub type W = crate::W<SmprSpec>;
#[doc = "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smp1 {
    #[doc = "0: 1.5 ADC clock cycles"]
    B0x0 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    B0x1 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    B0x2 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    B0x3 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    B0x4 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    B0x5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    B0x6 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    B0x7 = 7,
}
impl From<Smp1> for u8 {
    #[inline(always)]
    fn from(variant: Smp1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smp1 {
    type Ux = u8;
}
impl crate::IsEnum for Smp1 {}
#[doc = "Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smp1R = crate::FieldReader<Smp1>;
impl Smp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smp1 {
        match self.bits {
            0 => Smp1::B0x0,
            1 => Smp1::B0x1,
            2 => Smp1::B0x2,
            3 => Smp1::B0x3,
            4 => Smp1::B0x4,
            5 => Smp1::B0x5,
            6 => Smp1::B0x6,
            7 => Smp1::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smp1::B0x0
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smp1::B0x1
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Smp1::B0x2
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Smp1::B0x3
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Smp1::B0x4
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Smp1::B0x5
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Smp1::B0x6
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Smp1::B0x7
    }
}
#[doc = "Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smp1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Smp1, crate::Safe>;
impl<'a, REG> Smp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x0)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x1)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x2)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x3)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x4)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x6)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Smp1::B0x7)
    }
}
#[doc = "Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smp2 {
    #[doc = "0: 1.5 ADC clock cycles"]
    B0x0 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    B0x1 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    B0x2 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    B0x3 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    B0x4 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    B0x5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    B0x6 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    B0x7 = 7,
}
impl From<Smp2> for u8 {
    #[inline(always)]
    fn from(variant: Smp2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smp2 {
    type Ux = u8;
}
impl crate::IsEnum for Smp2 {}
#[doc = "Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smp2R = crate::FieldReader<Smp2>;
impl Smp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smp2 {
        match self.bits {
            0 => Smp2::B0x0,
            1 => Smp2::B0x1,
            2 => Smp2::B0x2,
            3 => Smp2::B0x3,
            4 => Smp2::B0x4,
            5 => Smp2::B0x5,
            6 => Smp2::B0x6,
            7 => Smp2::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smp2::B0x0
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smp2::B0x1
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Smp2::B0x2
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Smp2::B0x3
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Smp2::B0x4
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Smp2::B0x5
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Smp2::B0x6
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Smp2::B0x7
    }
}
#[doc = "Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smp2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Smp2, crate::Safe>;
impl<'a, REG> Smp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x0)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x1)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x2)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x3)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x4)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x6)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Smp2::B0x7)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel0 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel0> for bool {
    #[inline(always)]
    fn from(variant: Smpsel0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL0` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel0R = crate::BitReader<Smpsel0>;
impl Smpsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel0 {
        match self.bits {
            false => Smpsel0::B0x0,
            true => Smpsel0::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel0::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel0::B0x1
    }
}
#[doc = "Field `SMPSEL0` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel0W<'a, REG> = crate::BitWriter<'a, REG, Smpsel0>;
impl<'a, REG> Smpsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel0::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel0::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel1 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel1> for bool {
    #[inline(always)]
    fn from(variant: Smpsel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL1` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel1R = crate::BitReader<Smpsel1>;
impl Smpsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel1 {
        match self.bits {
            false => Smpsel1::B0x0,
            true => Smpsel1::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel1::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel1::B0x1
    }
}
#[doc = "Field `SMPSEL1` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel1W<'a, REG> = crate::BitWriter<'a, REG, Smpsel1>;
impl<'a, REG> Smpsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel1::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel1::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel2 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel2> for bool {
    #[inline(always)]
    fn from(variant: Smpsel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL2` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel2R = crate::BitReader<Smpsel2>;
impl Smpsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel2 {
        match self.bits {
            false => Smpsel2::B0x0,
            true => Smpsel2::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel2::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel2::B0x1
    }
}
#[doc = "Field `SMPSEL2` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel2W<'a, REG> = crate::BitWriter<'a, REG, Smpsel2>;
impl<'a, REG> Smpsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel2::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel2::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel3 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel3> for bool {
    #[inline(always)]
    fn from(variant: Smpsel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL3` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel3R = crate::BitReader<Smpsel3>;
impl Smpsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel3 {
        match self.bits {
            false => Smpsel3::B0x0,
            true => Smpsel3::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel3::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel3::B0x1
    }
}
#[doc = "Field `SMPSEL3` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel3W<'a, REG> = crate::BitWriter<'a, REG, Smpsel3>;
impl<'a, REG> Smpsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel3::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel3::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel4 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel4> for bool {
    #[inline(always)]
    fn from(variant: Smpsel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL4` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel4R = crate::BitReader<Smpsel4>;
impl Smpsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel4 {
        match self.bits {
            false => Smpsel4::B0x0,
            true => Smpsel4::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel4::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel4::B0x1
    }
}
#[doc = "Field `SMPSEL4` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel4W<'a, REG> = crate::BitWriter<'a, REG, Smpsel4>;
impl<'a, REG> Smpsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel4::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel4::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel5 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel5> for bool {
    #[inline(always)]
    fn from(variant: Smpsel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL5` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel5R = crate::BitReader<Smpsel5>;
impl Smpsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel5 {
        match self.bits {
            false => Smpsel5::B0x0,
            true => Smpsel5::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel5::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel5::B0x1
    }
}
#[doc = "Field `SMPSEL5` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel5W<'a, REG> = crate::BitWriter<'a, REG, Smpsel5>;
impl<'a, REG> Smpsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel5::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel5::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel6 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel6> for bool {
    #[inline(always)]
    fn from(variant: Smpsel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL6` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel6R = crate::BitReader<Smpsel6>;
impl Smpsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel6 {
        match self.bits {
            false => Smpsel6::B0x0,
            true => Smpsel6::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel6::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel6::B0x1
    }
}
#[doc = "Field `SMPSEL6` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel6W<'a, REG> = crate::BitWriter<'a, REG, Smpsel6>;
impl<'a, REG> Smpsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel6::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel6::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel7 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel7> for bool {
    #[inline(always)]
    fn from(variant: Smpsel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL7` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel7R = crate::BitReader<Smpsel7>;
impl Smpsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel7 {
        match self.bits {
            false => Smpsel7::B0x0,
            true => Smpsel7::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel7::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel7::B0x1
    }
}
#[doc = "Field `SMPSEL7` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel7W<'a, REG> = crate::BitWriter<'a, REG, Smpsel7>;
impl<'a, REG> Smpsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel7::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel7::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel8 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel8> for bool {
    #[inline(always)]
    fn from(variant: Smpsel8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL8` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel8R = crate::BitReader<Smpsel8>;
impl Smpsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel8 {
        match self.bits {
            false => Smpsel8::B0x0,
            true => Smpsel8::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel8::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel8::B0x1
    }
}
#[doc = "Field `SMPSEL8` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel8W<'a, REG> = crate::BitWriter<'a, REG, Smpsel8>;
impl<'a, REG> Smpsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel8::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel8::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel9 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel9> for bool {
    #[inline(always)]
    fn from(variant: Smpsel9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL9` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel9R = crate::BitReader<Smpsel9>;
impl Smpsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel9 {
        match self.bits {
            false => Smpsel9::B0x0,
            true => Smpsel9::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel9::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel9::B0x1
    }
}
#[doc = "Field `SMPSEL9` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel9W<'a, REG> = crate::BitWriter<'a, REG, Smpsel9>;
impl<'a, REG> Smpsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel9::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel9::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel10 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel10> for bool {
    #[inline(always)]
    fn from(variant: Smpsel10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL10` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel10R = crate::BitReader<Smpsel10>;
impl Smpsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel10 {
        match self.bits {
            false => Smpsel10::B0x0,
            true => Smpsel10::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel10::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel10::B0x1
    }
}
#[doc = "Field `SMPSEL10` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel10W<'a, REG> = crate::BitWriter<'a, REG, Smpsel10>;
impl<'a, REG> Smpsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel10::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel10::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel11 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel11> for bool {
    #[inline(always)]
    fn from(variant: Smpsel11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL11` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel11R = crate::BitReader<Smpsel11>;
impl Smpsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel11 {
        match self.bits {
            false => Smpsel11::B0x0,
            true => Smpsel11::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel11::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel11::B0x1
    }
}
#[doc = "Field `SMPSEL11` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel11W<'a, REG> = crate::BitWriter<'a, REG, Smpsel11>;
impl<'a, REG> Smpsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel11::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel11::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel12 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel12> for bool {
    #[inline(always)]
    fn from(variant: Smpsel12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL12` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel12R = crate::BitReader<Smpsel12>;
impl Smpsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel12 {
        match self.bits {
            false => Smpsel12::B0x0,
            true => Smpsel12::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel12::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel12::B0x1
    }
}
#[doc = "Field `SMPSEL12` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel12W<'a, REG> = crate::BitWriter<'a, REG, Smpsel12>;
impl<'a, REG> Smpsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel12::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel12::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel13 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel13> for bool {
    #[inline(always)]
    fn from(variant: Smpsel13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL13` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel13R = crate::BitReader<Smpsel13>;
impl Smpsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel13 {
        match self.bits {
            false => Smpsel13::B0x0,
            true => Smpsel13::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel13::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel13::B0x1
    }
}
#[doc = "Field `SMPSEL13` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel13W<'a, REG> = crate::BitWriter<'a, REG, Smpsel13>;
impl<'a, REG> Smpsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel13::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel13::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel14 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel14> for bool {
    #[inline(always)]
    fn from(variant: Smpsel14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL14` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel14R = crate::BitReader<Smpsel14>;
impl Smpsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel14 {
        match self.bits {
            false => Smpsel14::B0x0,
            true => Smpsel14::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel14::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel14::B0x1
    }
}
#[doc = "Field `SMPSEL14` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel14W<'a, REG> = crate::BitWriter<'a, REG, Smpsel14>;
impl<'a, REG> Smpsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel14::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel14::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel15 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel15> for bool {
    #[inline(always)]
    fn from(variant: Smpsel15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL15` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel15R = crate::BitReader<Smpsel15>;
impl Smpsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel15 {
        match self.bits {
            false => Smpsel15::B0x0,
            true => Smpsel15::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel15::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel15::B0x1
    }
}
#[doc = "Field `SMPSEL15` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel15W<'a, REG> = crate::BitWriter<'a, REG, Smpsel15>;
impl<'a, REG> Smpsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel15::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel15::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel16 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel16> for bool {
    #[inline(always)]
    fn from(variant: Smpsel16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL16` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel16R = crate::BitReader<Smpsel16>;
impl Smpsel16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel16 {
        match self.bits {
            false => Smpsel16::B0x0,
            true => Smpsel16::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel16::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel16::B0x1
    }
}
#[doc = "Field `SMPSEL16` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel16W<'a, REG> = crate::BitWriter<'a, REG, Smpsel16>;
impl<'a, REG> Smpsel16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel16::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel16::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel17 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel17> for bool {
    #[inline(always)]
    fn from(variant: Smpsel17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL17` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel17R = crate::BitReader<Smpsel17>;
impl Smpsel17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel17 {
        match self.bits {
            false => Smpsel17::B0x0,
            true => Smpsel17::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel17::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel17::B0x1
    }
}
#[doc = "Field `SMPSEL17` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel17W<'a, REG> = crate::BitWriter<'a, REG, Smpsel17>;
impl<'a, REG> Smpsel17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel17::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel17::B0x1)
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpsel18 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B0x0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B0x1 = 1,
}
impl From<Smpsel18> for bool {
    #[inline(always)]
    fn from(variant: Smpsel18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL18` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel18R = crate::BitReader<Smpsel18>;
impl Smpsel18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpsel18 {
        match self.bits {
            false => Smpsel18::B0x0,
            true => Smpsel18::B0x1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smpsel18::B0x0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smpsel18::B0x1
    }
}
#[doc = "Field `SMPSEL18` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Smpsel18W<'a, REG> = crate::BitWriter<'a, REG, Smpsel18>;
impl<'a, REG> Smpsel18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel18::B0x0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpsel18::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp1(&self) -> Smp1R {
        Smp1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp2(&self) -> Smp2R {
        Smp2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel0(&self) -> Smpsel0R {
        Smpsel0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel1(&self) -> Smpsel1R {
        Smpsel1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel2(&self) -> Smpsel2R {
        Smpsel2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel3(&self) -> Smpsel3R {
        Smpsel3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel4(&self) -> Smpsel4R {
        Smpsel4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel5(&self) -> Smpsel5R {
        Smpsel5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel6(&self) -> Smpsel6R {
        Smpsel6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel7(&self) -> Smpsel7R {
        Smpsel7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel8(&self) -> Smpsel8R {
        Smpsel8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel9(&self) -> Smpsel9R {
        Smpsel9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel10(&self) -> Smpsel10R {
        Smpsel10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel11(&self) -> Smpsel11R {
        Smpsel11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel12(&self) -> Smpsel12R {
        Smpsel12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel13(&self) -> Smpsel13R {
        Smpsel13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel14(&self) -> Smpsel14R {
        Smpsel14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel15(&self) -> Smpsel15R {
        Smpsel15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel16(&self) -> Smpsel16R {
        Smpsel16R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel17(&self) -> Smpsel17R {
        Smpsel17R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel18(&self) -> Smpsel18R {
        Smpsel18R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> Smp1W<SmprSpec> {
        Smp1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> Smp2W<SmprSpec> {
        Smp2W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> Smpsel0W<SmprSpec> {
        Smpsel0W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> Smpsel1W<SmprSpec> {
        Smpsel1W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> Smpsel2W<SmprSpec> {
        Smpsel2W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> Smpsel3W<SmprSpec> {
        Smpsel3W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> Smpsel4W<SmprSpec> {
        Smpsel4W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> Smpsel5W<SmprSpec> {
        Smpsel5W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> Smpsel6W<SmprSpec> {
        Smpsel6W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> Smpsel7W<SmprSpec> {
        Smpsel7W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> Smpsel8W<SmprSpec> {
        Smpsel8W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> Smpsel9W<SmprSpec> {
        Smpsel9W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> Smpsel10W<SmprSpec> {
        Smpsel10W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> Smpsel11W<SmprSpec> {
        Smpsel11W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> Smpsel12W<SmprSpec> {
        Smpsel12W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> Smpsel13W<SmprSpec> {
        Smpsel13W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> Smpsel14W<SmprSpec> {
        Smpsel14W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> Smpsel15W<SmprSpec> {
        Smpsel15W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> Smpsel16W<SmprSpec> {
        Smpsel16W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> Smpsel17W<SmprSpec> {
        Smpsel17W::new(self, 25)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> Smpsel18W<SmprSpec> {
        Smpsel18W::new(self, 26)
    }
}
#[doc = "ADC sampling time register\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmprSpec;
impl crate::RegisterSpec for SmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr::R`](R) reader structure"]
impl crate::Readable for SmprSpec {}
#[doc = "`write(|w| ..)` method takes [`smpr::W`](W) writer structure"]
impl crate::Writable for SmprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SmprSpec {
    const RESET_VALUE: u32 = 0;
}
