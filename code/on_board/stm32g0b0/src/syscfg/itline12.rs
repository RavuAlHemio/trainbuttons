#[doc = "Register `ITLINE12` reader"]
pub type R = crate::R<Itline12Spec>;
#[doc = "Field `ADC` reader - ADC"]
pub type AdcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 12 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline12Spec;
impl crate::RegisterSpec for Itline12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline12::R`](R) reader structure"]
impl crate::Readable for Itline12Spec {}
#[doc = "`reset()` method sets ITLINE12 to value 0"]
impl crate::Resettable for Itline12Spec {
    const RESET_VALUE: u32 = 0;
}
