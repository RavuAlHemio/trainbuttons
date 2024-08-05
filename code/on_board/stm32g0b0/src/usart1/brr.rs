#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BRR` reader - USART baud rate"]
pub type BrrR = crate::FieldReader<u16>;
#[doc = "Field `BRR` writer - USART baud rate"]
pub type BrrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - USART baud rate"]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USART baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BrrW<BrrSpec> {
        BrrW::new(self, 0)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {
    const RESET_VALUE: u32 = 0;
}
