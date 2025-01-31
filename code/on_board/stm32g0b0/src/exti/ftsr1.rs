#[doc = "Register `FTSR1` reader"]
pub type R = crate::R<Ftsr1Spec>;
#[doc = "Register `FTSR1` writer"]
pub type W = crate::W<Ftsr1Spec>;
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft0R = crate::BitReader;
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft1R = crate::BitReader;
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft2R = crate::BitReader;
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft3R = crate::BitReader;
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft4R = crate::BitReader;
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft5R = crate::BitReader;
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft6R = crate::BitReader;
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft7R = crate::BitReader;
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT8` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft8R = crate::BitReader;
#[doc = "Field `FT8` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT9` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft9R = crate::BitReader;
#[doc = "Field `FT9` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT10` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft10R = crate::BitReader;
#[doc = "Field `FT10` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft11R = crate::BitReader;
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT12` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft12R = crate::BitReader;
#[doc = "Field `FT12` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT13` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft13R = crate::BitReader;
#[doc = "Field `FT13` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT14` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft14R = crate::BitReader;
#[doc = "Field `FT14` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT15` reader - Falling trigger event configuration bit of configurable line"]
pub type Ft15R = crate::BitReader;
#[doc = "Field `FT15` writer - Falling trigger event configuration bit of configurable line"]
pub type Ft15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft0(&self) -> Ft0R {
        Ft0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft1(&self) -> Ft1R {
        Ft1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft2(&self) -> Ft2R {
        Ft2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft3(&self) -> Ft3R {
        Ft3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft4(&self) -> Ft4R {
        Ft4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft5(&self) -> Ft5R {
        Ft5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft6(&self) -> Ft6R {
        Ft6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft7(&self) -> Ft7R {
        Ft7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft8(&self) -> Ft8R {
        Ft8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft9(&self) -> Ft9R {
        Ft9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft10(&self) -> Ft10R {
        Ft10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft11(&self) -> Ft11R {
        Ft11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft12(&self) -> Ft12R {
        Ft12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft13(&self) -> Ft13R {
        Ft13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft14(&self) -> Ft14R {
        Ft14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft15(&self) -> Ft15R {
        Ft15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> Ft0W<Ftsr1Spec> {
        Ft0W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> Ft1W<Ftsr1Spec> {
        Ft1W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> Ft2W<Ftsr1Spec> {
        Ft2W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> Ft3W<Ftsr1Spec> {
        Ft3W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> Ft4W<Ftsr1Spec> {
        Ft4W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> Ft5W<Ftsr1Spec> {
        Ft5W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> Ft6W<Ftsr1Spec> {
        Ft6W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> Ft7W<Ftsr1Spec> {
        Ft7W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft8(&mut self) -> Ft8W<Ftsr1Spec> {
        Ft8W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft9(&mut self) -> Ft9W<Ftsr1Spec> {
        Ft9W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft10(&mut self) -> Ft10W<Ftsr1Spec> {
        Ft10W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> Ft11W<Ftsr1Spec> {
        Ft11W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft12(&mut self) -> Ft12W<Ftsr1Spec> {
        Ft12W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft13(&mut self) -> Ft13W<Ftsr1Spec> {
        Ft13W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft14(&mut self) -> Ft14W<Ftsr1Spec> {
        Ft14W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft15(&mut self) -> Ft15W<Ftsr1Spec> {
        Ft15W::new(self, 15)
    }
}
#[doc = "EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ftsr1Spec;
impl crate::RegisterSpec for Ftsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr1::R`](R) reader structure"]
impl crate::Readable for Ftsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ftsr1::W`](W) writer structure"]
impl crate::Writable for Ftsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR1 to value 0"]
impl crate::Resettable for Ftsr1Spec {
    const RESET_VALUE: u32 = 0;
}
