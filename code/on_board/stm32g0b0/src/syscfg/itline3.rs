#[doc = "Register `ITLINE3` reader"]
pub type R = crate::R<Itline3Spec>;
#[doc = "Field `FLASH_ITF` reader - FLASH_ITF"]
pub type FlashItfR = crate::BitReader;
#[doc = "Field `FLASH_ECC` reader - FLASH_ECC"]
pub type FlashEccR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FLASH_ITF"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FlashItfR {
        FlashItfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH_ECC"]
    #[inline(always)]
    pub fn flash_ecc(&self) -> FlashEccR {
        FlashEccR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 3 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline3Spec;
impl crate::RegisterSpec for Itline3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline3::R`](R) reader structure"]
impl crate::Readable for Itline3Spec {}
#[doc = "`reset()` method sets ITLINE3 to value 0"]
impl crate::Resettable for Itline3Spec {
    const RESET_VALUE: u32 = 0;
}
