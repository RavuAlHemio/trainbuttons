#[doc = "Register `ITLINE28` reader"]
pub type R = crate::R<Itline28Spec>;
#[doc = "Field `USART2` reader - USART2"]
pub type Usart2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART2"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline28Spec;
impl crate::RegisterSpec for Itline28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline28::R`](R) reader structure"]
impl crate::Readable for Itline28Spec {}
#[doc = "`reset()` method sets ITLINE28 to value 0"]
impl crate::Resettable for Itline28Spec {
    const RESET_VALUE: u32 = 0;
}
