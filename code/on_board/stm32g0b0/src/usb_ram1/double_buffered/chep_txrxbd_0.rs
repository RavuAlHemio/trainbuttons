#[doc = "Register `CHEP_TXRXBD_0` reader"]
pub type R = crate::R<ChepTxrxbd0Spec>;
#[doc = "Register `CHEP_TXRXBD_0` writer"]
pub type W = crate::W<ChepTxrxbd0Spec>;
#[doc = "Field `ADDR_TX` reader - These bits point to the starting address of the packet buffer containing data to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
pub type AddrTxR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_TX` writer - These bits point to the starting address of the packet buffer containing data to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
pub type AddrTxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COUNT_TX` reader - These bits contain the number of bytes to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it."]
pub type CountTxR = crate::FieldReader<u16>;
#[doc = "Field `COUNT_TX` writer - These bits contain the number of bytes to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it."]
pub type CountTxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - These bits point to the starting address of the packet buffer containing data to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
    #[inline(always)]
    pub fn addr_tx(&self) -> AddrTxR {
        AddrTxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - These bits contain the number of bytes to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it."]
    #[inline(always)]
    pub fn count_tx(&self) -> CountTxR {
        CountTxR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits point to the starting address of the packet buffer containing data to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
    #[inline(always)]
    #[must_use]
    pub fn addr_tx(&mut self) -> AddrTxW<ChepTxrxbd0Spec> {
        AddrTxW::new(self, 0)
    }
    #[doc = "Bits 16:25 - These bits contain the number of bytes to be transmitted by the endpoint/channel associated with the USB_CHEPnR register at the next IN token addressed to it."]
    #[inline(always)]
    #[must_use]
    pub fn count_tx(&mut self) -> CountTxW<ChepTxrxbd0Spec> {
        CountTxW::new(self, 16)
    }
}
#[doc = "Channel/endpoint transmit buffer descriptor 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chep_txrxbd_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep_txrxbd_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChepTxrxbd0Spec;
impl crate::RegisterSpec for ChepTxrxbd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chep_txrxbd_0::R`](R) reader structure"]
impl crate::Readable for ChepTxrxbd0Spec {}
#[doc = "`write(|w| ..)` method takes [`chep_txrxbd_0::W`](W) writer structure"]
impl crate::Writable for ChepTxrxbd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHEP_TXRXBD_0 to value 0"]
impl crate::Resettable for ChepTxrxbd0Spec {
    const RESET_VALUE: u32 = 0;
}
