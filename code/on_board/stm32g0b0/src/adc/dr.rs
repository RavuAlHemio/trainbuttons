#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `DATA` reader - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on page349. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on page349. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
