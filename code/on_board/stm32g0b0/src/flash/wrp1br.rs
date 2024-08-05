#[doc = "Register `WRP1BR` reader"]
pub type R = crate::R<Wrp1brSpec>;
#[doc = "Field `WRP1B_STRT` reader - WRP area B start offset"]
pub type Wrp1bStrtR = crate::FieldReader;
#[doc = "Field `WRP1B_END` reader - WRP area B end offset"]
pub type Wrp1bEndR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - WRP area B start offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> Wrp1bStrtR {
        Wrp1bStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area B end offset"]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> Wrp1bEndR {
        Wrp1bEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Flash WRP area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp1br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp1brSpec;
impl crate::RegisterSpec for Wrp1brSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1br::R`](R) reader structure"]
impl crate::Readable for Wrp1brSpec {}
#[doc = "`reset()` method sets WRP1BR to value 0xf000_0000"]
impl crate::Resettable for Wrp1brSpec {
    const RESET_VALUE: u32 = 0xf000_0000;
}
