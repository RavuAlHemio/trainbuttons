#[doc = "Register `CHEP_RXTXBD_0` reader"]
pub type R = crate::R<ChepRxtxbd0Spec>;
#[doc = "Register `CHEP_RXTXBD_0` writer"]
pub type W = crate::W<ChepRxtxbd0Spec>;
#[doc = "Field `ADDR_RX` reader - These bits point to the starting address of the packet buffer, which contains the data received by the endpoint/channel associated with the USB_CHEPnR register at the next OUT/SETUP token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
pub type AddrRxR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_RX` writer - These bits point to the starting address of the packet buffer, which contains the data received by the endpoint/channel associated with the USB_CHEPnR register at the next OUT/SETUP token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
pub type AddrRxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COUNT_RX` reader - These bits contain the number of bytes received by the endpoint/channel associated with the USB_CHEPnR register during the last OUT/SETUP transaction addressed to it."]
pub type CountRxR = crate::FieldReader<u16>;
#[doc = "Field `COUNT_RX` writer - These bits contain the number of bytes received by the endpoint/channel associated with the USB_CHEPnR register during the last OUT/SETUP transaction addressed to it."]
pub type CountRxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NUM_BLOCK` reader - These bits define the number of memory blocks allocated to this packet buffer. The actual amount of allocated memory depends on the BLSIZE value."]
pub type NumBlockR = crate::FieldReader;
#[doc = "Field `NUM_BLOCK` writer - These bits define the number of memory blocks allocated to this packet buffer. The actual amount of allocated memory depends on the BLSIZE value."]
pub type NumBlockW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "This bit selects the size of memory block used to define the allocated buffer area.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blsize {
    #[doc = "0: The memory block is 2-byte large, which is the minimum block allowed in a half-word wide memory. With this block size the allocated buffer size ranges from 2 to 62 bytes. The buffer size is NUM_BLOCK*2; NUM_BLOCK must not be 0."]
    Bytes2 = 0,
    #[doc = "1: The memory block is 32-byte large, which permits to reach the maximum packet length defined by USB specifications. With this block size the allocated buffer size theoretically ranges from 32 to 1024 bytes, which is the longest packet size allowed by USB standard specifications. However, the applicable size is limited by the available buffer memory. The buffer size is (NUM_BLOCK+1)*32."]
    Bytes32 = 1,
}
impl From<Blsize> for bool {
    #[inline(always)]
    fn from(variant: Blsize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLSIZE` reader - This bit selects the size of memory block used to define the allocated buffer area."]
