#[doc = "Register `TAMP_MISR` reader"]
pub type R = crate::R<TampMisrSpec>;
#[doc = "Field `TAMP1MF` reader - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised."]
pub type Tamp1mfR = crate::BitReader;
#[doc = "Field `TAMP2MF` reader - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised."]
pub type Tamp2mfR = crate::BitReader;
#[doc = "Field `TAMP3MF` reader - TAMP3 interrupt masked flag This flag is set by hardware when the tamper 3 interrupt is raised."]
pub type Tamp3mfR = crate::BitReader;
#[doc = "Field `ITAMP3MF` reader - LSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised."]
pub type Itamp3mfR = crate::BitReader;
#[doc = "Field `ITAMP4MF` reader - HSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised."]
pub type Itamp4mfR = crate::BitReader;
#[doc = "Field `ITAMP5MF` reader - RTC calendar overflow tamper interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised."]
pub type Itamp5mfR = crate::BitReader;
#[doc = "Field `ITAMP6MF` reader - ST manufacturer readout tamper interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised."]
pub type Itamp6mfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised."]
    #[inline(always)]
    pub fn tamp1mf(&self) -> Tamp1mfR {
        Tamp1mfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised."]
    #[inline(always)]
    pub fn tamp2mf(&self) -> Tamp2mfR {
        Tamp2mfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3 interrupt masked flag This flag is set by hardware when the tamper 3 interrupt is raised."]
    #[inline(always)]
    pub fn tamp3mf(&self) -> Tamp3mfR {
        Tamp3mfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - LSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised."]
    #[inline(always)]
    pub fn itamp3mf(&self) -> Itamp3mfR {
        Itamp3mfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised."]
    #[inline(always)]
    pub fn itamp4mf(&self) -> Itamp4mfR {
        Itamp4mfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RTC calendar overflow tamper interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised."]
    #[inline(always)]
    pub fn itamp5mf(&self) -> Itamp5mfR {
        Itamp5mfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ST manufacturer readout tamper interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised."]
    #[inline(always)]
    pub fn itamp6mf(&self) -> Itamp6mfR {
        Itamp6mfR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "TAMP masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampMisrSpec;
impl crate::RegisterSpec for TampMisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_misr::R`](R) reader structure"]
impl crate::Readable for TampMisrSpec {}
#[doc = "`reset()` method sets TAMP_MISR to value 0"]
impl crate::Resettable for TampMisrSpec {
    const RESET_VALUE: u32 = 0;
}
