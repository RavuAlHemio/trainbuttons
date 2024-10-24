#[doc = "Register `ITLINE9` reader"]
pub type R = crate::R<Itline9Spec>;
#[doc = "Field `DMA1_CH1` reader - DMA1_CH1"]
pub type Dma1Ch1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA1_CH1"]
    #[inline(always)]
    pub fn dma1_ch1(&self) -> Dma1Ch1R {
        Dma1Ch1R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 9 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline9Spec;
impl crate::RegisterSpec for Itline9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline9::R`](R) reader structure"]
impl crate::Readable for Itline9Spec {}
#[doc = "`reset()` method sets ITLINE9 to value 0"]
impl crate::Resettable for Itline9Spec {
    const RESET_VALUE: u32 = 0;
}
