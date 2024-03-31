#[doc = "Register `APBSMENR2` reader"]
pub type R = crate::R<Apbsmenr2Spec>;
#[doc = "Register `APBSMENR2` writer"]
pub type W = crate::W<Apbsmenr2Spec>;
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub type SyscfgsmenR = crate::BitReader;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub type SyscfgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode"]
pub type Tim1smenR = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode"]
pub type Tim1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode"]
pub type Spi1smenR = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode"]
pub type Spi1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep mode"]
pub type Usart1smenR = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep mode"]
pub type Usart1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode"]
pub type Tim14smenR = crate::BitReader;
#[doc = "Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode"]
pub type Tim14smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clock enable during Sleep mode"]
pub type Tim15smenR = crate::BitReader;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clock enable during Sleep mode"]
pub type Tim15smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub type Tim16smenR = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub type Tim16smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub type Tim17smenR = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub type Tim17smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSMEN` reader - ADC clock enable during Sleep mode"]
pub type AdcsmenR = crate::BitReader;
#[doc = "Field `ADCSMEN` writer - ADC clock enable during Sleep mode"]
pub type AdcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SyscfgsmenR {
        SyscfgsmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1smen(&self) -> Tim1smenR {
        Tim1smenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1smen(&self) -> Spi1smenR {
        Spi1smenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1smen(&self) -> Usart1smenR {
        Usart1smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14smen(&self) -> Tim14smenR {
        Tim14smenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim15smen(&self) -> Tim15smenR {
        Tim15smenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim16smen(&self) -> Tim16smenR {
        Tim16smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim17smen(&self) -> Tim17smenR {
        Tim17smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adcsmen(&self) -> AdcsmenR {
        AdcsmenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SyscfgsmenW<Apbsmenr2Spec> {
        SyscfgsmenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> Tim1smenW<Apbsmenr2Spec> {
        Tim1smenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> Spi1smenW<Apbsmenr2Spec> {
        Spi1smenW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> Usart1smenW<Apbsmenr2Spec> {
        Usart1smenW::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim14smen(&mut self) -> Tim14smenW<Apbsmenr2Spec> {
        Tim14smenW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> Tim15smenW<Apbsmenr2Spec> {
        Tim15smenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> Tim16smenW<Apbsmenr2Spec> {
        Tim16smenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> Tim17smenW<Apbsmenr2Spec> {
        Tim17smenW::new(self, 18)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> AdcsmenW<Apbsmenr2Spec> {
        AdcsmenW::new(self, 20)
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbsmenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbsmenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbsmenr2Spec;
impl crate::RegisterSpec for Apbsmenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbsmenr2::R`](R) reader structure"]
impl crate::Readable for Apbsmenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbsmenr2::W`](W) writer structure"]
impl crate::Writable for Apbsmenr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSMENR2 to value 0x0017_d801"]
impl crate::Resettable for Apbsmenr2Spec {
    const RESET_VALUE: u32 = 0x0017_d801;
}
