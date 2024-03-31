#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<Ccmr2OutputSpec>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<Ccmr2OutputSpec>;
#[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc3s {
    #[doc = "0: CC3 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC3 channel is configured as input, IC3 is mapped on TI3"]
    B0x1 = 1,
    #[doc = "2: CC3 channel is configured as input, IC3 is mapped on TI4"]
    B0x2 = 2,
    #[doc = "3: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc3s> for u8 {
    #[inline(always)]
    fn from(variant: Cc3s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc3s {
    type Ux = u8;
}
impl crate::IsEnum for Cc3s {}
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type Cc3sR = crate::FieldReader<Cc3s>;
impl Cc3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3s {
        match self.bits {
            0 => Cc3s::B0x0,
            1 => Cc3s::B0x1,
            2 => Cc3s::B0x2,
            3 => Cc3s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3s::B0x0
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3s::B0x1
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc3s::B0x2
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc3s::B0x3
    }
}
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc3s, crate::Safe>;
impl<'a, REG> Cc3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x0)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x1)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x2)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x3)
    }
}
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable Refer to OC1FE description."]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable Refer to OC1FE description."]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable Refer to OC1PE description."]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable Refer to OC1PE description."]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M1` reader - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc3m1R = crate::FieldReader;
#[doc = "Field `OC3M1` writer - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc3m1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable Refer to OC1CE description."]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable Refer to OC1CE description."]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc4s {
    #[doc = "0: CC4 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC4 channel is configured as input, IC4 is mapped on TI4"]
    B0x1 = 1,
    #[doc = "2: CC4 channel is configured as input, IC4 is mapped on TI3"]
    B0x2 = 2,
    #[doc = "3: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc4s> for u8 {
    #[inline(always)]
    fn from(variant: Cc4s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc4s {
    type Ux = u8;
}
impl crate::IsEnum for Cc4s {}
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type Cc4sR = crate::FieldReader<Cc4s>;
impl Cc4sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4s {
        match self.bits {
            0 => Cc4s::B0x0,
            1 => Cc4s::B0x1,
            2 => Cc4s::B0x2,
            3 => Cc4s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4s::B0x0
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4s::B0x1
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc4s::B0x2
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc4s::B0x3
    }
}
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc4s, crate::Safe>;
impl<'a, REG> Cc4sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x0)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x1)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x2)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x3)
    }
}
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable Refer to OC1FE description."]
pub type Oc4feR = crate::BitReader;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable Refer to OC1FE description."]
pub type Oc4feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable Refer to OC1PE description."]
pub type Oc4peR = crate::BitReader;
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable Refer to OC1PE description."]
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M1` reader - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type Oc4m1R = crate::FieldReader;
#[doc = "Field `OC4M1` writer - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type Oc4m1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable Refer to OC1CE description."]
pub type Oc4ceR = crate::BitReader;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable Refer to OC1CE description."]
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M2` reader - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc3m2R = crate::BitReader;
#[doc = "Field `OC3M2` writer - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type Oc3m2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M2` reader - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type Oc4m2R = crate::BitReader;
#[doc = "Field `OC4M2` writer - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type Oc4m2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m1(&self) -> Oc3m1R {
        Oc3m1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc4fe(&self) -> Oc4feR {
        Oc4feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m1(&self) -> Oc4m1R {
        Oc4m1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m2(&self) -> Oc3m2R {
        Oc3m2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m2(&self) -> Oc4m2R {
        Oc4m2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> Cc3sW<Ccmr2OutputSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> Oc3feW<Ccmr2OutputSpec> {
        Oc3feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> Oc3peW<Ccmr2OutputSpec> {
        Oc3peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3m1(&mut self) -> Oc3m1W<Ccmr2OutputSpec> {
        Oc3m1W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> Oc3ceW<Ccmr2OutputSpec> {
        Oc3ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> Cc4sW<Ccmr2OutputSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> Oc4feW<Ccmr2OutputSpec> {
        Oc4feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> Oc4peW<Ccmr2OutputSpec> {
        Oc4peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4m1(&mut self) -> Oc4m1W<Ccmr2OutputSpec> {
        Oc4m1W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> Oc4ceW<Ccmr2OutputSpec> {
        Oc4ceW::new(self, 15)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3m2(&mut self) -> Oc3m2W<Ccmr2OutputSpec> {
        Oc3m2W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4m2(&mut self) -> Oc4m2W<Ccmr2OutputSpec> {
        Oc4m2W::new(self, 24)
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2OutputSpec;
impl crate::RegisterSpec for Ccmr2OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for Ccmr2OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for Ccmr2OutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for Ccmr2OutputSpec {
    const RESET_VALUE: u32 = 0;
}
