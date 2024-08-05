#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1"]
pub type Ewup1R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1"]
pub type Ewup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2"]
pub type Ewup2R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2"]
pub type Ewup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3"]
pub type Ewup3R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3"]
pub type Ewup3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4"]
pub type Ewup4R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4"]
pub type Ewup4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP5` reader - Enable WKUP5 wakeup pin"]
pub type Ewup5R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable WKUP5 wakeup pin"]
pub type Ewup5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP6` reader - Enable WKUP6 wakeup pin"]
pub type Ewup6R = crate::BitReader;
#[doc = "Field `EWUP6` writer - Enable WKUP6 wakeup pin"]
pub type Ewup6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration"]
pub type ApcR = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration"]
pub type ApcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWUL` reader - Enable internal wakeup line"]
pub type EiwulR = crate::BitReader;
#[doc = "Field `EIWUL` writer - Enable internal wakeup line"]
pub type EiwulW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> Ewup1R {
        Ewup1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> Ewup2R {
        Ewup2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> Ewup3R {
        Ewup3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> Ewup4R {
        Ewup4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable WKUP5 wakeup pin"]
    #[inline(always)]
    pub fn ewup5(&self) -> Ewup5R {
        Ewup5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable WKUP6 wakeup pin"]
    #[inline(always)]
    pub fn ewup6(&self) -> Ewup6R {
        Ewup6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&self) -> ApcR {
        ApcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    pub fn eiwul(&self) -> EiwulR {
        EiwulR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> Ewup1W<Cr3Spec> {
        Ewup1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> Ewup2W<Cr3Spec> {
        Ewup2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> Ewup3W<Cr3Spec> {
        Ewup3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> Ewup4W<Cr3Spec> {
        Ewup4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable WKUP5 wakeup pin"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> Ewup5W<Cr3Spec> {
        Ewup5W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable WKUP6 wakeup pin"]
    #[inline(always)]
    #[must_use]
    pub fn ewup6(&mut self) -> Ewup6W<Cr3Spec> {
        Ewup6W::new(self, 5)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> ApcW<Cr3Spec> {
        ApcW::new(self, 10)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EiwulW<Cr3Spec> {
        EiwulW::new(self, 15)
    }
}
#[doc = "Power control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x8000"]
impl crate::Resettable for Cr3Spec {
    const RESET_VALUE: u32 = 0x8000;
}
