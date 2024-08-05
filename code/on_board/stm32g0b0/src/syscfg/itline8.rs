#[doc = "Register `ITLINE8` reader"]
pub type R = crate::R<Itline8Spec>;
#[doc = "Field `USB` reader - USB"]
pub type UsbR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "interrupt line 8 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline8Spec;
impl crate::RegisterSpec for Itline8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline8::R`](R) reader structure"]
impl crate::Readable for Itline8Spec {}
#[doc = "`reset()` method sets ITLINE8 to value 0"]
impl crate::Resettable for Itline8Spec {
    const RESET_VALUE: u32 = 0;
}
