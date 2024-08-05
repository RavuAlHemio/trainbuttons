#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PrescSpec>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PrescSpec>;
#[doc = "Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler {
    #[doc = "0: input clock not divided"]
    B0x0 = 0,
    #[doc = "1: input clock divided by 2"]
    B0x1 = 1,
    #[doc = "2: input clock divided by 4"]
    B0x2 = 2,
    #[doc = "3: input clock divided by 6"]
    B0x3 = 3,
    #[doc = "4: input clock divided by 8"]
    B0x4 = 4,
    #[doc = "5: input clock divided by 10"]
    B0x5 = 5,
    #[doc = "6: input clock divided by 12"]
    B0x6 = 6,
    #[doc = "7: input clock divided by 16"]
    B0x7 = 7,
    #[doc = "8: input clock divided by 32"]
    B0x8 = 8,
    #[doc = "9: input clock divided by 64"]
    B0x9 = 9,
    #[doc = "10: input clock divided by 128"]
    B0xA = 10,
    #[doc = "11: input clock divided by 256"]
    B0xB = 11,
}
impl From<Prescaler> for u8 {
    #[inline(always)]
    fn from(variant: Prescaler) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescaler {
    type Ux = u8;
}
impl crate::IsEnum for Prescaler {}
#[doc = "Field `PRESCALER` reader - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
pub type PrescalerR = crate::FieldReader<Prescaler>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescaler> {
        match self.bits {
            0 => Some(Prescaler::B0x0),
            1 => Some(Prescaler::B0x1),
            2 => Some(Prescaler::B0x2),
            3 => Some(Prescaler::B0x3),
            4 => Some(Prescaler::B0x4),
            5 => Some(Prescaler::B0x5),
            6 => Some(Prescaler::B0x6),
            7 => Some(Prescaler::B0x7),
            8 => Some(Prescaler::B0x8),
            9 => Some(Prescaler::B0x9),
            10 => Some(Prescaler::B0xA),
            11 => Some(Prescaler::B0xB),
            _ => None,
        }
    }
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Prescaler::B0x0
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Prescaler::B0x1
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Prescaler::B0x2
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Prescaler::B0x3
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Prescaler::B0x4
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Prescaler::B0x5
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Prescaler::B0x6
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Prescaler::B0x7
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Prescaler::B0x8
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Prescaler::B0x9
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Prescaler::B0xA
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Prescaler::B0xB
    }
}
#[doc = "Field `PRESCALER` writer - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prescaler>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x0)
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x1)
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x2)
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x3)
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x4)
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x5)
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x6)
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x7)
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x8)
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0x9)
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0xA)
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::B0xB)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<PrescSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescSpec;
impl crate::RegisterSpec for PrescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PrescSpec {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PrescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PrescSpec {
    const RESET_VALUE: u32 = 0;
}
