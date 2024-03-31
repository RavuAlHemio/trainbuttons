#[doc = "Register `AF1` reader"]
pub type R = crate::R<Af1Spec>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<Af1Spec>;
#[doc = "ETR source selection These bits select the ETR input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etrsel {
    #[doc = "0: ETR legacy mode"]
    B0x0 = 0,
    #[doc = "1: COMP1"]
    B0x1 = 1,
    #[doc = "2: COMP2"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Etrsel> for u8 {
    #[inline(always)]
    fn from(variant: Etrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etrsel {
    type Ux = u8;
}
impl crate::IsEnum for Etrsel {}
#[doc = "Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved"]
pub type EtrselR = crate::FieldReader<Etrsel>;
impl EtrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Etrsel> {
        match self.bits {
            0 => Some(Etrsel::B0x0),
            1 => Some(Etrsel::B0x1),
            2 => Some(Etrsel::B0x2),
            3 => Some(Etrsel::B0x3),
            _ => None,
        }
    }
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Etrsel::B0x0
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Etrsel::B0x1
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Etrsel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Etrsel::B0x3
    }
}
#[doc = "Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved"]
pub type EtrselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Etrsel>;
impl<'a, REG> EtrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x0)
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x1)
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Etrsel::B0x3)
    }
}
impl R {
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved"]
    #[inline(always)]
    pub fn etrsel(&self) -> EtrselR {
        EtrselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> EtrselW<Af1Spec> {
        EtrselW::new(self, 14)
    }
}
#[doc = "TIM alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af1Spec;
impl crate::RegisterSpec for Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for Af1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for Af1Spec {
    const RESET_VALUE: u32 = 0;
}
