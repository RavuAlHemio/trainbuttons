#[doc = "Register `ADC_CFGR2` reader"]
pub type R = crate::R<AdcCfgr2Spec>;
#[doc = "Register `ADC_CFGR2` writer"]
pub type W = crate::W<AdcCfgr2Spec>;
#[doc = "Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovse {
    #[doc = "0: Oversampler disabled"]
    B0x0 = 0,
    #[doc = "1: Oversampler enabled"]
    B0x1 = 1,
}
impl From<Ovse> for bool {
    #[inline(always)]
    fn from(variant: Ovse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OvseR = crate::BitReader<Ovse>;
impl OvseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovse {
        match self.bits {
            false => Ovse::B0x0,
            true => Ovse::B0x1,
        }
    }
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovse::B0x0
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovse::B0x1
    }
}
#[doc = "Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OvseW<'a, REG> = crate::BitWriter<'a, REG, Ovse>;
impl<'a, REG> OvseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovse::B0x0)
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovse::B0x1)
    }
}
#[doc = "Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovsr {
    #[doc = "0: 2x"]
    B0x0 = 0,
    #[doc = "1: 4x"]
    B0x1 = 1,
    #[doc = "2: 8x"]
    B0x2 = 2,
    #[doc = "3: 16x"]
    B0x3 = 3,
    #[doc = "4: 32x"]
    B0x4 = 4,
    #[doc = "5: 64x"]
    B0x5 = 5,
    #[doc = "6: 128x"]
    B0x6 = 6,
    #[doc = "7: 256x"]
    B0x7 = 7,
}
impl From<Ovsr> for u8 {
    #[inline(always)]
    fn from(variant: Ovsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovsr {
    type Ux = u8;
}
impl crate::IsEnum for Ovsr {}
#[doc = "Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OvsrR = crate::FieldReader<Ovsr>;
impl OvsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovsr {
        match self.bits {
            0 => Ovsr::B0x0,
            1 => Ovsr::B0x1,
            2 => Ovsr::B0x2,
            3 => Ovsr::B0x3,
            4 => Ovsr::B0x4,
            5 => Ovsr::B0x5,
            6 => Ovsr::B0x6,
            7 => Ovsr::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovsr::B0x0
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovsr::B0x1
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ovsr::B0x2
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ovsr::B0x3
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ovsr::B0x4
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ovsr::B0x5
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ovsr::B0x6
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ovsr::B0x7
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OvsrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ovsr, crate::Safe>;
impl<'a, REG> OvsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x0)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x1)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x2)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x3)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x4)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x5)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x6)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::B0x7)
    }
}
#[doc = "Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovss {
    #[doc = "0: No shift"]
    B0x0 = 0,
    #[doc = "1: Shift 1-bit"]
    B0x1 = 1,
    #[doc = "2: Shift 2-bits"]
    B0x2 = 2,
    #[doc = "3: Shift 3-bits"]
    B0x3 = 3,
    #[doc = "4: Shift 4-bits"]
    B0x4 = 4,
    #[doc = "5: Shift 5-bits"]
    B0x5 = 5,
    #[doc = "6: Shift 6-bits"]
    B0x6 = 6,
    #[doc = "7: Shift 7-bits"]
    B0x7 = 7,
    #[doc = "8: Shift 8-bits"]
    B0x8 = 8,
}
impl From<Ovss> for u8 {
    #[inline(always)]
    fn from(variant: Ovss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovss {
    type Ux = u8;
}
impl crate::IsEnum for Ovss {}
#[doc = "Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OvssR = crate::FieldReader<Ovss>;
impl OvssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ovss> {
        match self.bits {
            0 => Some(Ovss::B0x0),
            1 => Some(Ovss::B0x1),
            2 => Some(Ovss::B0x2),
            3 => Some(Ovss::B0x3),
            4 => Some(Ovss::B0x4),
            5 => Some(Ovss::B0x5),
            6 => Some(Ovss::B0x6),
            7 => Some(Ovss::B0x7),
            8 => Some(Ovss::B0x8),
            _ => None,
        }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovss::B0x0
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovss::B0x1
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ovss::B0x2
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ovss::B0x3
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ovss::B0x4
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ovss::B0x5
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ovss::B0x6
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ovss::B0x7
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Ovss::B0x8
    }
}
#[doc = "Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OvssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ovss>;
impl<'a, REG> OvssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No shift"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x0)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x1)
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x2)
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x3)
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x4)
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x5)
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x6)
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x7)
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ovss::B0x8)
    }
}
#[doc = "Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tovs {
    #[doc = "0: All oversampled conversions for a channel are done consecutively after a trigger"]
    B0x0 = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a trigger"]
    B0x1 = 1,
}
impl From<Tovs> for bool {
    #[inline(always)]
    fn from(variant: Tovs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TovsR = crate::BitReader<Tovs>;
impl TovsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tovs {
        match self.bits {
            false => Tovs::B0x0,
            true => Tovs::B0x1,
        }
    }
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tovs::B0x0
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tovs::B0x1
    }
}
#[doc = "Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TovsW<'a, REG> = crate::BitWriter<'a, REG, Tovs>;
impl<'a, REG> TovsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tovs::B0x0)
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tovs::B0x1)
    }
}
#[doc = "Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lftrig {
    #[doc = "0: Low Frequency Trigger Mode disabled"]
    B0x0 = 0,
    #[doc = "1: Low Frequency Trigger Mode enabled"]
    B0x1 = 1,
}
impl From<Lftrig> for bool {
    #[inline(always)]
    fn from(variant: Lftrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFTRIG` reader - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type LftrigR = crate::BitReader<Lftrig>;
impl LftrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lftrig {
        match self.bits {
            false => Lftrig::B0x0,
            true => Lftrig::B0x1,
        }
    }
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lftrig::B0x0
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lftrig::B0x1
    }
}
#[doc = "Field `LFTRIG` writer - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type LftrigW<'a, REG> = crate::BitWriter<'a, REG, Lftrig>;
impl<'a, REG> LftrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lftrig::B0x0)
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lftrig::B0x1)
    }
}
#[doc = "ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckmode {
    #[doc = "0: ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    Adcclk = 0,
    #[doc = "1: PCLK/2 (Synchronous clock mode)"]
    PclkBy2 = 1,
    #[doc = "2: PCLK/4 (Synchronous clock mode)"]
    PclkBy4 = 2,
    #[doc = "3: PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    Pclk = 3,
}
impl From<Ckmode> for u8 {
    #[inline(always)]
    fn from(variant: Ckmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckmode {
    type Ux = u8;
}
impl crate::IsEnum for Ckmode {}
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type CkmodeR = crate::FieldReader<Ckmode>;
impl CkmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckmode {
        match self.bits {
            0 => Ckmode::Adcclk,
            1 => Ckmode::PclkBy2,
            2 => Ckmode::PclkBy4,
            3 => Ckmode::Pclk,
            _ => unreachable!(),
        }
    }
    #[doc = "ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    #[inline(always)]
    pub fn is_adcclk(&self) -> bool {
        *self == Ckmode::Adcclk
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_pclk_by_2(&self) -> bool {
        *self == Ckmode::PclkBy2
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_pclk_by_4(&self) -> bool {
        *self == Ckmode::PclkBy4
    }
    #[doc = "PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == Ckmode::Pclk
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type CkmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckmode, crate::Safe>;
impl<'a, REG> CkmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    #[inline(always)]
    pub fn adcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmode::Adcclk)
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmode::PclkBy2)
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmode::PclkBy4)
    }
    #[doc = "PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmode::Pclk)
    }
}
impl R {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovse(&self) -> OvseR {
        OvseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovsr(&self) -> OvsrR {
        OvsrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tovs(&self) -> TovsR {
        TovsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn lftrig(&self) -> LftrigR {
        LftrigR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CkmodeR {
        CkmodeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OvseW<AdcCfgr2Spec> {
        OvseW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OvsrW<AdcCfgr2Spec> {
        OvsrW::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OvssW<AdcCfgr2Spec> {
        OvssW::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TovsW<AdcCfgr2Spec> {
        TovsW::new(self, 9)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LftrigW<AdcCfgr2Spec> {
        LftrigW::new(self, 29)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CkmodeW<AdcCfgr2Spec> {
        CkmodeW::new(self, 30)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCfgr2Spec;
impl crate::RegisterSpec for AdcCfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cfgr2::R`](R) reader structure"]
impl crate::Readable for AdcCfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_cfgr2::W`](W) writer structure"]
impl crate::Writable for AdcCfgr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CFGR2 to value 0"]
impl crate::Resettable for AdcCfgr2Spec {
    const RESET_VALUE: u32 = 0;
}
