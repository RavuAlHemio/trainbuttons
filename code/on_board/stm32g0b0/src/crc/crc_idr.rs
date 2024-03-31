#[doc = "Register `CRC_IDR` reader"]
pub type R = crate::R<CrcIdrSpec>;
#[doc = "Register `CRC_IDR` writer"]
pub type W = crate::W<CrcIdrSpec>;
#[doc = "Field `IDR` reader - General-purpose 32-bit data register bits"]
pub type IdrR = crate::FieldReader<u32>;
#[doc = "Field `IDR` writer - General-purpose 32-bit data register bits"]
pub type IdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General-purpose 32-bit data register bits"]
    #[inline(always)]
    pub fn idr(&self) -> IdrR {
        IdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General-purpose 32-bit data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn idr(&mut self) -> IdrW<CrcIdrSpec> {
        IdrW::new(self, 0)
    }
}
#[doc = "Independent data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_idr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_idr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcIdrSpec;
impl crate::RegisterSpec for CrcIdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_idr::R`](R) reader structure"]
impl crate::Readable for CrcIdrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_idr::W`](W) writer structure"]
impl crate::Writable for CrcIdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_IDR to value 0"]
impl crate::Resettable for CrcIdrSpec {
    const RESET_VALUE: u32 = 0;
}
