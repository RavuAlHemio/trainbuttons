#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Field `WUF1` reader - Wakeup flag 1"]
pub type Wuf1R = crate::BitReader;
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub type Wuf2R = crate::BitReader;
#[doc = "Field `WUF3` reader - Wakeup flag 3"]
pub type Wuf3R = crate::BitReader;
#[doc = "Field `WUF4` reader - Wakeup flag 4"]
pub type Wuf4R = crate::BitReader;
#[doc = "Field `WUF5` reader - Wakeup flag 5"]
pub type Wuf5R = crate::BitReader;
#[doc = "Field `WUF6` reader - Wakeup flag 6"]
pub type Wuf6R = crate::BitReader;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SbfR = crate::BitReader;
#[doc = "Field `WUFI` reader - Wakeup flag internal"]
pub type WufiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> Wuf1R {
        Wuf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> Wuf2R {
        Wuf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&self) -> Wuf3R {
        Wuf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> Wuf4R {
        Wuf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> Wuf5R {
        Wuf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag 6"]
    #[inline(always)]
    pub fn wuf6(&self) -> Wuf6R {
        Wuf6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WufiR {
        WufiR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for Sr1Spec {
    const RESET_VALUE: u32 = 0;
}
