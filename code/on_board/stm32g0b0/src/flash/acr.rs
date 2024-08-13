#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Latency {
    #[doc = "0: Zero wait states, for clock speeds ≤ 24 MHz in Vcore range 1 and ≤ 8 MHz in Vcore range 2."]
    Ws0 = 0,
    #[doc = "1: One wait state, for clock speeds ≤ 48 MHz in Vcore range 1 and ≤ 16 MHz in Vcore range 2."]
    Ws1 = 1,
    #[doc = "2: Two wait states, for clock speeds ≤ 64 MHz in Vcore range 1."]
    Ws2 = 2,
}
impl From<Latency> for u8 {
    #[inline(always)]
    fn from(variant: Latency) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Latency {
    type Ux = u8;
}
impl crate::IsEnum for Latency {}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LatencyR = crate::FieldReader<Latency>;
impl LatencyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Latency> {
        match self.bits {
            0 => Some(Latency::Ws0),
            1 => Some(Latency::Ws1),
            2 => Some(Latency::Ws2),
            _ => None,
        }
    }
    #[doc = "Zero wait states, for clock speeds ≤ 24 MHz in Vcore range 1 and ≤ 8 MHz in Vcore range 2."]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == Latency::Ws0
    }
    #[doc = "One wait state, for clock speeds ≤ 48 MHz in Vcore range 1 and ≤ 16 MHz in Vcore range 2."]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == Latency::Ws1
    }
    #[doc = "Two wait states, for clock speeds ≤ 64 MHz in Vcore range 1."]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == Latency::Ws2
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Latency>;
impl<'a, REG> LatencyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait states, for clock speeds ≤ 24 MHz in Vcore range 1 and ≤ 8 MHz in Vcore range 2."]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::Ws0)
    }
    #[doc = "One wait state, for clock speeds ≤ 48 MHz in Vcore range 1 and ≤ 16 MHz in Vcore range 2."]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::Ws1)
    }
    #[doc = "Two wait states, for clock speeds ≤ 64 MHz in Vcore range 1."]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::Ws2)
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PrftenR = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PrftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type IcenR = crate::BitReader;
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type IcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type IcrstR = crate::BitReader;
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type IcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - Flash User area empty"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - Flash User area empty"]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PrftenR {
        PrftenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> IcrstR {
        IcrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LatencyW<AcrSpec> {
        LatencyW::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PrftenW<AcrSpec> {
        PrftenW::new(self, 8)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> IcenW<AcrSpec> {
        IcenW::new(self, 9)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> IcrstW<AcrSpec> {
        IcrstW::new(self, 11)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<AcrSpec> {
        EmptyW::new(self, 16)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0600"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0x0600;
}
