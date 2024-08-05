#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel8;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel9;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel10;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel11;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel12;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel13;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel14;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7 as Afsel15;
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel8R;
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel9R;
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel10R;
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel11R;
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel12R;
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel13R;
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel14R;
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7R as Afsel15R;
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel8W;
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel9W;
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel10W;
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel11W;
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel12W;
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel13W;
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel14W;
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use super::afrl::Afsel7W as Afsel15W;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel8(&self) -> Afsel8R {
        Afsel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel9(&self) -> Afsel9R {
        Afsel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel10(&self) -> Afsel10R {
        Afsel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel11(&self) -> Afsel11R {
        Afsel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel12(&self) -> Afsel12R {
        Afsel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel13(&self) -> Afsel13R {
        Afsel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel14(&self) -> Afsel14R {
        Afsel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel15(&self) -> Afsel15R {
        Afsel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> Afsel8W<AfrhSpec> {
        Afsel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> Afsel9W<AfrhSpec> {
        Afsel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> Afsel10W<AfrhSpec> {
        Afsel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> Afsel11W<AfrhSpec> {
        Afsel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> Afsel12W<AfrhSpec> {
        Afsel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> Afsel13W<AfrhSpec> {
        Afsel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> Afsel14W<AfrhSpec> {
        Afsel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> Afsel15W<AfrhSpec> {
        Afsel15W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AfrhSpec {
    const RESET_VALUE: u32 = 0;
}
