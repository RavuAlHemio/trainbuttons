#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Operation error"]
pub type OperrR = crate::BitReader;
#[doc = "Field `OPERR` writer - Operation error"]
pub type OperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` reader - Programming error"]
pub type ProgerrR = crate::BitReader;
#[doc = "Field `PROGERR` writer - Programming error"]
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WrperrR = crate::BitReader;
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WrperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PgaerrR = crate::BitReader;
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PgaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZERR` reader - Size error"]
pub type SizerrR = crate::BitReader;
#[doc = "Field `SIZERR` writer - Size error"]
pub type SizerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PgserrR = crate::BitReader;
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PgserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISERR` reader - Fast programming data miss error"]
pub type MiserrR = crate::BitReader;
#[doc = "Field `MISERR` writer - Fast programming data miss error"]
pub type MiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FasterrR = crate::BitReader;
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub type FasterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTVERR` reader - Option and Engineering bits loading validity error"]
pub type OptverrR = crate::BitReader;
#[doc = "Field `OPTVERR` writer - Option and Engineering bits loading validity error"]
pub type OptverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY1` reader - BSY1"]
pub type Bsy1R = crate::BitReader;
#[doc = "Field `BSY1` writer - BSY1"]
pub type Bsy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY2` reader - BSY2"]
pub type Bsy2R = crate::BitReader;
#[doc = "Field `BSY2` writer - BSY2"]
pub type Bsy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGBSY` reader - Programming or erase configuration busy."]
pub type CfgbsyR = crate::BitReader;
#[doc = "Field `CFGBSY` writer - Programming or erase configuration busy."]
pub type CfgbsyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WrperrR {
        WrperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PgaerrR {
        PgaerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SizerrR {
        SizerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PgserrR {
        PgserrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&self) -> MiserrR {
        MiserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FasterrR {
        FasterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Option and Engineering bits loading validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OptverrR {
        OptverrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BSY1"]
    #[inline(always)]
    pub fn bsy1(&self) -> Bsy1R {
        Bsy1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BSY2"]
    #[inline(always)]
    pub fn bsy2(&self) -> Bsy2R {
        Bsy2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Programming or erase configuration busy."]
    #[inline(always)]
    pub fn cfgbsy(&self) -> CfgbsyR {
        CfgbsyR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EopW<SrSpec> {
        EopW::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OperrW<SrSpec> {
        OperrW::new(self, 1)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> ProgerrW<SrSpec> {
        ProgerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WrperrW<SrSpec> {
        WrperrW::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PgaerrW<SrSpec> {
        PgaerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SizerrW<SrSpec> {
        SizerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PgserrW<SrSpec> {
        PgserrW::new(self, 7)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    #[must_use]
    pub fn miserr(&mut self) -> MiserrW<SrSpec> {
        MiserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FasterrW<SrSpec> {
        FasterrW::new(self, 9)
    }
    #[doc = "Bit 15 - Option and Engineering bits loading validity error"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OptverrW<SrSpec> {
        OptverrW::new(self, 15)
    }
    #[doc = "Bit 16 - BSY1"]
    #[inline(always)]
    #[must_use]
    pub fn bsy1(&mut self) -> Bsy1W<SrSpec> {
        Bsy1W::new(self, 16)
    }
    #[doc = "Bit 17 - BSY2"]
    #[inline(always)]
    #[must_use]
    pub fn bsy2(&mut self) -> Bsy2W<SrSpec> {
        Bsy2W::new(self, 17)
    }
    #[doc = "Bit 18 - Programming or erase configuration busy."]
    #[inline(always)]
    #[must_use]
    pub fn cfgbsy(&mut self) -> CfgbsyW<SrSpec> {
        CfgbsyW::new(self, 18)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
