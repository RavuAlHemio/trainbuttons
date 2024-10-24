#[doc = "Register `CFR` writer"]
pub type W = crate::W<CfrSpec>;
#[doc = "Field `CSOF0` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF1` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF2` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF3` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF4` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF5` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF6` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> Csof0W<CfrSpec> {
        Csof0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> Csof1W<CfrSpec> {
        Csof1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> Csof2W<CfrSpec> {
        Csof2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> Csof3W<CfrSpec> {
        Csof3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> Csof4W<CfrSpec> {
        Csof4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> Csof5W<CfrSpec> {
        Csof5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> Csof6W<CfrSpec> {
        Csof6W::new(self, 6)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfrSpec;
impl crate::RegisterSpec for CfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CfrSpec {
    const RESET_VALUE: u32 = 0;
}
