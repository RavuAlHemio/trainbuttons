#[doc = "Register `WRP2AR` reader"]
pub type R = crate::R<Wrp2arSpec>;
#[doc = "Register `WRP2AR` writer"]
pub type W = crate::W<Wrp2arSpec>;
#[doc = "Field `WRP2A_STRT` reader - WRP2A_STRT"]
pub type Wrp2aStrtR = crate::FieldReader;
#[doc = "Field `WRP2A_STRT` writer - WRP2A_STRT"]
pub type Wrp2aStrtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2A_END` reader - WRP2A_END"]
pub type Wrp2aEndR = crate::FieldReader;
#[doc = "Field `WRP2A_END` writer - WRP2A_END"]
pub type Wrp2aEndW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP2A_STRT"]
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> Wrp2aStrtR {
        Wrp2aStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2A_END"]
    #[inline(always)]
    pub fn wrp2a_end(&self) -> Wrp2aEndR {
        Wrp2aEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2A_STRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_strt(&mut self) -> Wrp2aStrtW<Wrp2arSpec> {
        Wrp2aStrtW::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP2A_END"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_end(&mut self) -> Wrp2aEndW<Wrp2arSpec> {
        Wrp2aEndW::new(self, 16)
    }
}
#[doc = "FLASH WRP2 area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp2arSpec;
impl crate::RegisterSpec for Wrp2arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2ar::R`](R) reader structure"]
impl crate::Readable for Wrp2arSpec {}
#[doc = "`write(|w| ..)` method takes [`wrp2ar::W`](W) writer structure"]
impl crate::Writable for Wrp2arSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP2AR to value 0"]
impl crate::Resettable for Wrp2arSpec {
    const RESET_VALUE: u32 = 0;
}
