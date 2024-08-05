#[doc = "Register `FPR1` reader"]
pub type R = crate::R<Fpr1Spec>;
#[doc = "Register `FPR1` writer"]
pub type W = crate::W<Fpr1Spec>;
#[doc = "Field `FPIF0` reader - Falling edge event pending for configurable line"]
pub type Fpif0R = crate::BitReader;
#[doc = "Field `FPIF0` writer - Falling edge event pending for configurable line"]
pub type Fpif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF1` reader - Falling edge event pending for configurable line"]
pub type Fpif1R = crate::BitReader;
#[doc = "Field `FPIF1` writer - Falling edge event pending for configurable line"]
pub type Fpif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF2` reader - Falling edge event pending for configurable line"]
pub type Fpif2R = crate::BitReader;
#[doc = "Field `FPIF2` writer - Falling edge event pending for configurable line"]
pub type Fpif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF3` reader - Falling edge event pending for configurable line"]
pub type Fpif3R = crate::BitReader;
#[doc = "Field `FPIF3` writer - Falling edge event pending for configurable line"]
pub type Fpif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF4` reader - Falling edge event pending for configurable line"]
pub type Fpif4R = crate::BitReader;
#[doc = "Field `FPIF4` writer - Falling edge event pending for configurable line"]
pub type Fpif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF5` reader - Falling edge event pending for configurable line"]
pub type Fpif5R = crate::BitReader;
#[doc = "Field `FPIF5` writer - Falling edge event pending for configurable line"]
pub type Fpif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF6` reader - Falling edge event pending for configurable line"]
pub type Fpif6R = crate::BitReader;
#[doc = "Field `FPIF6` writer - Falling edge event pending for configurable line"]
pub type Fpif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF7` reader - Falling edge event pending for configurable line"]
pub type Fpif7R = crate::BitReader;
#[doc = "Field `FPIF7` writer - Falling edge event pending for configurable line"]
pub type Fpif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF8` reader - Falling edge event pending for configurable line"]
pub type Fpif8R = crate::BitReader;
#[doc = "Field `FPIF8` writer - Falling edge event pending for configurable line"]
pub type Fpif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF9` reader - Falling edge event pending for configurable line"]
pub type Fpif9R = crate::BitReader;
#[doc = "Field `FPIF9` writer - Falling edge event pending for configurable line"]
pub type Fpif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF10` reader - Falling edge event pending for configurable line"]
pub type Fpif10R = crate::BitReader;
#[doc = "Field `FPIF10` writer - Falling edge event pending for configurable line"]
pub type Fpif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF11` reader - Falling edge event pending for configurable line"]
pub type Fpif11R = crate::BitReader;
#[doc = "Field `FPIF11` writer - Falling edge event pending for configurable line"]
pub type Fpif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF12` reader - Falling edge event pending for configurable line"]
pub type Fpif12R = crate::BitReader;
#[doc = "Field `FPIF12` writer - Falling edge event pending for configurable line"]
pub type Fpif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF13` reader - Falling edge event pending for configurable line"]
pub type Fpif13R = crate::BitReader;
#[doc = "Field `FPIF13` writer - Falling edge event pending for configurable line"]
pub type Fpif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF14` reader - Falling edge event pending for configurable line"]
pub type Fpif14R = crate::BitReader;
#[doc = "Field `FPIF14` writer - Falling edge event pending for configurable line"]
pub type Fpif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF15` reader - Falling edge event pending for configurable line"]
pub type Fpif15R = crate::BitReader;
#[doc = "Field `FPIF15` writer - Falling edge event pending for configurable line"]
pub type Fpif15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif0(&self) -> Fpif0R {
        Fpif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif1(&self) -> Fpif1R {
        Fpif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif2(&self) -> Fpif2R {
        Fpif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif3(&self) -> Fpif3R {
        Fpif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif4(&self) -> Fpif4R {
        Fpif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif5(&self) -> Fpif5R {
        Fpif5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif6(&self) -> Fpif6R {
        Fpif6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif7(&self) -> Fpif7R {
        Fpif7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif8(&self) -> Fpif8R {
        Fpif8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif9(&self) -> Fpif9R {
        Fpif9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif10(&self) -> Fpif10R {
        Fpif10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif11(&self) -> Fpif11R {
        Fpif11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif12(&self) -> Fpif12R {
        Fpif12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif13(&self) -> Fpif13R {
        Fpif13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif14(&self) -> Fpif14R {
        Fpif14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif15(&self) -> Fpif15R {
        Fpif15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif0(&mut self) -> Fpif0W<Fpr1Spec> {
        Fpif0W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif1(&mut self) -> Fpif1W<Fpr1Spec> {
        Fpif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif2(&mut self) -> Fpif2W<Fpr1Spec> {
        Fpif2W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif3(&mut self) -> Fpif3W<Fpr1Spec> {
        Fpif3W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif4(&mut self) -> Fpif4W<Fpr1Spec> {
        Fpif4W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif5(&mut self) -> Fpif5W<Fpr1Spec> {
        Fpif5W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif6(&mut self) -> Fpif6W<Fpr1Spec> {
        Fpif6W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif7(&mut self) -> Fpif7W<Fpr1Spec> {
        Fpif7W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif8(&mut self) -> Fpif8W<Fpr1Spec> {
        Fpif8W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif9(&mut self) -> Fpif9W<Fpr1Spec> {
        Fpif9W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif10(&mut self) -> Fpif10W<Fpr1Spec> {
        Fpif10W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif11(&mut self) -> Fpif11W<Fpr1Spec> {
        Fpif11W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif12(&mut self) -> Fpif12W<Fpr1Spec> {
        Fpif12W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif13(&mut self) -> Fpif13W<Fpr1Spec> {
        Fpif13W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif14(&mut self) -> Fpif14W<Fpr1Spec> {
        Fpif14W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn fpif15(&mut self) -> Fpif15W<Fpr1Spec> {
        Fpif15W::new(self, 15)
    }
}
#[doc = "EXTI falling edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpr1Spec;
impl crate::RegisterSpec for Fpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr1::R`](R) reader structure"]
impl crate::Readable for Fpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpr1::W`](W) writer structure"]
impl crate::Writable for Fpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPR1 to value 0"]
impl crate::Resettable for Fpr1Spec {
    const RESET_VALUE: u32 = 0;
}
