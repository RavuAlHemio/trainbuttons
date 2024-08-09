#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DaddrSpec>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DaddrSpec>;
#[doc = "Field `ADD` reader - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Field `EF` reader - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers."]
pub type EfR = crate::BitReader;
#[doc = "Field `EF` writer - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers."]
pub type EfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers."]
    #[inline(always)]
    pub fn ef(&self) -> EfR {
        EfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<DaddrSpec> {
        AddW::new(self, 0)
    }
    #[doc = "Bit 7 - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers."]
    #[inline(always)]
    #[must_use]
    pub fn ef(&mut self) -> EfW<DaddrSpec> {
        EfW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaddrSpec;
impl crate::RegisterSpec for DaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DaddrSpec {
    const RESET_VALUE: u32 = 0;
}
