#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel0R = crate::FieldReader;
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel1R = crate::FieldReader;
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel2R = crate::FieldReader;
#[doc = "Field `AFSEL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel3R = crate::FieldReader;
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel4R = crate::FieldReader;
#[doc = "Field `AFSEL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel5R = crate::FieldReader;
#[doc = "Field `AFSEL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel6R = crate::FieldReader;
#[doc = "Field `AFSEL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel7R = crate::FieldReader;
#[doc = "Field `AFSEL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel0(&self) -> Afsel0R {
        Afsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel1(&self) -> Afsel1R {
        Afsel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel2(&self) -> Afsel2R {
        Afsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel3(&self) -> Afsel3R {
        Afsel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel4(&self) -> Afsel4R {
        Afsel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel5(&self) -> Afsel5R {
        Afsel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel6(&self) -> Afsel6R {
        Afsel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel7(&self) -> Afsel7R {
        Afsel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> Afsel0W<AfrlSpec> {
        Afsel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> Afsel1W<AfrlSpec> {
        Afsel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> Afsel2W<AfrlSpec> {
        Afsel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> Afsel3W<AfrlSpec> {
        Afsel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> Afsel4W<AfrlSpec> {
        Afsel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> Afsel5W<AfrlSpec> {
        Afsel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> Afsel6W<AfrlSpec> {
        Afsel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> Afsel7W<AfrlSpec> {
        Afsel7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {
    const RESET_VALUE: u32 = 0;
}
