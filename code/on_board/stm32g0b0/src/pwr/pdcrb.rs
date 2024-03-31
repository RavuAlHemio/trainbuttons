#[doc = "Register `PDCRB` reader"]
pub type R = crate::R<PdcrbSpec>;
#[doc = "Register `PDCRB` writer"]
pub type W = crate::W<PdcrbSpec>;
#[doc = "Field `PD0` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd2R = crate::BitReader;
#[doc = "Field `PD2` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd4R = crate::BitReader;
#[doc = "Field `PD4` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd5R = crate::BitReader;
#[doc = "Field `PD5` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd6R = crate::BitReader;
#[doc = "Field `PD6` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd7R = crate::BitReader;
#[doc = "Field `PD7` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd8R = crate::BitReader;
#[doc = "Field `PD8` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd9R = crate::BitReader;
#[doc = "Field `PD9` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd10R = crate::BitReader;
#[doc = "Field `PD10` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd11R = crate::BitReader;
#[doc = "Field `PD11` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd12R = crate::BitReader;
#[doc = "Field `PD12` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd13R = crate::BitReader;
#[doc = "Field `PD13` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd14R = crate::BitReader;
#[doc = "Field `PD14` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15` reader - Port B pull-down bit y (y=0..15)"]
pub type Pd15R = crate::BitReader;
#[doc = "Field `PD15` writer - Port B pull-down bit y (y=0..15)"]
pub type Pd15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd11(&self) -> Pd11R {
        Pd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd12(&self) -> Pd12R {
        Pd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd13(&self) -> Pd13R {
        Pd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd14(&self) -> Pd14R {
        Pd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd15(&self) -> Pd15R {
        Pd15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> Pd0W<PdcrbSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> Pd1W<PdcrbSpec> {
        Pd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> Pd2W<PdcrbSpec> {
        Pd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> Pd3W<PdcrbSpec> {
        Pd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> Pd4W<PdcrbSpec> {
        Pd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> Pd5W<PdcrbSpec> {
        Pd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> Pd6W<PdcrbSpec> {
        Pd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> Pd7W<PdcrbSpec> {
        Pd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> Pd8W<PdcrbSpec> {
        Pd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> Pd9W<PdcrbSpec> {
        Pd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> Pd10W<PdcrbSpec> {
        Pd10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> Pd11W<PdcrbSpec> {
        Pd11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> Pd12W<PdcrbSpec> {
        Pd12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> Pd13W<PdcrbSpec> {
        Pd13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> Pd14W<PdcrbSpec> {
        Pd14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> Pd15W<PdcrbSpec> {
        Pd15W::new(self, 15)
    }
}
#[doc = "Power Port B pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcrbSpec;
impl crate::RegisterSpec for PdcrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrb::R`](R) reader structure"]
impl crate::Readable for PdcrbSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcrb::W`](W) writer structure"]
impl crate::Writable for PdcrbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRB to value 0"]
impl crate::Resettable for PdcrbSpec {
    const RESET_VALUE: u32 = 0;
}
