#[doc = "Register `IOPSMENR` reader"]
pub type R = crate::R<IopsmenrSpec>;
#[doc = "Register `IOPSMENR` writer"]
pub type W = crate::W<IopsmenrSpec>;
#[doc = "Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode"]
pub type GpioasmenR = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode"]
pub type GpioasmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode"]
pub type GpiobsmenR = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode"]
pub type GpiobsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode"]
pub type GpiocsmenR = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode"]
pub type GpiocsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode"]
pub type GpiodsmenR = crate::BitReader;
#[doc = "Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode"]
pub type GpiodsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode"]
pub type GpioesmenR = crate::BitReader;
#[doc = "Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode"]
pub type GpioesmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode"]
pub type GpiofsmenR = crate::BitReader;
#[doc = "Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode"]
pub type GpiofsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GpioasmenR {
        GpioasmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GpiobsmenR {
        GpiobsmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GpiocsmenR {
        GpiocsmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GpiodsmenR {
        GpiodsmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GpioesmenR {
        GpioesmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GpiofsmenR {
        GpiofsmenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GpioasmenW<IopsmenrSpec> {
        GpioasmenW::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GpiobsmenW<IopsmenrSpec> {
        GpiobsmenW::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GpiocsmenW<IopsmenrSpec> {
        GpiocsmenW::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GpiodsmenW<IopsmenrSpec> {
        GpiodsmenW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GpioesmenW<IopsmenrSpec> {
        GpioesmenW::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GpiofsmenW<IopsmenrSpec> {
        GpiofsmenW::new(self, 5)
    }
}
#[doc = "GPIO in Sleep mode clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopsmenrSpec;
impl crate::RegisterSpec for IopsmenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopsmenr::R`](R) reader structure"]
impl crate::Readable for IopsmenrSpec {}
#[doc = "`write(|w| ..)` method takes [`iopsmenr::W`](W) writer structure"]
impl crate::Writable for IopsmenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPSMENR to value 0x3f"]
impl crate::Resettable for IopsmenrSpec {
    const RESET_VALUE: u32 = 0x3f;
}
