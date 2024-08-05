#[doc = "Register `ECCR` reader"]
pub type R = crate::R<EccrSpec>;
#[doc = "Register `ECCR` writer"]
pub type W = crate::W<EccrSpec>;
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type AddrEccR = crate::FieldReader<u16>;
#[doc = "Field `SYSF_ECC` reader - ECC fail for Corrected ECC Error or Double ECC Error in info block"]
pub type SysfEccR = crate::BitReader;
#[doc = "Field `ECCIE` reader - ECC correction interrupt enable"]
pub type EccieR = crate::BitReader;
#[doc = "Field `ECCIE` writer - ECC correction interrupt enable"]
pub type EccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - ECC correction"]
pub type EcccR = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction"]
pub type EcccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - ECC detection"]
pub type EccdR = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection"]
pub type EccdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> AddrEccR {
        AddrEccR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 20 - ECC fail for Corrected ECC Error or Double ECC Error in info block"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SysfEccR {
        SysfEccR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn eccie(&self) -> EccieR {
        EccieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> EcccR {
        EcccR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> EccdR {
        EccdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> EccieW<EccrSpec> {
        EccieW::new(self, 24)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> EcccW<EccrSpec> {
        EcccW::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> EccdW<EccrSpec> {
        EccdW::new(self, 31)
    }
}
#[doc = "Flash ECC register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccrSpec;
impl crate::RegisterSpec for EccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for EccrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccr::W`](W) writer structure"]
impl crate::Writable for EccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for EccrSpec {
    const RESET_VALUE: u32 = 0;
}
