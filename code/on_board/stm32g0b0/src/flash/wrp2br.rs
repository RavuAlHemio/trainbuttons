#[doc = "Register `WRP2BR` reader"]
pub type R = crate::R<Wrp2brSpec>;
#[doc = "Register `WRP2BR` writer"]
pub type W = crate::W<Wrp2brSpec>;
#[doc = "Field `WRP2B_STRT` reader - WRP2B_STRT"]
pub type Wrp2bStrtR = crate::FieldReader;
#[doc = "Field `WRP2B_STRT` writer - WRP2B_STRT"]
pub type Wrp2bStrtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2B_END` reader - WRP2B_END"]
pub type Wrp2bEndR = crate::FieldReader;
#[doc = "Field `WRP2B_END` writer - WRP2B_END"]
pub type Wrp2bEndW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP2B_STRT"]
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> Wrp2bStrtR {
        Wrp2bStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2B_END"]
    #[inline(always)]
    pub fn wrp2b_end(&self) -> Wrp2bEndR {
        Wrp2bEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2B_STRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_strt(&mut self) -> Wrp2bStrtW<Wrp2brSpec> {
        Wrp2bStrtW::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP2B_END"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_end(&mut self) -> Wrp2bEndW<Wrp2brSpec> {
        Wrp2bEndW::new(self, 16)
    }
}
#[doc = "FLASH WRP2 area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp2brSpec;
impl crate::RegisterSpec for Wrp2brSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2br::R`](R) reader structure"]
impl crate::Readable for Wrp2brSpec {}
#[doc = "`write(|w| ..)` method takes [`wrp2br::W`](W) writer structure"]
impl crate::Writable for Wrp2brSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP2BR to value 0"]
impl crate::Resettable for Wrp2brSpec {
    const RESET_VALUE: u32 = 0;
}
