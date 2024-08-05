#[doc = "Register `ITLINE25` reader"]
pub type R = crate::R<Itline25Spec>;
#[doc = "Field `SPI1` reader - SPI1"]
pub type Spi1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 25 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline25Spec;
impl crate::RegisterSpec for Itline25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline25::R`](R) reader structure"]
impl crate::Readable for Itline25Spec {}
#[doc = "`reset()` method sets ITLINE25 to value 0"]
impl crate::Resettable for Itline25Spec {
    const RESET_VALUE: u32 = 0;
}
