#[doc = "Register `APBRSTR2` reader"]
pub type R = crate::R<Apbrstr2Spec>;
#[doc = "Register `APBRSTR2` writer"]
pub type W = crate::W<Apbrstr2Spec>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG, COMP and VREFBUF reset"]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG, COMP and VREFBUF reset"]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type Tim1rstR = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14RST` reader - TIM14 timer reset"]
pub type Tim14rstR = crate::BitReader;
#[doc = "Field `TIM14RST` writer - TIM14 timer reset"]
pub type Tim14rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub type Tim15rstR = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub type Tim15rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type Tim16rstR = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub type Tim17rstR = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub type Tim17rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> Tim14rstR {
        Tim14rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> Tim15rstR {
        Tim15rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> Tim17rstR {
        Tim17rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<Apbrstr2Spec> {
        SyscfgrstW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> Tim1rstW<Apbrstr2Spec> {
        Tim1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apbrstr2Spec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apbrstr2Spec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> Tim14rstW<Apbrstr2Spec> {
        Tim14rstW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> Tim15rstW<Apbrstr2Spec> {
        Tim15rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> Tim16rstW<Apbrstr2Spec> {
        Tim16rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> Tim17rstW<Apbrstr2Spec> {
        Tim17rstW::new(self, 18)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> AdcrstW<Apbrstr2Spec> {
        AdcrstW::new(self, 20)
    }
}
#[doc = "APB peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrstr2Spec;
impl crate::RegisterSpec for Apbrstr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr2::R`](R) reader structure"]
impl crate::Readable for Apbrstr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrstr2::W`](W) writer structure"]
impl crate::Writable for Apbrstr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBRSTR2 to value 0"]
impl crate::Resettable for Apbrstr2Spec {
    const RESET_VALUE: u32 = 0;
}
