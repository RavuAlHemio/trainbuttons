#[doc = "Register `RPR1` reader"]
pub type R = crate::R<Rpr1Spec>;
#[doc = "Register `RPR1` writer"]
pub type W = crate::W<Rpr1Spec>;
#[doc = "Field `RPIF0` reader - Rising edge event pending for configurable line"]
pub type Rpif0R = crate::BitReader;
#[doc = "Field `RPIF0` writer - Rising edge event pending for configurable line"]
pub type Rpif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF1` reader - Rising edge event pending for configurable line"]
pub type Rpif1R = crate::BitReader;
#[doc = "Field `RPIF1` writer - Rising edge event pending for configurable line"]
pub type Rpif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF2` reader - Rising edge event pending for configurable line"]
pub type Rpif2R = crate::BitReader;
#[doc = "Field `RPIF2` writer - Rising edge event pending for configurable line"]
pub type Rpif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF3` reader - Rising edge event pending for configurable line"]
pub type Rpif3R = crate::BitReader;
#[doc = "Field `RPIF3` writer - Rising edge event pending for configurable line"]
pub type Rpif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF4` reader - Rising edge event pending for configurable line"]
pub type Rpif4R = crate::BitReader;
#[doc = "Field `RPIF4` writer - Rising edge event pending for configurable line"]
pub type Rpif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge Pending bit"]
pub type Rpif5R = crate::BitReader;
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge Pending bit"]
pub type Rpif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF6` reader - Rising edge event pending for configurable line"]
pub type Rpif6R = crate::BitReader;
#[doc = "Field `RPIF6` writer - Rising edge event pending for configurable line"]
pub type Rpif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF7` reader - Rising edge event pending for configurable line"]
pub type Rpif7R = crate::BitReader;
#[doc = "Field `RPIF7` writer - Rising edge event pending for configurable line"]
pub type Rpif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF8` reader - Rising edge event pending for configurable line"]
pub type Rpif8R = crate::BitReader;
#[doc = "Field `RPIF8` writer - Rising edge event pending for configurable line"]
pub type Rpif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF9` reader - Rising edge event pending for configurable line"]
pub type Rpif9R = crate::BitReader;
#[doc = "Field `RPIF9` writer - Rising edge event pending for configurable line"]
pub type Rpif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF10` reader - Rising edge event pending for configurable line"]
pub type Rpif10R = crate::BitReader;
#[doc = "Field `RPIF10` writer - Rising edge event pending for configurable line"]
pub type Rpif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF11` reader - Rising edge event pending for configurable line"]
pub type Rpif11R = crate::BitReader;
#[doc = "Field `RPIF11` writer - Rising edge event pending for configurable line"]
pub type Rpif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF12` reader - Rising edge event pending for configurable line"]
pub type Rpif12R = crate::BitReader;
#[doc = "Field `RPIF12` writer - Rising edge event pending for configurable line"]
pub type Rpif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF13` reader - Rising edge event pending for configurable line"]
pub type Rpif13R = crate::BitReader;
#[doc = "Field `RPIF13` writer - Rising edge event pending for configurable line"]
pub type Rpif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF14` reader - Rising edge event pending for configurable line"]
pub type Rpif14R = crate::BitReader;
#[doc = "Field `RPIF14` writer - Rising edge event pending for configurable line"]
pub type Rpif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF15` reader - Rising edge event pending for configurable line"]
pub type Rpif15R = crate::BitReader;
#[doc = "Field `RPIF15` writer - Rising edge event pending for configurable line"]
pub type Rpif15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif0(&self) -> Rpif0R {
        Rpif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif1(&self) -> Rpif1R {
        Rpif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif2(&self) -> Rpif2R {
        Rpif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif3(&self) -> Rpif3R {
        Rpif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif4(&self) -> Rpif4R {
        Rpif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    pub fn rpif5(&self) -> Rpif5R {
        Rpif5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif6(&self) -> Rpif6R {
        Rpif6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif7(&self) -> Rpif7R {
        Rpif7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif8(&self) -> Rpif8R {
        Rpif8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif9(&self) -> Rpif9R {
        Rpif9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif10(&self) -> Rpif10R {
        Rpif10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif11(&self) -> Rpif11R {
        Rpif11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif12(&self) -> Rpif12R {
        Rpif12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif13(&self) -> Rpif13R {
        Rpif13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif14(&self) -> Rpif14R {
        Rpif14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif15(&self) -> Rpif15R {
        Rpif15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif0(&mut self) -> Rpif0W<Rpr1Spec> {
        Rpif0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif1(&mut self) -> Rpif1W<Rpr1Spec> {
        Rpif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> Rpif2W<Rpr1Spec> {
        Rpif2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif3(&mut self) -> Rpif3W<Rpr1Spec> {
        Rpif3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif4(&mut self) -> Rpif4W<Rpr1Spec> {
        Rpif4W::new(self, 4)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif5(&mut self) -> Rpif5W<Rpr1Spec> {
        Rpif5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif6(&mut self) -> Rpif6W<Rpr1Spec> {
        Rpif6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif7(&mut self) -> Rpif7W<Rpr1Spec> {
        Rpif7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif8(&mut self) -> Rpif8W<Rpr1Spec> {
        Rpif8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif9(&mut self) -> Rpif9W<Rpr1Spec> {
        Rpif9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif10(&mut self) -> Rpif10W<Rpr1Spec> {
        Rpif10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif11(&mut self) -> Rpif11W<Rpr1Spec> {
        Rpif11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif12(&mut self) -> Rpif12W<Rpr1Spec> {
        Rpif12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif13(&mut self) -> Rpif13W<Rpr1Spec> {
        Rpif13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif14(&mut self) -> Rpif14W<Rpr1Spec> {
        Rpif14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif15(&mut self) -> Rpif15W<Rpr1Spec> {
        Rpif15W::new(self, 15)
    }
}
#[doc = "EXTI rising edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rpr1Spec;
impl crate::RegisterSpec for Rpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr1::R`](R) reader structure"]
impl crate::Readable for Rpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rpr1::W`](W) writer structure"]
impl crate::Writable for Rpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for Rpr1Spec {
    const RESET_VALUE: u32 = 0;
}
