#[doc = "Register `ITLINE29` reader"]
pub type R = crate::R<Itline29Spec>;
#[doc = "Field `USART3` reader - USART3"]
pub type Usart3R = crate::BitReader;
#[doc = "Field `USART4` reader - USART4"]
pub type Usart4R = crate::BitReader;
#[doc = "Field `USART5` reader - USART5"]
pub type Usart5R = crate::BitReader;
#[doc = "Field `USART6` reader - USART6"]
pub type Usart6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART4"]
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> Usart5R {
        Usart5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART6"]
    #[inline(always)]
    pub fn usart6(&self) -> Usart6R {
        Usart6R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline29::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itline29Spec;
impl crate::RegisterSpec for Itline29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline29::R`](R) reader structure"]
impl crate::Readable for Itline29Spec {}
#[doc = "`reset()` method sets ITLINE29 to value 0"]
impl crate::Resettable for Itline29Spec {
    const RESET_VALUE: u32 = 0;
}
