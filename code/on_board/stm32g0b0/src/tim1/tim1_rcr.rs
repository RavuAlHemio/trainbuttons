#[doc = "Register `TIM1_RCR` reader"]
pub type R = crate::R<Tim1RcrSpec>;
#[doc = "Register `TIM1_RCR` writer"]
pub type W = crate::W<Tim1RcrSpec>;
#[doc = "Field `REP` reader - Repetition counter value"]
pub type RepR = crate::FieldReader<u16>;
#[doc = "Field `REP` writer - Repetition counter value"]
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> RepW<Tim1RcrSpec> {
        RepW::new(self, 0)
    }
}
#[doc = "repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1RcrSpec;
impl crate::RegisterSpec for Tim1RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_rcr::R`](R) reader structure"]
impl crate::Readable for Tim1RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_rcr::W`](W) writer structure"]
impl crate::Writable for Tim1RcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_RCR to value 0"]
impl crate::Resettable for Tim1RcrSpec {
    const RESET_VALUE: u32 = 0;
}
