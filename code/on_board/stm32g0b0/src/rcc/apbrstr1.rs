#[doc = "Register `APBRSTR1` reader"]
pub type R = crate::R<Apbrstr1Spec>;
#[doc = "Register `APBRSTR1` writer"]
pub type W = crate::W<Apbrstr1Spec>;
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub type Tim3rstR = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4RST` reader - TIM4 timer reset"]
pub type Tim4rstR = crate::BitReader;
#[doc = "Field `TIM4RST` writer - TIM4 timer reset"]
pub type Tim4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6RST` reader - TIM6 timer reset"]
pub type Tim6rstR = crate::BitReader;
#[doc = "Field `TIM6RST` writer - TIM6 timer reset"]
pub type Tim6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7RST` reader - TIM7 timer reset"]
pub type Tim7rstR = crate::BitReader;
#[doc = "Field `TIM7RST` writer - TIM7 timer reset"]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5RST` reader - USART5RST"]
pub type Usart5rstR = crate::BitReader;
#[doc = "Field `USART5RST` writer - USART5RST"]
pub type Usart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6RST` reader - USART6RST"]
pub type Usart6rstR = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6RST"]
pub type Usart6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USBRST"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type Spi2rstR = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub type Spi3rstR = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub type Usart3rstR = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4RST` reader - USART4 reset"]
pub type Usart4rstR = crate::BitReader;
#[doc = "Field `USART4RST` writer - USART4 reset"]
pub type Usart4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3RST` reader - I2C3RST reset"]
pub type I2c3rstR = crate::BitReader;
#[doc = "Field `I2C3RST` writer - I2C3RST reset"]
pub type I2c3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DbgrstR = crate::BitReader;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DbgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PwrrstR = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PwrrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> Tim4rstR {
        Tim4rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> Tim6rstR {
        Tim6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USART5RST"]
    #[inline(always)]
    pub fn usart5rst(&self) -> Usart5rstR {
        Usart5rstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&self) -> Usart6rstR {
        Usart6rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    pub fn usart4rst(&self) -> Usart4rstR {
        Usart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3RST reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2c3rstR {
        I2c3rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DbgrstR {
        DbgrstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PwrrstR {
        PwrrstR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> Tim3rstW<Apbrstr1Spec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> Tim4rstW<Apbrstr1Spec> {
        Tim4rstW::new(self, 2)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> Tim6rstW<Apbrstr1Spec> {
        Tim6rstW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> Tim7rstW<Apbrstr1Spec> {
        Tim7rstW::new(self, 5)
    }
    #[doc = "Bit 8 - USART5RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> Usart5rstW<Apbrstr1Spec> {
        Usart5rstW::new(self, 8)
    }
    #[doc = "Bit 9 - USART6RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> Usart6rstW<Apbrstr1Spec> {
        Usart6rstW::new(self, 9)
    }
    #[doc = "Bit 13 - USBRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<Apbrstr1Spec> {
        UsbrstW::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> Spi2rstW<Apbrstr1Spec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> Spi3rstW<Apbrstr1Spec> {
        Spi3rstW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> Usart2rstW<Apbrstr1Spec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> Usart3rstW<Apbrstr1Spec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart4rst(&mut self) -> Usart4rstW<Apbrstr1Spec> {
        Usart4rstW::new(self, 19)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apbrstr1Spec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2c2rstW<Apbrstr1Spec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3RST reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2c3rstW<Apbrstr1Spec> {
        I2c3rstW::new(self, 23)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DbgrstW<Apbrstr1Spec> {
        DbgrstW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PwrrstW<Apbrstr1Spec> {
        PwrrstW::new(self, 28)
    }
}
#[doc = "APB peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrstr1Spec;
impl crate::RegisterSpec for Apbrstr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr1::R`](R) reader structure"]
impl crate::Readable for Apbrstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrstr1::W`](W) writer structure"]
impl crate::Writable for Apbrstr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBRSTR1 to value 0"]
impl crate::Resettable for Apbrstr1Spec {
    const RESET_VALUE: u32 = 0;
}
