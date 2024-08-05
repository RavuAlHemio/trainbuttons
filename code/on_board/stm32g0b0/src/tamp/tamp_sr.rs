#[doc = "Register `TAMP_SR` reader"]
pub type R = crate::R<TampSrSpec>;
#[doc = "Field `TAMP1F` reader - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input."]
pub type Tamp1fR = crate::BitReader;
#[doc = "Field `TAMP2F` reader - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input."]
pub type Tamp2fR = crate::BitReader;
#[doc = "Field `TAMP3F` reader - TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP3 input."]
pub type Tamp3fR = crate::BitReader;
#[doc = "Field `ITAMP3F` reader - LSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
pub type Itamp3fR = crate::BitReader;
#[doc = "Field `ITAMP4F` reader - HSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
pub type Itamp4fR = crate::BitReader;
#[doc = "Field `ITAMP5F` reader - RTC calendar overflow tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
pub type Itamp5fR = crate::BitReader;
#[doc = "Field `ITAMP6F` reader - ST manufacturer readout tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
pub type Itamp6fR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input."]
    #[inline(always)]
    pub fn tamp1f(&self) -> Tamp1fR {
        Tamp1fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input."]
    #[inline(always)]
    pub fn tamp2f(&self) -> Tamp2fR {
        Tamp2fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP3 input."]
    #[inline(always)]
    pub fn tamp3f(&self) -> Tamp3fR {
        Tamp3fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - LSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
    #[inline(always)]
    pub fn itamp3f(&self) -> Itamp3fR {
        Itamp3fR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
    #[inline(always)]
    pub fn itamp4f(&self) -> Itamp4fR {
        Itamp4fR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RTC calendar overflow tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
    #[inline(always)]
    pub fn itamp5f(&self) -> Itamp5fR {
        Itamp5fR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ST manufacturer readout tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
    #[inline(always)]
    pub fn itamp6f(&self) -> Itamp6fR {
        Itamp6fR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "TAMP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tamp_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampSrSpec;
impl crate::RegisterSpec for TampSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_sr::R`](R) reader structure"]
impl crate::Readable for TampSrSpec {}
#[doc = "`reset()` method sets TAMP_SR to value 0"]
impl crate::Resettable for TampSrSpec {
    const RESET_VALUE: u32 = 0;
}
