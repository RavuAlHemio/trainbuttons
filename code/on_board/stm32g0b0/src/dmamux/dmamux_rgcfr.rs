#[doc = "Register `DMAMUX_RGCFR` writer"]
pub type W = crate::W<DmamuxRgcfrSpec>;
#[doc = "Field `COF0` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type Cof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF1` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type Cof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF2` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type Cof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF3` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type Cof3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> Cof0W<DmamuxRgcfrSpec> {
        Cof0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> Cof1W<DmamuxRgcfrSpec> {
        Cof1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> Cof2W<DmamuxRgcfrSpec> {
        Cof2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> Cof3W<DmamuxRgcfrSpec> {
        Cof3W::new(self, 3)
    }
}
#[doc = "DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamuxRgcfrSpec;
impl crate::RegisterSpec for DmamuxRgcfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmamux_rgcfr::W`](W) writer structure"]
impl crate::Writable for DmamuxRgcfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_RGCFR to value 0"]
impl crate::Resettable for DmamuxRgcfrSpec {
    const RESET_VALUE: u32 = 0;
}