pub type BlsizeR = crate::BitReader<Blsize>;
impl BlsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blsize {
        match self.bits {
            false => Blsize::Bytes2,
            true => Blsize::Bytes32,
        }
    }
    #[doc = "The memory block is 2-byte large, which is the minimum block allowed in a half-word wide memory. With this block size the allocated buffer size ranges from 2 to 62 bytes. The buffer size is NUM_BLOCK*2; NUM_BLOCK must not be 0."]
    #[inline(always)]
    pub fn is_bytes_2(&self) -> bool {
        *self == Blsize::Bytes2
    }
    #[doc = "The memory block is 32-byte large, which permits to reach the maximum packet length defined by USB specifications. With this block size the allocated buffer size theoretically ranges from 32 to 1024 bytes, which is the longest packet size allowed by USB standard specifications. However, the applicable size is limited by the available buffer memory. The buffer size is (NUM_BLOCK+1)*32."]
    #[inline(always)]
    pub fn is_bytes_32(&self) -> bool {
        *self == Blsize::Bytes32
    }
}
#[doc = "Field `BLSIZE` writer - This bit selects the size of memory block used to define the allocated buffer area."]
pub type BlsizeW<'a, REG> = crate::BitWriter<'a, REG, Blsize>;
impl<'a, REG> BlsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The memory block is 2-byte large, which is the minimum block allowed in a half-word wide memory. With this block size the allocated buffer size ranges from 2 to 62 bytes. The buffer size is NUM_BLOCK*2; NUM_BLOCK must not be 0."]
    #[inline(always)]
    pub fn bytes_2(self) -> &'a mut crate::W<REG> {
        self.variant(Blsize::Bytes2)
    }
    #[doc = "The memory block is 32-byte large, which permits to reach the maximum packet length defined by USB specifications. With this block size the allocated buffer size theoretically ranges from 32 to 1024 bytes, which is the longest packet size allowed by USB standard specifications. However, the applicable size is limited by the available buffer memory. The buffer size is (NUM_BLOCK+1)*32."]
    #[inline(always)]
    pub fn bytes_32(self) -> &'a mut crate::W<REG> {
        self.variant(Blsize::Bytes32)
    }
}
impl R {
    #[doc = "Bits 0:15 - These bits point to the starting address of the packet buffer, which contains the data received by the endpoint/channel associated with the USB_CHEPnR register at the next OUT/SETUP token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
    #[inline(always)]
    pub fn addr_rx(&self) -> AddrRxR {
        AddrRxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - These bits contain the number of bytes received by the endpoint/channel associated with the USB_CHEPnR register during the last OUT/SETUP transaction addressed to it."]
    #[inline(always)]
    pub fn count_rx(&self) -> CountRxR {
        CountRxR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:30 - These bits define the number of memory blocks allocated to this packet buffer. The actual amount of allocated memory depends on the BLSIZE value."]
    #[inline(always)]
    pub fn num_block(&self) -> NumBlockR {
        NumBlockR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - This bit selects the size of memory block used to define the allocated buffer area."]
    #[inline(always)]
    pub fn blsize(&self) -> BlsizeR {
        BlsizeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits point to the starting address of the packet buffer, which contains the data received by the endpoint/channel associated with the USB_CHEPnR register at the next OUT/SETUP token addressed to it. Bits 1 and 0 must always be written as \"00\" since packet memory is word wide and all packet buffers must be word aligned."]
    #[inline(always)]
    #[must_use]
    pub fn addr_rx(&mut self) -> AddrRxW<ChepRxtxbd0Spec> {
        AddrRxW::new(self, 0)
    }
    #[doc = "Bits 16:25 - These bits contain the number of bytes received by the endpoint/channel associated with the USB_CHEPnR register during the last OUT/SETUP transaction addressed to it."]
    #[inline(always)]
    #[must_use]
    pub fn count_rx(&mut self) -> CountRxW<ChepRxtxbd0Spec> {
        CountRxW::new(self, 16)
    }
    #[doc = "Bits 26:30 - These bits define the number of memory blocks allocated to this packet buffer. The actual amount of allocated memory depends on the BLSIZE value."]
    #[inline(always)]
    #[must_use]
    pub fn num_block(&mut self) -> NumBlockW<ChepRxtxbd0Spec> {
        NumBlockW::new(self, 26)
    }
    #[doc = "Bit 31 - This bit selects the size of memory block used to define the allocated buffer area."]
    #[inline(always)]
    #[must_use]
    pub fn blsize(&mut self) -> BlsizeW<ChepRxtxbd0Spec> {
        BlsizeW::new(self, 31)
    }
}
#[doc = "Channel/endpoint receive buffer descriptor 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chep_rxtxbd_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep_rxtxbd_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChepRxtxbd0Spec;
impl crate::RegisterSpec for ChepRxtxbd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chep_rxtxbd_0::R`](R) reader structure"]
impl crate::Readable for ChepRxtxbd0Spec {}
#[doc = "`write(|w| ..)` method takes [`chep_rxtxbd_0::W`](W) writer structure"]
impl crate::Writable for ChepRxtxbd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHEP_RXTXBD_0 to value 0"]
impl crate::Resettable for ChepRxtxbd0Spec {
    const RESET_VALUE: u32 = 0;
}
