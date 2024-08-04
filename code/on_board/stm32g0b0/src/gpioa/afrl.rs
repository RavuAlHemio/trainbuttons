#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel0;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel1;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel2;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel3;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel4;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel5;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7 as Afsel6;
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel0R;
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel1R;
#[doc = "Field `AFSEL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel2R;
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel3R;
#[doc = "Field `AFSEL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel4R;
#[doc = "Field `AFSEL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel5R;
#[doc = "Field `AFSEL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7R as Afsel6R;
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel0W;
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel1W;
#[doc = "Field `AFSEL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel2W;
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel3W;
#[doc = "Field `AFSEL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel4W;
#[doc = "Field `AFSEL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel5W;
#[doc = "Field `AFSEL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use Afsel7W as Afsel6W;
#[doc = "Alternate function selection for port x bit y (y = 0..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel7 {
    #[doc = "0: Alternate function 0"]
    Af0 = 0,
    #[doc = "1: Alternate function 1"]
    Af1 = 1,
    #[doc = "2: Alternate function 2"]
    Af2 = 2,
    #[doc = "3: Alternate function 3"]
    Af3 = 3,
    #[doc = "4: Alternate function 4"]
    Af4 = 4,
    #[doc = "5: Alternate function 5"]
    Af5 = 5,
    #[doc = "6: Alternate function 6"]
    Af6 = 6,
    #[doc = "7: Alternate function 7"]
    Af7 = 7,
}
impl From<Afsel7> for u8 {
    #[inline(always)]
    fn from(variant: Afsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel7 {}
#[doc = "Field `AFSEL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel7R = crate::FieldReader<Afsel7>;
impl Afsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Afsel7> {
        match self.bits {
            0 => Some(Afsel7::Af0),
            1 => Some(Afsel7::Af1),
            2 => Some(Afsel7::Af2),
            3 => Some(Afsel7::Af3),
            4 => Some(Afsel7::Af4),
            5 => Some(Afsel7::Af5),
            6 => Some(Afsel7::Af6),
            7 => Some(Afsel7::Af7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == Afsel7::Af0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Afsel7::Af1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Afsel7::Af2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Afsel7::Af3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Afsel7::Af4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Afsel7::Af5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == Afsel7::Af6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == Afsel7::Af7
    }
}
#[doc = "Field `AFSEL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afsel7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel7>;
impl<'a, REG> Afsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::Af7)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel0(&self) -> Afsel0R {
        Afsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel1(&self) -> Afsel1R {
        Afsel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel2(&self) -> Afsel2R {
        Afsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel3(&self) -> Afsel3R {
        Afsel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel4(&self) -> Afsel4R {
        Afsel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel5(&self) -> Afsel5R {
        Afsel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel6(&self) -> Afsel6R {
        Afsel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel7(&self) -> Afsel7R {
        Afsel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> Afsel0W<AfrlSpec> {
        Afsel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> Afsel1W<AfrlSpec> {
        Afsel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> Afsel2W<AfrlSpec> {
        Afsel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> Afsel3W<AfrlSpec> {
        Afsel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> Afsel4W<AfrlSpec> {
        Afsel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> Afsel5W<AfrlSpec> {
        Afsel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> Afsel6W<AfrlSpec> {
        Afsel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> Afsel7W<AfrlSpec> {
        Afsel7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {
    const RESET_VALUE: u32 = 0;
}
