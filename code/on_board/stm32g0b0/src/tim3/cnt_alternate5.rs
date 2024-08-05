#[doc = "Register `CNT_ALTERNATE5` reader"]
pub type R = crate::R<CntAlternate5Spec>;
#[doc = "Register `CNT_ALTERNATE5` writer"]
pub type W = crate::W<CntAlternate5Spec>;
#[doc = "Field `CNT` reader - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UifcpyR = crate::BitReader;
#[doc = "Field `UIFCPY` writer - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UifcpyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UifcpyR {
        UifcpyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CntAlternate5Spec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UifcpyW<CntAlternate5Spec> {
        UifcpyW::new(self, 31)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt_alternate5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt_alternate5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntAlternate5Spec;
impl crate::RegisterSpec for CntAlternate5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt_alternate5::R`](R) reader structure"]
impl crate::Readable for CntAlternate5Spec {}
#[doc = "`write(|w| ..)` method takes [`cnt_alternate5::W`](W) writer structure"]
impl crate::Writable for CntAlternate5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT_ALTERNATE5 to value 0"]
impl crate::Resettable for CntAlternate5Spec {
    const RESET_VALUE: u32 = 0;
}
