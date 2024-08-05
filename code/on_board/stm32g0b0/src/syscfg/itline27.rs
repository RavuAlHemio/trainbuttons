#[doc = "Register `ITLINE27` reader"]
pub type R = crate::R<Itline27Spec>;
#[doc = "Field `USART1` reader - USART1"]
pub type Usart1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART1"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 27 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`itline27::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline27Spec;
impl crate::RegisterSpec for Itline27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline27::R`](R) reader structure"]
impl crate::Readable for Itline27Spec {}
#[doc = "`reset()` method sets ITLINE27 to value 0"]
impl crate::Resettable for Itline27Spec {
    const RESET_VALUE: u32 = 0;
}
