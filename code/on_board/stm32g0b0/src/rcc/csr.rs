#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `LSION` reader - LSI oscillator enable"]
pub type LsionR = crate::BitReader;
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `RMVF` reader - Remove reset flags"]
pub type RmvfR = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flags"]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OblrstfR = crate::BitReader;
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `PWRRSTF` reader - BOR or POR/PDR flag"]
pub type PwrrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub type IwdgrstfR = crate::BitReader;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WwdgrstfR = crate::BitReader;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LpwrrstfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flags"]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OblrstfR {
        OblrstfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR or POR/PDR flag"]
    #[inline(always)]
    pub fn pwrrstf(&self) -> PwrrstfR {
        PwrrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IwdgrstfR {
        IwdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WwdgrstfR {
        WwdgrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LpwrrstfR {
        LpwrrstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LsionW<CsrSpec> {
        LsionW::new(self, 0)
    }
    #[doc = "Bit 23 - Remove reset flags"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RmvfW<CsrSpec> {
        RmvfW::new(self, 23)
    }
}
#[doc = "Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
