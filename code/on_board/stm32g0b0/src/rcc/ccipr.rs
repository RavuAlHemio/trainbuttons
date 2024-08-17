#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CciprSpec>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CciprSpec>;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart1sel {
    #[doc = "0: Use the peripheral clock."]
    Pclk = 0,
    #[doc = "1: Use the system clock."]
    Sysclk = 1,
    #[doc = "2: Use the high-speed internal 16 MHz oscillator."]
    Hsi16 = 2,
    #[doc = "3: Use the low-speed external oscillator."]
    Lse = 3,
}
impl From<Usart1sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart1sel {
    type Ux = u8;
}
impl crate::IsEnum for Usart1sel {}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type Usart1selR = crate::FieldReader<Usart1sel>;
impl Usart1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1sel {
        match self.bits {
            0 => Usart1sel::Pclk,
            1 => Usart1sel::Sysclk,
            2 => Usart1sel::Hsi16,
            3 => Usart1sel::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Use the peripheral clock."]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == Usart1sel::Pclk
    }
    #[doc = "Use the system clock."]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Usart1sel::Sysclk
    }
    #[doc = "Use the high-speed internal 16 MHz oscillator."]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == Usart1sel::Hsi16
    }
    #[doc = "Use the low-speed external oscillator."]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == Usart1sel::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type Usart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usart1sel, crate::Safe>;
impl<'a, REG> Usart1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use the peripheral clock."]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::Pclk)
    }
    #[doc = "Use the system clock."]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::Sysclk)
    }
    #[doc = "Use the high-speed internal 16 MHz oscillator."]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::Hsi16)
    }
    #[doc = "Use the low-speed external oscillator."]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::Lse)
    }
}
#[doc = "USART2 clock source selection"]
pub use Usart1sel as Usart2sel;
#[doc = "USART3 clock source selection"]
pub use Usart1sel as Usart3sel;
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub use Usart1selR as Usart2selR;
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub use Usart1selR as Usart3selR;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub use Usart1selW as Usart2selW;
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub use Usart1selW as Usart3selW;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2c1selR = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2c1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2SEL` reader - I2S1 clock source selection"]
pub type I2s2selR = crate::FieldReader;
#[doc = "Field `I2S2SEL` writer - I2S1 clock source selection"]
pub type I2s2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM1SEL` reader - TIM1 clock source selection"]
pub type Tim1selR = crate::BitReader;
#[doc = "Field `TIM1SEL` writer - TIM1 clock source selection"]
pub type Tim1selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SEL` reader - TIM15 clock source selection"]
pub type Tim15selR = crate::BitReader;
#[doc = "Field `TIM15SEL` writer - TIM15 clock source selection"]
pub type Tim15selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type AdcselR = crate::FieldReader;
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type AdcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> Usart1selR {
        Usart1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> Usart2selR {
        Usart2selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> Usart3selR {
        Usart3selR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2c1selR {
        I2c1selR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2s2selR {
        I2s2selR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    pub fn tim1sel(&self) -> Tim1selR {
        Tim1selR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    pub fn tim15sel(&self) -> Tim15selR {
        Tim15selR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> AdcselR {
        AdcselR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> Usart1selW<CciprSpec> {
        Usart1selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> Usart2selW<CciprSpec> {
        Usart2selW::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> Usart3selW<CciprSpec> {
        Usart3selW::new(self, 4)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2c1selW<CciprSpec> {
        I2c1selW::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2s2selW<CciprSpec> {
        I2s2selW::new(self, 14)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sel(&mut self) -> Tim1selW<CciprSpec> {
        Tim1selW::new(self, 22)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sel(&mut self) -> Tim15selW<CciprSpec> {
        Tim15selW::new(self, 24)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> AdcselW<CciprSpec> {
        AdcselW::new(self, 30)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciprSpec;
impl crate::RegisterSpec for CciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CciprSpec {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CciprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CciprSpec {
    const RESET_VALUE: u32 = 0;
}
