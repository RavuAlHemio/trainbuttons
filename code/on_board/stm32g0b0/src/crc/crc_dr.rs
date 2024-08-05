#[doc = "Register `CRC_DR` reader"]
pub type R = crate::R<CrcDrSpec>;
#[doc = "Register `CRC_DR` writer"]
pub type W = crate::W<CrcDrSpec>;
#[doc = "Field `DR` reader - Data register bits"]
pub type DrR = crate::FieldReader<u32>;
#[doc = "Field `DR` writer - Data register bits"]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data register bits"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<CrcDrSpec> {
        DrW::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcDrSpec;
impl crate::RegisterSpec for CrcDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_dr::R`](R) reader structure"]
impl crate::Readable for CrcDrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_dr::W`](W) writer structure"]
impl crate::Writable for CrcDrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_DR to value 0xffff_ffff"]
impl crate::Resettable for CrcDrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
