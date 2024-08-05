#[doc = "Register `CCIPR2` reader"]
pub type R = crate::R<Ccipr2Spec>;
#[doc = "Register `CCIPR2` writer"]
pub type W = crate::W<Ccipr2Spec>;
#[doc = "Field `I2S1SEL` reader - 2S1SEL"]
pub type I2s1selR = crate::FieldReader;
#[doc = "Field `I2S1SEL` writer - 2S1SEL"]
pub type I2s1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2SEL` reader - I2S2SEL"]
pub type I2s2selR = crate::FieldReader;
#[doc = "Field `I2S2SEL` writer - I2S2SEL"]
pub type I2s2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "USBSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbsel {
    #[doc = "1: USB clock is taken from PLLQCLK"]
    Pllqclk = 1,
    #[doc = "2: USB clock is taken from HSE"]
    Hse = 2,
}
impl From<Usbsel> for u8 {
    #[inline(always)]
    fn from(variant: Usbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbsel {
    type Ux = u8;
}
impl crate::IsEnum for Usbsel {}
#[doc = "Field `USBSEL` reader - USBSEL"]
pub type UsbselR = crate::FieldReader<Usbsel>;
impl UsbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbsel> {
        match self.bits {
            1 => Some(Usbsel::Pllqclk),
            2 => Some(Usbsel::Hse),
            _ => None,
        }
    }
    #[doc = "USB clock is taken from PLLQCLK"]
    #[inline(always)]
    pub fn is_pllqclk(&self) -> bool {
        *self == Usbsel::Pllqclk
    }
    #[doc = "USB clock is taken from HSE"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == Usbsel::Hse
    }
}
#[doc = "Field `USBSEL` writer - USBSEL"]
pub type UsbselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbsel>;
impl<'a, REG> UsbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB clock is taken from PLLQCLK"]
    #[inline(always)]
    pub fn pllqclk(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::Pllqclk)
    }
    #[doc = "USB clock is taken from HSE"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::Hse)
    }
}
impl R {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2s1selR {
        I2s1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2s2selR {
        I2s2selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 12:13 - USBSEL"]
    #[inline(always)]
    pub fn usbsel(&self) -> UsbselR {
        UsbselR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1sel(&mut self) -> I2s1selW<Ccipr2Spec> {
        I2s1selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2s2selW<Ccipr2Spec> {
        I2s2selW::new(self, 2)
    }
    #[doc = "Bits 12:13 - USBSEL"]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> UsbselW<Ccipr2Spec> {
        UsbselW::new(self, 12)
    }
}
#[doc = "Peripherals independent clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccipr2Spec;
impl crate::RegisterSpec for Ccipr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr2::R`](R) reader structure"]
impl crate::Readable for Ccipr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure"]
impl crate::Writable for Ccipr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for Ccipr2Spec {
    const RESET_VALUE: u32 = 0;
}
