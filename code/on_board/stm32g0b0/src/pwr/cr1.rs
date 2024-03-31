#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LPMS` reader - Low-power mode selection"]
pub type LpmsR = crate::FieldReader;
#[doc = "Field `LPMS` writer - Low-power mode selection"]
pub type LpmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FPD_STOP` reader - Flash memory powered down during Stop mode"]
pub type FpdStopR = crate::BitReader;
#[doc = "Field `FPD_STOP` writer - Flash memory powered down during Stop mode"]
pub type FpdStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPD_LPRUN` reader - Flash memory powered down during Low-power run mode"]
pub type FpdLprunR = crate::BitReader;
#[doc = "Field `FPD_LPRUN` writer - Flash memory powered down during Low-power run mode"]
pub type FpdLprunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPD_LPSLP` reader - Flash memory powered down during Low-power sleep mode"]
pub type FpdLpslpR = crate::BitReader;
#[doc = "Field `FPD_LPSLP` writer - Flash memory powered down during Low-power sleep mode"]
pub type FpdLpslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DbpR = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VosR = crate::FieldReader;
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPR` reader - Low-power run"]
pub type LprR = crate::BitReader;
#[doc = "Field `LPR` writer - Low-power run"]
pub type LprW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    pub fn lpms(&self) -> LpmsR {
        LpmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Flash memory powered down during Stop mode"]
    #[inline(always)]
    pub fn fpd_stop(&self) -> FpdStopR {
        FpdStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash memory powered down during Low-power run mode"]
    #[inline(always)]
    pub fn fpd_lprun(&self) -> FpdLprunR {
        FpdLprunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash memory powered down during Low-power sleep mode"]
    #[inline(always)]
    pub fn fpd_lpslp(&self) -> FpdLpslpR {
        FpdLpslpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DbpR {
        DbpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VosR {
        VosR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LprR {
        LprR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LpmsW<Cr1Spec> {
        LpmsW::new(self, 0)
    }
    #[doc = "Bit 3 - Flash memory powered down during Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_stop(&mut self) -> FpdStopW<Cr1Spec> {
        FpdStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Flash memory powered down during Low-power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_lprun(&mut self) -> FpdLprunW<Cr1Spec> {
        FpdLprunW::new(self, 4)
    }
    #[doc = "Bit 5 - Flash memory powered down during Low-power sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_lpslp(&mut self) -> FpdLpslpW<Cr1Spec> {
        FpdLpslpW::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DbpW<Cr1Spec> {
        DbpW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VosW<Cr1Spec> {
        VosW::new(self, 9)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LprW<Cr1Spec> {
        LprW::new(self, 14)
    }
}
#[doc = "Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x0208"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0x0208;
}
