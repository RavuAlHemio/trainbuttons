#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PupdrSpec>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PupdrSpec>;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr0;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr1;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr2;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr3;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr4;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr5;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr6;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr7;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr8;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr9;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr10;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr11;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr12;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr13;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use Pupdr15 as Pupdr14;
#[doc = "Field `PUPDR0` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr0R;
#[doc = "Field `PUPDR1` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr1R;
#[doc = "Field `PUPDR2` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr2R;
#[doc = "Field `PUPDR3` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr3R;
#[doc = "Field `PUPDR4` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr4R;
#[doc = "Field `PUPDR5` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr5R;
#[doc = "Field `PUPDR6` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr6R;
#[doc = "Field `PUPDR7` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr7R;
#[doc = "Field `PUPDR8` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr8R;
#[doc = "Field `PUPDR9` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr9R;
#[doc = "Field `PUPDR10` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr10R;
#[doc = "Field `PUPDR11` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr11R;
#[doc = "Field `PUPDR12` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr12R;
#[doc = "Field `PUPDR13` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr13R;
#[doc = "Field `PUPDR14` reader - Port x configuration bits (y = 0..15)"]
pub use Pupdr15R as Pupdr14R;
#[doc = "Field `PUPDR0` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr0W;
#[doc = "Field `PUPDR1` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr1W;
#[doc = "Field `PUPDR2` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr2W;
#[doc = "Field `PUPDR3` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr3W;
#[doc = "Field `PUPDR4` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr4W;
#[doc = "Field `PUPDR5` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr5W;
#[doc = "Field `PUPDR6` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr6W;
#[doc = "Field `PUPDR7` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr7W;
#[doc = "Field `PUPDR8` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr8W;
#[doc = "Field `PUPDR9` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr9W;
#[doc = "Field `PUPDR10` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr10W;
#[doc = "Field `PUPDR11` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr11W;
#[doc = "Field `PUPDR12` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr12W;
#[doc = "Field `PUPDR13` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr13W;
#[doc = "Field `PUPDR14` writer - Port x configuration bits (y = 0..15)"]
pub use Pupdr15W as Pupdr14W;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupdr15 {
    #[doc = "0: No pull-up or pull-down"]
    No = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<Pupdr15> for u8 {
    #[inline(always)]
    fn from(variant: Pupdr15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupdr15 {
    type Ux = u8;
}
impl crate::IsEnum for Pupdr15 {}
#[doc = "Field `PUPDR15` reader - Port x configuration bits (y = 0..15)"]
pub type Pupdr15R = crate::FieldReader<Pupdr15>;
impl Pupdr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupdr15> {
        match self.bits {
            0 => Some(Pupdr15::No),
            1 => Some(Pupdr15::PullUp),
            2 => Some(Pupdr15::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up or pull-down"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Pupdr15::No
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Pupdr15::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Pupdr15::PullDown
    }
}
#[doc = "Field `PUPDR15` writer - Port x configuration bits (y = 0..15)"]
pub type Pupdr15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupdr15>;
impl<'a, REG> Pupdr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up or pull-down"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdr15::No)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdr15::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdr15::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr0(&self) -> Pupdr0R {
        Pupdr0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr1(&self) -> Pupdr1R {
        Pupdr1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr2(&self) -> Pupdr2R {
        Pupdr2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&self) -> Pupdr3R {
        Pupdr3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr4(&self) -> Pupdr4R {
        Pupdr4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr5(&self) -> Pupdr5R {
        Pupdr5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr6(&self) -> Pupdr6R {
        Pupdr6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr7(&self) -> Pupdr7R {
        Pupdr7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr8(&self) -> Pupdr8R {
        Pupdr8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr9(&self) -> Pupdr9R {
        Pupdr9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr10(&self) -> Pupdr10R {
        Pupdr10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr11(&self) -> Pupdr11R {
        Pupdr11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr12(&self) -> Pupdr12R {
        Pupdr12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr13(&self) -> Pupdr13R {
        Pupdr13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr14(&self) -> Pupdr14R {
        Pupdr14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr15(&self) -> Pupdr15R {
        Pupdr15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr0(&mut self) -> Pupdr0W<PupdrSpec> {
        Pupdr0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr1(&mut self) -> Pupdr1W<PupdrSpec> {
        Pupdr1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr2(&mut self) -> Pupdr2W<PupdrSpec> {
        Pupdr2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> Pupdr3W<PupdrSpec> {
        Pupdr3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr4(&mut self) -> Pupdr4W<PupdrSpec> {
        Pupdr4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr5(&mut self) -> Pupdr5W<PupdrSpec> {
        Pupdr5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr6(&mut self) -> Pupdr6W<PupdrSpec> {
        Pupdr6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr7(&mut self) -> Pupdr7W<PupdrSpec> {
        Pupdr7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr8(&mut self) -> Pupdr8W<PupdrSpec> {
        Pupdr8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr9(&mut self) -> Pupdr9W<PupdrSpec> {
        Pupdr9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr10(&mut self) -> Pupdr10W<PupdrSpec> {
        Pupdr10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr11(&mut self) -> Pupdr11W<PupdrSpec> {
        Pupdr11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr12(&mut self) -> Pupdr12W<PupdrSpec> {
        Pupdr12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr13(&mut self) -> Pupdr13W<PupdrSpec> {
        Pupdr13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr14(&mut self) -> Pupdr14W<PupdrSpec> {
        Pupdr14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr15(&mut self) -> Pupdr15W<PupdrSpec> {
        Pupdr15W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PupdrSpec;
impl crate::RegisterSpec for PupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PupdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0x2400_0000"]
impl crate::Resettable for PupdrSpec {
    const RESET_VALUE: u32 = 0x2400_0000;
}
