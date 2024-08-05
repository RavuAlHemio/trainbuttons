#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OptrSpec>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OptrSpec>;
#[doc = "Field `RDP` reader - Read protection level"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type NRstStopR = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type NRstStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type NRstStdbyR = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type NRstStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IdwgSwR = crate::BitReader;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IdwgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IwdgStopR = crate::BitReader;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IwdgStdbyR = crate::BitReader;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IwdgStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WwdgSwR = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WwdgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nSWAP_BANK` reader - nSWAP_BANK"]
pub type NSwapBankR = crate::BitReader;
#[doc = "Field `nSWAP_BANK` writer - nSWAP_BANK"]
pub type NSwapBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL_BANK` reader - DUAL_BANK"]
pub type DualBankR = crate::BitReader;
#[doc = "Field `DUAL_BANK` writer - DUAL_BANK"]
pub type DualBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PARITY_CHECK` reader - SRAM parity check control"]
pub type RamParityCheckR = crate::BitReader;
#[doc = "Field `RAM_PARITY_CHECK` writer - SRAM parity check control"]
pub type RamParityCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT_SEL` reader - nBOOT_SEL"]
pub type NBootSelR = crate::BitReader;
#[doc = "Field `nBOOT_SEL` writer - nBOOT_SEL"]
pub type NBootSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type NBoot1R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type NBoot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type NBoot0R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type NBoot0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRstStopR {
        NRstStopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRstStdbyR {
        NRstStdbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IdwgSwR {
        IdwgSwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IwdgStopR {
        IwdgStopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IwdgStdbyR {
        IwdgStdbyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WwdgSwR {
        WwdgSwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - nSWAP_BANK"]
    #[inline(always)]
    pub fn n_swap_bank(&self) -> NSwapBankR {
        NSwapBankR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DUAL_BANK"]
    #[inline(always)]
    pub fn dual_bank(&self) -> DualBankR {
        DualBankR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RamParityCheckR {
        RamParityCheckR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    pub fn n_boot_sel(&self) -> NBootSelR {
        NBootSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBoot1R {
        NBoot1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBoot0R {
        NBoot0R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RdpW<OptrSpec> {
        RdpW::new(self, 0)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> NRstStopW<OptrSpec> {
        NRstStopW::new(self, 13)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> NRstStdbyW<OptrSpec> {
        NRstStdbyW::new(self, 14)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IdwgSwW<OptrSpec> {
        IdwgSwW::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IwdgStopW<OptrSpec> {
        IwdgStopW::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IwdgStdbyW<OptrSpec> {
        IwdgStdbyW::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WwdgSwW<OptrSpec> {
        WwdgSwW::new(self, 19)
    }
    #[doc = "Bit 20 - nSWAP_BANK"]
    #[inline(always)]
    #[must_use]
    pub fn n_swap_bank(&mut self) -> NSwapBankW<OptrSpec> {
        NSwapBankW::new(self, 20)
    }
    #[doc = "Bit 21 - DUAL_BANK"]
    #[inline(always)]
    #[must_use]
    pub fn dual_bank(&mut self) -> DualBankW<OptrSpec> {
        DualBankW::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    #[must_use]
    pub fn ram_parity_check(&mut self) -> RamParityCheckW<OptrSpec> {
        RamParityCheckW::new(self, 22)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot_sel(&mut self) -> NBootSelW<OptrSpec> {
        NBootSelW::new(self, 24)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> NBoot1W<OptrSpec> {
        NBoot1W::new(self, 25)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> NBoot0W<OptrSpec> {
        NBoot0W::new(self, 26)
    }
}
#[doc = "Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptrSpec;
impl crate::RegisterSpec for OptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OptrSpec {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTR to value 0xf000_0000"]
impl crate::Resettable for OptrSpec {
    const RESET_VALUE: u32 = 0xf000_0000;
}
