#[doc = "Register `ITLINE21` reader"]
pub type R = crate::R<Itline21Spec>;
#[doc = "Field `TIM16` reader - TIM16"]
pub type Tim16R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM16"]
    #[inline(always)]
    pub fn tim16(&self) -> Tim16R {
        Tim16R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline21Spec;
impl crate::RegisterSpec for Itline21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline21::R`](R) reader structure"]
impl crate::Readable for Itline21Spec {}
#[doc = "`reset()` method sets ITLINE21 to value 0"]
impl crate::Resettable for Itline21Spec {
    const RESET_VALUE: u32 = 0;
}
