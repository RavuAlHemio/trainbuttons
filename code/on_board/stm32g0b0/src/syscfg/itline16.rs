#[doc = "Register `ITLINE16` reader"]
pub type R = crate::R<Itline16Spec>;
#[doc = "Field `TIM3` reader - TIM3"]
pub type Tim3R = crate::BitReader;
#[doc = "Field `TIM4` reader - TIM4"]
pub type Tim4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM4"]
    #[inline(always)]
    pub fn tim4(&self) -> Tim4R {
        Tim4R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline16Spec;
impl crate::RegisterSpec for Itline16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline16::R`](R) reader structure"]
impl crate::Readable for Itline16Spec {}
#[doc = "`reset()` method sets ITLINE16 to value 0"]
impl crate::Resettable for Itline16Spec {
    const RESET_VALUE: u32 = 0;
}
