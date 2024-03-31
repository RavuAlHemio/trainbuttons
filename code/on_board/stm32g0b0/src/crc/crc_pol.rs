#[doc = "Register `CRC_POL` reader"]
pub type R = crate::R<CrcPolSpec>;
#[doc = "Register `CRC_POL` writer"]
pub type W = crate::W<CrcPolSpec>;
#[doc = "Field `POL` reader - Programmable polynomial"]
pub type PolR = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - Programmable polynomial"]
pub type PolW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<CrcPolSpec> {
        PolW::new(self, 0)
    }
}
#[doc = "polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcPolSpec;
impl crate::RegisterSpec for CrcPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pol::R`](R) reader structure"]
impl crate::Readable for CrcPolSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_pol::W`](W) writer structure"]
impl crate::Writable for CrcPolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_POL to value 0x04c1_1db7"]
impl crate::Resettable for CrcPolSpec {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
