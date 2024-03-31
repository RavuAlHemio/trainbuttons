#[doc = "Register `ITLINE10` reader"]
pub type R = crate::R<Itline10Spec>;
#[doc = "Field `DMA1_CH2` reader - DMA1_CH1"]
pub type Dma1Ch2R = crate::BitReader;
#[doc = "Field `DMA1_CH3` reader - DMA1_CH3"]
pub type Dma1Ch3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA1_CH1"]
    #[inline(always)]
    pub fn dma1_ch2(&self) -> Dma1Ch2R {
        Dma1Ch2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1_CH3"]
    #[inline(always)]
    pub fn dma1_ch3(&self) -> Dma1Ch3R {
        Dma1Ch3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 10 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline10Spec;
impl crate::RegisterSpec for Itline10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline10::R`](R) reader structure"]
impl crate::Readable for Itline10Spec {}
#[doc = "`reset()` method sets ITLINE10 to value 0"]
impl crate::Resettable for Itline10Spec {
    const RESET_VALUE: u32 = 0;
}
