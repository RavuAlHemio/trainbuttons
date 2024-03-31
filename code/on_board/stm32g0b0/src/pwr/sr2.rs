#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Field `FLASH_RDY` reader - Flash ready flag"]
pub type FlashRdyR = crate::BitReader;
#[doc = "Field `REGLPS` reader - Low-power regulator started"]
pub type ReglpsR = crate::BitReader;
#[doc = "Field `REGLPF` reader - Low-power regulator flag"]
pub type ReglpfR = crate::BitReader;
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VosfR = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Flash ready flag"]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FlashRdyR {
        FlashRdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low-power regulator started"]
    #[inline(always)]
    pub fn reglps(&self) -> ReglpsR {
        ReglpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> ReglpfR {
        ReglpfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VosfR {
        VosfR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for Sr2Spec {
    const RESET_VALUE: u32 = 0;
}
