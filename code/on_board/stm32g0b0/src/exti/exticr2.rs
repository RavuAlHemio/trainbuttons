#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<Exticr2Spec>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<Exticr2Spec>;
#[doc = "Field `EXTI0_7` reader - GPIO port selection"]
pub type Exti0_7R = crate::FieldReader;
#[doc = "Field `EXTI0_7` writer - GPIO port selection"]
pub type Exti0_7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI8_15` reader - GPIO port selection"]
pub type Exti8_15R = crate::FieldReader;
#[doc = "Field `EXTI8_15` writer - GPIO port selection"]
pub type Exti8_15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI16_23` reader - GPIO port selection"]
pub type Exti16_23R = crate::FieldReader;
#[doc = "Field `EXTI16_23` writer - GPIO port selection"]
pub type Exti16_23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI24_31` reader - GPIO port selection"]
pub type Exti24_31R = crate::FieldReader;
#[doc = "Field `EXTI24_31` writer - GPIO port selection"]
pub type Exti24_31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> Exti0_7R {
        Exti0_7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> Exti8_15R {
        Exti8_15R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> Exti16_23R {
        Exti16_23R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> Exti24_31R {
        Exti24_31R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> Exti0_7W<Exticr2Spec> {
        Exti0_7W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> Exti8_15W<Exticr2Spec> {
        Exti8_15W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> Exti16_23W<Exticr2Spec> {
        Exti16_23W::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> Exti24_31W<Exticr2Spec> {
        Exti24_31W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exticr2Spec;
impl crate::RegisterSpec for Exticr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for Exticr2Spec {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for Exticr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for Exticr2Spec {
    const RESET_VALUE: u32 = 0;
}
