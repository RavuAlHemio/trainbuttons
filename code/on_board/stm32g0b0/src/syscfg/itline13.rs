#[doc = "Register `ITLINE13` reader"]
pub type R = crate::R<Itline13Spec>;
#[doc = "Field `TIM1_CCU` reader - TIM1_CCU"]
pub type Tim1CcuR = crate::BitReader;
#[doc = "Field `TIM1_TRG` reader - TIM1_TRG"]
pub type Tim1TrgR = crate::BitReader;
#[doc = "Field `TIM1_UPD` reader - TIM1_UPD"]
pub type Tim1UpdR = crate::BitReader;
#[doc = "Field `TIM1_BRK` reader - TIM1_BRK"]
pub type Tim1BrkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM1_CCU"]
    #[inline(always)]
    pub fn tim1_ccu(&self) -> Tim1CcuR {
        Tim1CcuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM1_TRG"]
    #[inline(always)]
    pub fn tim1_trg(&self) -> Tim1TrgR {
        Tim1TrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM1_UPD"]
    #[inline(always)]
    pub fn tim1_upd(&self) -> Tim1UpdR {
        Tim1UpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM1_BRK"]
    #[inline(always)]
    pub fn tim1_brk(&self) -> Tim1BrkR {
        Tim1BrkR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "interrupt line 13 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline13Spec;
impl crate::RegisterSpec for Itline13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline13::R`](R) reader structure"]
impl crate::Readable for Itline13Spec {}
#[doc = "`reset()` method sets ITLINE13 to value 0"]
impl crate::Resettable for Itline13Spec {
    const RESET_VALUE: u32 = 0;
}
