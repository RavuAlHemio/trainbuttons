#[doc = "Register `DMA_CMAR7` reader"]
pub type R = crate::R<DmaCmar7Spec>;
#[doc = "Register `DMA_CMAR7` writer"]
pub type W = crate::W<DmaCmar7Spec>;
#[doc = "Field `MA` reader - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]=01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE=10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR=1 and the memory destination address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR=1 and the peripheral destination address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]=01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE=10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR=1 and the memory destination address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR=1 and the peripheral destination address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]=01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE=10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR=1 and the memory destination address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR=1 and the peripheral destination address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]=01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE=10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR=1 and the memory destination address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR=1 and the peripheral destination address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MaW<DmaCmar7Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cmar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCmar7Spec;
impl crate::RegisterSpec for DmaCmar7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cmar7::R`](R) reader structure"]
impl crate::Readable for DmaCmar7Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cmar7::W`](W) writer structure"]
impl crate::Writable for DmaCmar7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CMAR7 to value 0"]
impl crate::Resettable for DmaCmar7Spec {
    const RESET_VALUE: u32 = 0;
}
