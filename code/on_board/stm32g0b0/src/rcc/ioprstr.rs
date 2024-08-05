#[doc = "Register `IOPRSTR` reader"]
pub type R = crate::R<IoprstrSpec>;
#[doc = "Register `IOPRSTR` writer"]
pub type W = crate::W<IoprstrSpec>;
#[doc = "Field `GPIOARST` reader - GPIOARST"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - GPIOARST"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - GPIOBRST"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - GPIOBRST"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - GPIOCRST"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - GPIOCRST"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - GPIODRST"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - GPIODRST"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - GPIOERST"]
pub type GpioerstR = crate::BitReader;
#[doc = "Field `GPIOERST` writer - GPIOERST"]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - GPIOFRST"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - GPIOFRST"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GpioarstW<IoprstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GpiobrstW<IoprstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GpiocrstW<IoprstrSpec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GpiodrstW<IoprstrSpec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GpioerstW<IoprstrSpec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GpiofrstW<IoprstrSpec> {
        GpiofrstW::new(self, 5)
    }
}
#[doc = "I/O port reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoprstrSpec;
impl crate::RegisterSpec for IoprstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioprstr::R`](R) reader structure"]
impl crate::Readable for IoprstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ioprstr::W`](W) writer structure"]
impl crate::Writable for IoprstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IoprstrSpec {
    const RESET_VALUE: u32 = 0;
}
