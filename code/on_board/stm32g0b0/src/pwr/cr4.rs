#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<Cr4Spec>;
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity"]
pub type Wp1R = crate::BitReader;
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity"]
pub type Wp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity"]
pub type Wp2R = crate::BitReader;
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity"]
pub type Wp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity"]
pub type Wp3R = crate::BitReader;
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity"]
pub type Wp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP4` reader - Wakeup pin WKUP4 polarity"]
pub type Wp4R = crate::BitReader;
#[doc = "Field `WP4` writer - Wakeup pin WKUP4 polarity"]
pub type Wp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP5` reader - Wakeup pin WKUP5 polarity"]
pub type Wp5R = crate::BitReader;
#[doc = "Field `WP5` writer - Wakeup pin WKUP5 polarity"]
pub type Wp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP6` reader - WKUP6 wakeup pin polarity"]
pub type Wp6R = crate::BitReader;
#[doc = "Field `WP6` writer - WKUP6 wakeup pin polarity"]
pub type Wp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub type VbeR = crate::BitReader;
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub type VbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub type VbrsR = crate::BitReader;
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub type VbrsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wp1(&self) -> Wp1R {
        Wp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wp2(&self) -> Wp2R {
        Wp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wp3(&self) -> Wp3R {
        Wp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    pub fn wp4(&self) -> Wp4R {
        Wp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    pub fn wp5(&self) -> Wp5R {
        Wp5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUP6 wakeup pin polarity"]
    #[inline(always)]
    pub fn wp6(&self) -> Wp6R {
        Wp6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VbeR {
        VbeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VbrsR {
        VbrsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> Wp1W<Cr4Spec> {
        Wp1W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> Wp2W<Cr4Spec> {
        Wp2W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> Wp3W<Cr4Spec> {
        Wp3W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> Wp4W<Cr4Spec> {
        Wp4W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> Wp5W<Cr4Spec> {
        Wp5W::new(self, 4)
    }
    #[doc = "Bit 5 - WKUP6 wakeup pin polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp6(&mut self) -> Wp6W<Cr4Spec> {
        Wp6W::new(self, 5)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VbeW<Cr4Spec> {
        VbeW::new(self, 8)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VbrsW<Cr4Spec> {
        VbrsW::new(self, 9)
    }
}
#[doc = "Power control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for Cr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for Cr4Spec {
    const RESET_VALUE: u32 = 0;
}
