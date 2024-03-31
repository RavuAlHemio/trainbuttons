#[doc = "Register `I2C_PECR` reader"]
pub type R = crate::R<I2cPecrSpec>;
#[doc = "Field `PEC` reader - Packet error checking register"]
pub type PecR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_pecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cPecrSpec;
impl crate::RegisterSpec for I2cPecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_pecr::R`](R) reader structure"]
impl crate::Readable for I2cPecrSpec {}
#[doc = "`reset()` method sets I2C_PECR to value 0"]
impl crate::Resettable for I2cPecrSpec {
    const RESET_VALUE: u32 = 0;
}
