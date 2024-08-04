#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OtyperSpec>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OtyperSpec>;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot0;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot1;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot2;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot3;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot4;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot5;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot6;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot7;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot8;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot9;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot10;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot11;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot12;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot13;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Ot15 as Ot14;
#[doc = "Field `OT0` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot0R;
#[doc = "Field `OT1` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot1R;
#[doc = "Field `OT2` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot2R;
#[doc = "Field `OT3` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot3R;
#[doc = "Field `OT4` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot4R;
#[doc = "Field `OT5` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot5R;
#[doc = "Field `OT6` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot6R;
#[doc = "Field `OT7` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot7R;
#[doc = "Field `OT8` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot8R;
#[doc = "Field `OT9` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot9R;
#[doc = "Field `OT10` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot10R;
#[doc = "Field `OT11` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot11R;
#[doc = "Field `OT12` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot12R;
#[doc = "Field `OT13` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot13R;
#[doc = "Field `OT14` reader - Port x configuration bits (y = 0..15)"]
pub use Ot15R as Ot14R;
#[doc = "Field `OT0` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot0W;
#[doc = "Field `OT1` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot1W;
#[doc = "Field `OT2` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot2W;
#[doc = "Field `OT3` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot3W;
#[doc = "Field `OT4` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot4W;
#[doc = "Field `OT5` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot5W;
#[doc = "Field `OT6` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot6W;
#[doc = "Field `OT7` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot7W;
#[doc = "Field `OT8` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot8W;
#[doc = "Field `OT9` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot9W;
#[doc = "Field `OT10` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot10W;
#[doc = "Field `OT11` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot11W;
#[doc = "Field `OT12` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot12W;
#[doc = "Field `OT13` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot13W;
#[doc = "Field `OT14` writer - Port x configuration bits (y = 0..15)"]
pub use Ot15W as Ot14W;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot15 {
    #[doc = "0: Push-pull output"]
    PushPull = 0,
    #[doc = "1: Open-drain output"]
    OpenDrain = 1,
}
impl From<Ot15> for bool {
    #[inline(always)]
    fn from(variant: Ot15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT15` reader - Port x configuration bits (y = 0..15)"]
pub type Ot15R = crate::BitReader<Ot15>;
impl Ot15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot15 {
        match self.bits {
            false => Ot15::PushPull,
            true => Ot15::OpenDrain,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == Ot15::PushPull
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Ot15::OpenDrain
    }
}
#[doc = "Field `OT15` writer - Port x configuration bits (y = 0..15)"]
pub type Ot15W<'a, REG> = crate::BitWriter<'a, REG, Ot15>;
impl<'a, REG> Ot15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(Ot15::PushPull)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Ot15::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&self) -> Ot0R {
        Ot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&self) -> Ot1R {
        Ot1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&self) -> Ot2R {
        Ot2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> Ot3R {
        Ot3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&self) -> Ot4R {
        Ot4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot5(&self) -> Ot5R {
        Ot5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot6(&self) -> Ot6R {
        Ot6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot7(&self) -> Ot7R {
        Ot7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot8(&self) -> Ot8R {
        Ot8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot9(&self) -> Ot9R {
        Ot9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot10(&self) -> Ot10R {
        Ot10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot11(&self) -> Ot11R {
        Ot11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot12(&self) -> Ot12R {
        Ot12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot13(&self) -> Ot13R {
        Ot13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot14(&self) -> Ot14R {
        Ot14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot15(&self) -> Ot15R {
        Ot15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> Ot0W<OtyperSpec> {
        Ot0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> Ot1W<OtyperSpec> {
        Ot1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> Ot2W<OtyperSpec> {
        Ot2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> Ot3W<OtyperSpec> {
        Ot3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> Ot4W<OtyperSpec> {
        Ot4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot5(&mut self) -> Ot5W<OtyperSpec> {
        Ot5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot6(&mut self) -> Ot6W<OtyperSpec> {
        Ot6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot7(&mut self) -> Ot7W<OtyperSpec> {
        Ot7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot8(&mut self) -> Ot8W<OtyperSpec> {
        Ot8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot9(&mut self) -> Ot9W<OtyperSpec> {
        Ot9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot10(&mut self) -> Ot10W<OtyperSpec> {
        Ot10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot11(&mut self) -> Ot11W<OtyperSpec> {
        Ot11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot12(&mut self) -> Ot12W<OtyperSpec> {
        Ot12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot13(&mut self) -> Ot13W<OtyperSpec> {
        Ot13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot14(&mut self) -> Ot14W<OtyperSpec> {
        Ot14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot15(&mut self) -> Ot15W<OtyperSpec> {
        Ot15W::new(self, 15)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otyper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtyperSpec;
impl crate::RegisterSpec for OtyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OtyperSpec {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OtyperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OtyperSpec {
    const RESET_VALUE: u32 = 0;
}
