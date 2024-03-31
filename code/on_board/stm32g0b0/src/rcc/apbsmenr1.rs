#[doc = "Register `APBSMENR1` reader"]
pub type R = crate::R<Apbsmenr1Spec>;
#[doc = "Register `APBSMENR1` writer"]
pub type W = crate::W<Apbsmenr1Spec>;
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode"]
pub type Tim3smenR = crate::BitReader;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode"]
pub type Tim3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clock enable during Sleep mode"]
pub type Tim4smenR = crate::BitReader;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clock enable during Sleep mode"]
pub type Tim4smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode"]
pub type Tim6smenR = crate::BitReader;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode"]
pub type Tim6smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode"]
pub type Tim7smenR = crate::BitReader;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode"]
pub type Tim7smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5SMEN` reader - USART5 clock enable"]
pub type Usart5smenR = crate::BitReader;
#[doc = "Field `USART5SMEN` writer - USART5 clock enable"]
pub type Usart5smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6SMEN` reader - USART6 clock enable"]
pub type Usart6smenR = crate::BitReader;
#[doc = "Field `USART6SMEN` writer - USART6 clock enable"]
pub type Usart6smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode"]
pub type RtcapbsmenR = crate::BitReader;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode"]
pub type RtcapbsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode"]
pub type WwdgsmenR = crate::BitReader;
#[doc = "Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode"]
pub type WwdgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSMEN` reader - USB clock enable during Sleep mode"]
pub type UsbsmenR = crate::BitReader;
#[doc = "Field `USBSMEN` writer - USB clock enable during Sleep mode"]
pub type UsbsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode"]
pub type Spi2smenR = crate::BitReader;
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode"]
pub type Spi2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode"]
pub type Spi3smenR = crate::BitReader;
#[doc = "Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode"]
pub type Spi3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during Sleep mode"]
pub type Usart2smenR = crate::BitReader;
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during Sleep mode"]
pub type Usart2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3SMEN` reader - USART3 clock enable during Sleep mode"]
pub type Usart3smenR = crate::BitReader;
#[doc = "Field `USART3SMEN` writer - USART3 clock enable during Sleep mode"]
pub type Usart3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4SMEN` reader - USART4 clock enable during Sleep mode"]
pub type Usart4smenR = crate::BitReader;
#[doc = "Field `USART4SMEN` writer - USART4 clock enable during Sleep mode"]
pub type Usart4smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2c1smenR = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2c1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2c2smenR = crate::BitReader;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2c2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2c3smenR = crate::BitReader;
#[doc = "Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode"]
pub type I2c3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSMEN` reader - Debug support clock enable during Sleep mode"]
pub type DbgsmenR = crate::BitReader;
#[doc = "Field `DBGSMEN` writer - Debug support clock enable during Sleep mode"]
pub type DbgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during Sleep mode"]
pub type PwrsmenR = crate::BitReader;
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during Sleep mode"]
pub type PwrsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3smen(&self) -> Tim3smenR {
        Tim3smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4smen(&self) -> Tim4smenR {
        Tim4smenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6smen(&self) -> Tim6smenR {
        Tim6smenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7smen(&self) -> Tim7smenR {
        Tim7smenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5smen(&self) -> Usart5smenR {
        Usart5smenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6smen(&self) -> Usart6smenR {
        Usart6smenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RtcapbsmenR {
        RtcapbsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WwdgsmenR {
        WwdgsmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&self) -> UsbsmenR {
        UsbsmenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> Spi2smenR {
        Spi2smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3smen(&self) -> Spi3smenR {
        Spi3smenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2smen(&self) -> Usart2smenR {
        Usart2smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3smen(&self) -> Usart3smenR {
        Usart3smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart4smen(&self) -> Usart4smenR {
        Usart4smenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2c1smenR {
        I2c1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2c2smenR {
        I2c2smenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2c3smenR {
        I2c3smenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DbgsmenR {
        DbgsmenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PwrsmenR {
        PwrsmenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> Tim3smenW<Apbsmenr1Spec> {
        Tim3smenW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> Tim4smenW<Apbsmenr1Spec> {
        Tim4smenW::new(self, 2)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> Tim6smenW<Apbsmenr1Spec> {
        Tim6smenW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> Tim7smenW<Apbsmenr1Spec> {
        Tim7smenW::new(self, 5)
    }
    #[doc = "Bit 8 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5smen(&mut self) -> Usart5smenW<Apbsmenr1Spec> {
        Usart5smenW::new(self, 8)
    }
    #[doc = "Bit 9 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6smen(&mut self) -> Usart6smenW<Apbsmenr1Spec> {
        Usart6smenW::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RtcapbsmenW<Apbsmenr1Spec> {
        RtcapbsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WwdgsmenW<Apbsmenr1Spec> {
        WwdgsmenW::new(self, 11)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> UsbsmenW<Apbsmenr1Spec> {
        UsbsmenW::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> Spi2smenW<Apbsmenr1Spec> {
        Spi2smenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3smen(&mut self) -> Spi3smenW<Apbsmenr1Spec> {
        Spi3smenW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> Usart2smenW<Apbsmenr1Spec> {
        Usart2smenW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> Usart3smenW<Apbsmenr1Spec> {
        Usart3smenW::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart4smen(&mut self) -> Usart4smenW<Apbsmenr1Spec> {
        Usart4smenW::new(self, 19)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2c1smenW<Apbsmenr1Spec> {
        I2c1smenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2c2smenW<Apbsmenr1Spec> {
        I2c2smenW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2c3smenW<Apbsmenr1Spec> {
        I2c3smenW::new(self, 23)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DbgsmenW<Apbsmenr1Spec> {
        DbgsmenW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PwrsmenW<Apbsmenr1Spec> {
        PwrsmenW::new(self, 28)
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbsmenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbsmenr1Spec;
impl crate::RegisterSpec for Apbsmenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbsmenr1::R`](R) reader structure"]
impl crate::Readable for Apbsmenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbsmenr1::W`](W) writer structure"]
impl crate::Writable for Apbsmenr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSMENR1 to value 0xffff_ffb7"]
impl crate::Resettable for Apbsmenr1Spec {
    const RESET_VALUE: u32 = 0xffff_ffb7;
}
