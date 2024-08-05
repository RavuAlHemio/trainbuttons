#[doc = "Register `ITLINE26` reader"]
pub type R = crate::R<Itline26Spec>;
#[doc = "Field `SPI2` reader - SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI3` reader - SPI3"]
pub type Spi3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 14 - SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "interrupt line 26 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline26Spec;
impl crate::RegisterSpec for Itline26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline26::R`](R) reader structure"]
impl crate::Readable for Itline26Spec {}
#[doc = "`reset()` method sets ITLINE26 to value 0"]
impl crate::Resettable for Itline26Spec {
    const RESET_VALUE: u32 = 0;
}
