#[doc = "Register `OR1` reader"]
pub type R = crate::R<Or1Spec>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<Or1Spec>;
#[doc = "Ocref_clr source selection This bit selects the ocref_clr input source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OcrefClr {
    #[doc = "0: COMP1 output is connected to the OCREF_CLR input"]
    B0x0 = 0,
    #[doc = "1: COMP2 output is connected to the OCREF_CLR input"]
    B0x1 = 1,
}
impl From<OcrefClr> for bool {
    #[inline(always)]
    fn from(variant: OcrefClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCREF_CLR` reader - Ocref_clr source selection This bit selects the ocref_clr input source."]
pub type OcrefClrR = crate::BitReader<OcrefClr>;
impl OcrefClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OcrefClr {
        match self.bits {
            false => OcrefClr::B0x0,
            true => OcrefClr::B0x1,
        }
    }
    #[doc = "COMP1 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OcrefClr::B0x0
    }
    #[doc = "COMP2 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OcrefClr::B0x1
    }
}
#[doc = "Field `OCREF_CLR` writer - Ocref_clr source selection This bit selects the ocref_clr input source."]
pub type OcrefClrW<'a, REG> = crate::BitWriter<'a, REG, OcrefClr>;
impl<'a, REG> OcrefClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OcrefClr::B0x0)
    }
    #[doc = "COMP2 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OcrefClr::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    pub fn ocref_clr(&self) -> OcrefClrR {
        OcrefClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    #[must_use]
    pub fn ocref_clr(&mut self) -> OcrefClrW<Or1Spec> {
        OcrefClrW::new(self, 0)
    }
}
#[doc = "TIM option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or1Spec;
impl crate::RegisterSpec for Or1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for Or1Spec {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for Or1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for Or1Spec {
    const RESET_VALUE: u32 = 0;
}
