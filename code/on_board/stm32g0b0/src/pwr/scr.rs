#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub type Cwuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub type Cwuf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub type Cwuf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4"]
pub type Cwuf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5"]
pub type Cwuf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF6` writer - Clear wakeup flag 6"]
pub type Cwuf6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CsbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> Cwuf1W<ScrSpec> {
        Cwuf1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> Cwuf2W<ScrSpec> {
        Cwuf2W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> Cwuf3W<ScrSpec> {
        Cwuf3W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> Cwuf4W<ScrSpec> {
        Cwuf4W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> Cwuf5W<ScrSpec> {
        Cwuf5W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear wakeup flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf6(&mut self) -> Cwuf6W<ScrSpec> {
        Cwuf6W::new(self, 5)
    }
    #[doc = "Bit 8 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CsbfW<ScrSpec> {
        CsbfW::new(self, 8)
    }
}
#[doc = "Power status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0;
}
