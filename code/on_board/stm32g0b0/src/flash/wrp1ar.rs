#[doc = "Register `WRP1AR` reader"]
pub type R = crate::R<Wrp1arSpec>;
#[doc = "Field `WRP1A_STRT` reader - WRP area A start offset"]
pub type Wrp1aStrtR = crate::FieldReader;
#[doc = "Field `WRP1A_END` reader - WRP area A end offset"]
pub type Wrp1aEndR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - WRP area A start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> Wrp1aStrtR {
        Wrp1aStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> Wrp1aEndR {
        Wrp1aEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Flash WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1ar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp1arSpec;
impl crate::RegisterSpec for Wrp1arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1ar::R`](R) reader structure"]
impl crate::Readable for Wrp1arSpec {}
#[doc = "`reset()` method sets WRP1AR to value 0xf000_0000"]
impl crate::Resettable for Wrp1arSpec {
    const RESET_VALUE: u32 = 0xf000_0000;
}
