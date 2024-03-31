#[doc = "Register `ADC_CCR` reader"]
pub type R = crate::R<AdcCcrSpec>;
#[doc = "Register `ADC_CCR` writer"]
pub type W = crate::W<AdcCcrSpec>;
#[doc = "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: input ADC clock not divided"]
    B0x0 = 0,
    #[doc = "1: input ADC clock divided by 2"]
    B0x1 = 1,
    #[doc = "2: input ADC clock divided by 4"]
    B0x2 = 2,
    #[doc = "3: input ADC clock divided by 6"]
    B0x3 = 3,
    #[doc = "4: input ADC clock divided by 8"]
    B0x4 = 4,
    #[doc = "5: input ADC clock divided by 10"]
    B0x5 = 5,
    #[doc = "6: input ADC clock divided by 12"]
    B0x6 = 6,
    #[doc = "7: input ADC clock divided by 16"]
    B0x7 = 7,
    #[doc = "8: input ADC clock divided by 32"]
    B0x8 = 8,
    #[doc = "9: input ADC clock divided by 64"]
    B0x9 = 9,
    #[doc = "10: input ADC clock divided by 128"]
    B0xA = 10,
    #[doc = "11: input ADC clock divided by 256"]
    B0xB = 11,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Presc> {
        match self.bits {
            0 => Some(Presc::B0x0),
            1 => Some(Presc::B0x1),
            2 => Some(Presc::B0x2),
            3 => Some(Presc::B0x3),
            4 => Some(Presc::B0x4),
            5 => Some(Presc::B0x5),
            6 => Some(Presc::B0x6),
            7 => Some(Presc::B0x7),
            8 => Some(Presc::B0x8),
            9 => Some(Presc::B0x9),
            10 => Some(Presc::B0xA),
            11 => Some(Presc::B0xB),
            _ => None,
        }
    }
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Presc::B0x0
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Presc::B0x1
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Presc::B0x2
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Presc::B0x3
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Presc::B0x4
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Presc::B0x5
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Presc::B0x6
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Presc::B0x7
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Presc::B0x8
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Presc::B0x9
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Presc::B0xA
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Presc::B0xB
    }
}
#[doc = "Field `PRESC` writer - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x0)
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x1)
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x2)
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x3)
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x4)
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x5)
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x6)
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x7)
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x8)
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x9)
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0xA)
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0xB)
    }
}
#[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrefen {
    #[doc = "0: VREFINT disabled"]
    B0x0 = 0,
    #[doc = "1: VREFINT enabled"]
    B0x1 = 1,
}
impl From<Vrefen> for bool {
    #[inline(always)]
    fn from(variant: Vrefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type VrefenR = crate::BitReader<Vrefen>;
impl VrefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefen {
        match self.bits {
            false => Vrefen::B0x0,
            true => Vrefen::B0x1,
        }
    }
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vrefen::B0x0
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vrefen::B0x1
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type VrefenW<'a, REG> = crate::BitWriter<'a, REG, Vrefen>;
impl<'a, REG> VrefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefen::B0x0)
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefen::B0x1)
    }
}
#[doc = "Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsen {
    #[doc = "0: Temperature sensor disabled"]
    B0x0 = 0,
    #[doc = "1: Temperature sensor enabled"]
    B0x1 = 1,
}
impl From<Tsen> for bool {
    #[inline(always)]
    fn from(variant: Tsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TsenR = crate::BitReader<Tsen>;
impl TsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsen {
        match self.bits {
            false => Tsen::B0x0,
            true => Tsen::B0x1,
        }
    }
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsen::B0x0
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsen::B0x1
    }
}
#[doc = "Field `TSEN` writer - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG, Tsen>;
impl<'a, REG> TsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::B0x0)
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::B0x1)
    }
}
#[doc = "VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbaten {
    #[doc = "0: VBAT channel disabled"]
    B0x0 = 0,
    #[doc = "1: VBAT channel enabled"]
    B0x1 = 1,
}
impl From<Vbaten> for bool {
    #[inline(always)]
    fn from(variant: Vbaten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
pub type VbatenR = crate::BitReader<Vbaten>;
impl VbatenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbaten {
        match self.bits {
            false => Vbaten::B0x0,
            true => Vbaten::B0x1,
        }
    }
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vbaten::B0x0
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vbaten::B0x1
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
pub type VbatenW<'a, REG> = crate::BitWriter<'a, REG, Vbaten>;
impl<'a, REG> VbatenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbaten::B0x0)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbaten::B0x1)
    }
}
impl R {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn vrefen(&self) -> VrefenR {
        VrefenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn vbaten(&self) -> VbatenR {
        VbatenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<AdcCcrSpec> {
        PrescW::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VrefenW<AdcCcrSpec> {
        VrefenW::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<AdcCcrSpec> {
        TsenW::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VbatenW<AdcCcrSpec> {
        VbatenW::new(self, 24)
    }
}
#[doc = "ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCcrSpec;
impl crate::RegisterSpec for AdcCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ccr::R`](R) reader structure"]
impl crate::Readable for AdcCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_ccr::W`](W) writer structure"]
impl crate::Writable for AdcCcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CCR to value 0"]
impl crate::Resettable for AdcCcrSpec {
    const RESET_VALUE: u32 = 0;
}
