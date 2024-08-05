#[doc = "Register `ITLINE24` reader"]
pub type R = crate::R<Itline24Spec>;
#[doc = "Field `I2C2` reader - I2C2"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C3` reader - I2C3"]
pub type I2c3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C3"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 24 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline24Spec;
impl crate::RegisterSpec for Itline24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline24::R`](R) reader structure"]
impl crate::Readable for Itline24Spec {}
#[doc = "`reset()` method sets ITLINE24 to value 0"]
impl crate::Resettable for Itline24Spec {
    const RESET_VALUE: u32 = 0;
}
