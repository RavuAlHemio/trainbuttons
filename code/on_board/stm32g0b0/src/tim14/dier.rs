#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uie {
    #[doc = "0: Update interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Update interrupt enabled"]
    B0x1 = 1,
}
impl From<Uie> for bool {
    #[inline(always)]
    fn from(variant: Uie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UieR = crate::BitReader<Uie>;
impl UieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uie {
        match self.bits {
            false => Uie::B0x0,
            true => Uie::B0x1,
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uie::B0x0
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uie::B0x1
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG, Uie>;
impl<'a, REG> UieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uie::B0x0)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uie::B0x1)
    }
}
#[doc = "Capture/Compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1ie {
    #[doc = "0: CC1 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CC1 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc1ie> for bool {
    #[inline(always)]
    fn from(variant: Cc1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"]
pub type Cc1ieR = crate::BitReader<Cc1ie>;
impl Cc1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1ie {
        match self.bits {
            false => Cc1ie::B0x0,
            true => Cc1ie::B0x1,
        }
    }
    #[doc = "CC1 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1ie::B0x0
    }
    #[doc = "CC1 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1ie::B0x1
    }
}
#[doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG, Cc1ie>;
impl<'a, REG> Cc1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ie::B0x0)
    }
    #[doc = "CC1 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UieW<DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> Cc1ieW<DierSpec> {
        Cc1ieW::new(self, 1)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {
    const RESET_VALUE: u32 = 0;
}
