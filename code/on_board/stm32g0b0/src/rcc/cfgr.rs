#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sw {
    #[doc = "0: SYSCLK is taken from HSISYS (HSI16 prescaled by HSIDIV)"]
    Hsisys = 0,
    #[doc = "1: SYSCLK is taken from HSE"]
    Hse = 1,
    #[doc = "2: SYSCLK is taken from PLLRCLK"]
    Pllrclk = 2,
    #[doc = "3: SYSCLK is taken from LSI"]
    Lsi = 3,
    #[doc = "4: SYSCLK is taken from LSE"]
    Lse = 4,
}
impl From<Sw> for u8 {
    #[inline(always)]
    fn from(variant: Sw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sw {
    type Ux = u8;
}
impl crate::IsEnum for Sw {}
#[doc = "Field `SW` reader - System clock switch"]
pub type SwR = crate::FieldReader<Sw>;
impl SwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sw> {
        match self.bits {
            0 => Some(Sw::Hsisys),
            1 => Some(Sw::Hse),
            2 => Some(Sw::Pllrclk),
            3 => Some(Sw::Lsi),
            4 => Some(Sw::Lse),
            _ => None,
        }
    }
    #[doc = "SYSCLK is taken from HSISYS (HSI16 prescaled by HSIDIV)"]
    #[inline(always)]
    pub fn is_hsisys(&self) -> bool {
        *self == Sw::Hsisys
    }
    #[doc = "SYSCLK is taken from HSE"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == Sw::Hse
    }
    #[doc = "SYSCLK is taken from PLLRCLK"]
    #[inline(always)]
    pub fn is_pllrclk(&self) -> bool {
        *self == Sw::Pllrclk
    }
    #[doc = "SYSCLK is taken from LSI"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == Sw::Lsi
    }
    #[doc = "SYSCLK is taken from LSE"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == Sw::Lse
    }
}
#[doc = "Field `SW` writer - System clock switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sw>;
impl<'a, REG> SwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSCLK is taken from HSISYS (HSI16 prescaled by HSIDIV)"]
    #[inline(always)]
    pub fn hsisys(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::Hsisys)
    }
    #[doc = "SYSCLK is taken from HSE"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::Hse)
    }
    #[doc = "SYSCLK is taken from PLLRCLK"]
    #[inline(always)]
    pub fn pllrclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::Pllrclk)
    }
    #[doc = "SYSCLK is taken from LSI"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::Lsi)
    }
    #[doc = "SYSCLK is taken from LSE"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::Lse)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sws {
    #[doc = "0: SYSCLK is taken from HSISYS (HSI16 prescaled by HSIDIV)"]
    Hsisys = 0,
    #[doc = "1: SYSCLK is taken from HSE"]
    Hse = 1,
    #[doc = "2: SYSCLK is taken from PLLRCLK"]
    Pllrclk = 2,
    #[doc = "3: SYSCLK is taken from LSI"]
    Lsi = 3,
    #[doc = "4: SYSCLK is taken from LSE"]
    Lse = 4,
}
impl From<Sws> for u8 {
    #[inline(always)]
    fn from(variant: Sws) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sws {
    type Ux = u8;
}
impl crate::IsEnum for Sws {}
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SwsR = crate::FieldReader<Sws>;
impl SwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sws> {
        match self.bits {
            0 => Some(Sws::Hsisys),
            1 => Some(Sws::Hse),
            2 => Some(Sws::Pllrclk),
            3 => Some(Sws::Lsi),
            4 => Some(Sws::Lse),
            _ => None,
        }
    }
    #[doc = "SYSCLK is taken from HSISYS (HSI16 prescaled by HSIDIV)"]
    #[inline(always)]
    pub fn is_hsisys(&self) -> bool {
        *self == Sws::Hsisys
    }
    #[doc = "SYSCLK is taken from HSE"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == Sws::Hse
    }
    #[doc = "SYSCLK is taken from PLLRCLK"]
    #[inline(always)]
    pub fn is_pllrclk(&self) -> bool {
        *self == Sws::Pllrclk
    }
    #[doc = "SYSCLK is taken from LSI"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == Sws::Lsi
    }
    #[doc = "SYSCLK is taken from LSE"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == Sws::Lse
    }
}
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hpre {
    #[doc = "0: HCLK is SYSCLK divided by 1"]
    Prescale1 = 0,
    #[doc = "8: HCLK is SYSCLK divided by 2"]
    Prescale2 = 8,
    #[doc = "9: HCLK is SYSCLK divided by 4"]
    Prescale4 = 9,
    #[doc = "10: HCLK is SYSCLK divided by 8"]
    Prescale8 = 10,
    #[doc = "11: HCLK is SYSCLK divided by 16"]
    Prescale16 = 11,
    #[doc = "12: HCLK is SYSCLK divided by 64"]
    Prescale64 = 12,
    #[doc = "13: HCLK is SYSCLK divided by 128"]
    Prescale128 = 13,
    #[doc = "14: HCLK is SYSCLK divided by 256"]
    Prescale256 = 14,
    #[doc = "15: HCLK is SYSCLK divided by 512"]
    Prescale512 = 15,
}
impl From<Hpre> for u8 {
    #[inline(always)]
    fn from(variant: Hpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hpre {
    type Ux = u8;
}
impl crate::IsEnum for Hpre {}
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HpreR = crate::FieldReader<Hpre>;
impl HpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hpre> {
        match self.bits {
            0 => Some(Hpre::Prescale1),
            8 => Some(Hpre::Prescale2),
            9 => Some(Hpre::Prescale4),
            10 => Some(Hpre::Prescale8),
            11 => Some(Hpre::Prescale16),
            12 => Some(Hpre::Prescale64),
            13 => Some(Hpre::Prescale128),
            14 => Some(Hpre::Prescale256),
            15 => Some(Hpre::Prescale512),
            _ => None,
        }
    }
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn is_prescale1(&self) -> bool {
        *self == Hpre::Prescale1
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_prescale2(&self) -> bool {
        *self == Hpre::Prescale2
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_prescale4(&self) -> bool {
        *self == Hpre::Prescale4
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_prescale8(&self) -> bool {
        *self == Hpre::Prescale8
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_prescale16(&self) -> bool {
        *self == Hpre::Prescale16
    }
    #[doc = "HCLK is SYSCLK divided by 64"]
    #[inline(always)]
    pub fn is_prescale64(&self) -> bool {
        *self == Hpre::Prescale64
    }
    #[doc = "HCLK is SYSCLK divided by 128"]
    #[inline(always)]
    pub fn is_prescale128(&self) -> bool {
        *self == Hpre::Prescale128
    }
    #[doc = "HCLK is SYSCLK divided by 256"]
    #[inline(always)]
    pub fn is_prescale256(&self) -> bool {
        *self == Hpre::Prescale256
    }
    #[doc = "HCLK is SYSCLK divided by 512"]
    #[inline(always)]
    pub fn is_prescale512(&self) -> bool {
        *self == Hpre::Prescale512
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hpre>;
impl<'a, REG> HpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn prescale1(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale1)
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn prescale2(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale2)
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn prescale4(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale4)
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn prescale8(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale8)
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn prescale16(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale16)
    }
    #[doc = "HCLK is SYSCLK divided by 64"]
    #[inline(always)]
    pub fn prescale64(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale64)
    }
    #[doc = "HCLK is SYSCLK divided by 128"]
    #[inline(always)]
    pub fn prescale128(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale128)
    }
    #[doc = "HCLK is SYSCLK divided by 256"]
    #[inline(always)]
    pub fn prescale256(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale256)
    }
    #[doc = "HCLK is SYSCLK divided by 512"]
    #[inline(always)]
    pub fn prescale512(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::Prescale512)
    }
}
#[doc = "APB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ppre {
    #[doc = "0: HCLK is SYSCLK divided by 1"]
    Prescale1 = 0,
    #[doc = "4: HCLK is SYSCLK divided by 2"]
    Prescale2 = 4,
    #[doc = "5: HCLK is SYSCLK divided by 4"]
    Prescale4 = 5,
    #[doc = "6: HCLK is SYSCLK divided by 8"]
    Prescale8 = 6,
    #[doc = "7: HCLK is SYSCLK divided by 16"]
    Prescale16 = 7,
}
impl From<Ppre> for u8 {
    #[inline(always)]
    fn from(variant: Ppre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ppre {
    type Ux = u8;
}
impl crate::IsEnum for Ppre {}
#[doc = "Field `PPRE` reader - APB prescaler"]
pub type PpreR = crate::FieldReader<Ppre>;
impl PpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ppre> {
        match self.bits {
            0 => Some(Ppre::Prescale1),
            4 => Some(Ppre::Prescale2),
            5 => Some(Ppre::Prescale4),
            6 => Some(Ppre::Prescale8),
            7 => Some(Ppre::Prescale16),
            _ => None,
        }
    }
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn is_prescale1(&self) -> bool {
        *self == Ppre::Prescale1
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_prescale2(&self) -> bool {
        *self == Ppre::Prescale2
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_prescale4(&self) -> bool {
        *self == Ppre::Prescale4
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_prescale8(&self) -> bool {
        *self == Ppre::Prescale8
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_prescale16(&self) -> bool {
        *self == Ppre::Prescale16
    }
}
#[doc = "Field `PPRE` writer - APB prescaler"]
pub type PpreW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ppre>;
impl<'a, REG> PpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn prescale1(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::Prescale1)
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn prescale2(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::Prescale2)
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn prescale4(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::Prescale4)
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn prescale8(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::Prescale8)
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn prescale16(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::Prescale16)
    }
}
#[doc = "Field `MCO2SEL` reader - MCO2SEL"]
pub type Mco2selR = crate::FieldReader;
#[doc = "Field `MCO2SEL` writer - MCO2SEL"]
pub type Mco2selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCO2PRE` reader - MCO2PRE"]
pub type Mco2preR = crate::FieldReader;
#[doc = "Field `MCO2PRE` writer - MCO2PRE"]
pub type Mco2preW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type McoselR = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type McopreR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    pub fn ppre(&self) -> PpreR {
        PpreR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&self) -> Mco2selR {
        Mco2selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MCO2PRE"]
    #[inline(always)]
    pub fn mco2pre(&self) -> Mco2preR {
        Mco2preR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> McopreR {
        McopreR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HpreW<CfgrSpec> {
        HpreW::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ppre(&mut self) -> PpreW<CfgrSpec> {
        PpreW::new(self, 12)
    }
    #[doc = "Bits 16:19 - MCO2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn mco2sel(&mut self) -> Mco2selW<CfgrSpec> {
        Mco2selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - MCO2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> Mco2preW<CfgrSpec> {
        Mco2preW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> McoselW<CfgrSpec> {
        McoselW::new(self, 24)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
