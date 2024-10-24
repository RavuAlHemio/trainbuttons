#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RgsrSpec>;
#[doc = "Field `OF0` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub type Of0R = crate::BitReader;
#[doc = "Field `OF1` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub type Of1R = crate::BitReader;
#[doc = "Field `OF2` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub type Of2R = crate::BitReader;
#[doc = "Field `OF3` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub type Of3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of0(&self) -> Of0R {
        Of0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of1(&self) -> Of1R {
        Of1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of2(&self) -> Of2R {
        Of2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of3(&self) -> Of3R {
        Of3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMAMUX request generator interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgsrSpec;
impl crate::RegisterSpec for RgsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RgsrSpec {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RgsrSpec {
    const RESET_VALUE: u32 = 0;
}
