#[doc = "Register `ITLINE23` reader"]
pub type R = crate::R<Itline23Spec>;
#[doc = "Field `I2C1` reader - I2C1"]
pub type I2c1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 23 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline23Spec;
impl crate::RegisterSpec for Itline23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline23::R`](R) reader structure"]
impl crate::Readable for Itline23Spec {}
#[doc = "`reset()` method sets ITLINE23 to value 0"]
impl crate::Resettable for Itline23Spec {
    const RESET_VALUE: u32 = 0;
}
