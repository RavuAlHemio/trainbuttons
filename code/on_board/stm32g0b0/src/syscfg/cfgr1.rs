#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_RMP` reader - PA11_RMP"]
pub type Pa11RmpR = crate::BitReader;
#[doc = "Field `PA11_RMP` writer - PA11_RMP"]
pub type Pa11RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA12_RMP` reader - PA11 and PA12 remapping bit."]
pub type Pa12RmpR = crate::BitReader;
#[doc = "Field `PA12_RMP` writer - PA11 and PA12 remapping bit."]
pub type Pa12RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IrPolR = crate::BitReader;
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IrPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_MOD` reader - IR Modulation Envelope signal selection."]
pub type IrModR = crate::FieldReader;
#[doc = "Field `IR_MOD` writer - IR Modulation Envelope signal selection."]
pub type IrModW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub type BoostenR = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub type BoostenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1_STROBE` reader - Strobe signal bit for UCPD1"]
pub type Ucpd1StrobeR = crate::BitReader;
#[doc = "Field `UCPD1_STROBE` writer - Strobe signal bit for UCPD1"]
pub type Ucpd1StrobeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD2_STROBE` reader - Strobe signal bit for UCPD2"]
pub type Ucpd2StrobeR = crate::BitReader;
#[doc = "Field `UCPD2_STROBE` writer - Strobe signal bit for UCPD2"]
pub type Ucpd2StrobeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PBx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2cPbxFmpR = crate::BitReader;
#[doc = "Field `I2C_PBx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2cPbxFmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FMP` reader - I2C_PB7_FMP"]
pub type I2cPb7FmpR = crate::BitReader;
#[doc = "Field `I2C_PB7_FMP` writer - I2C_PB7_FMP"]
pub type I2cPb7FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FMP` reader - I2C_PB8_FMP"]
pub type I2cPb8FmpR = crate::BitReader;
#[doc = "Field `I2C_PB8_FMP` writer - I2C_PB8_FMP"]
pub type I2cPb8FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FMP` reader - I2C_PB9_FMP"]
pub type I2cPb9FmpR = crate::BitReader;
#[doc = "Field `I2C_PB9_FMP` writer - I2C_PB9_FMP"]
pub type I2cPb9FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1"]
pub type I2c1FmpR = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1"]
pub type I2c1FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2"]
pub type I2c2FmpR = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2"]
pub type I2c2FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2cPa9FmpR = crate::BitReader;
#[doc = "Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2cPa9FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2cPa10FmpR = crate::BitReader;
#[doc = "Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2cPa10FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C3_FMP"]
pub type I2c3FmpR = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C3_FMP"]
pub type I2c3FmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - PA11_RMP"]
    #[inline(always)]
    pub fn pa11_rmp(&self) -> Pa11RmpR {
        Pa11RmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit."]
    #[inline(always)]
    pub fn pa12_rmp(&self) -> Pa12RmpR {
        Pa12RmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IrPolR {
        IrPolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection."]
    #[inline(always)]
    pub fn ir_mod(&self) -> IrModR {
        IrModR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BoostenR {
        BoostenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Strobe signal bit for UCPD1"]
    #[inline(always)]
    pub fn ucpd1_strobe(&self) -> Ucpd1StrobeR {
        Ucpd1StrobeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Strobe signal bit for UCPD2"]
    #[inline(always)]
    pub fn ucpd2_strobe(&self) -> Ucpd2StrobeR {
        Ucpd2StrobeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pbx_fmp(&self) -> I2cPbxFmpR {
        I2cPbxFmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C_PB7_FMP"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2cPb7FmpR {
        I2cPb7FmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C_PB8_FMP"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2cPb8FmpR {
        I2cPb8FmpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C_PB9_FMP"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2cPb9FmpR {
        I2cPb9FmpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2c1FmpR {
        I2c1FmpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2c2FmpR {
        I2c2FmpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2cPa9FmpR {
        I2cPa9FmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2cPa10FmpR {
        I2cPa10FmpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C3_FMP"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2c3FmpR {
        I2c3FmpR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MemModeW<Cfgr1Spec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 3 - PA11_RMP"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_rmp(&mut self) -> Pa11RmpW<Cfgr1Spec> {
        Pa11RmpW::new(self, 3)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit."]
    #[inline(always)]
    #[must_use]
    pub fn pa12_rmp(&mut self) -> Pa12RmpW<Cfgr1Spec> {
        Pa12RmpW::new(self, 4)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IrPolW<Cfgr1Spec> {
        IrPolW::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection."]
    #[inline(always)]
    #[must_use]
    pub fn ir_mod(&mut self) -> IrModW<Cfgr1Spec> {
        IrModW::new(self, 6)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BoostenW<Cfgr1Spec> {
        BoostenW::new(self, 8)
    }
    #[doc = "Bit 9 - Strobe signal bit for UCPD1"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_strobe(&mut self) -> Ucpd1StrobeW<Cfgr1Spec> {
        Ucpd1StrobeW::new(self, 9)
    }
    #[doc = "Bit 10 - Strobe signal bit for UCPD2"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2_strobe(&mut self) -> Ucpd2StrobeW<Cfgr1Spec> {
        Ucpd2StrobeW::new(self, 10)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pbx_fmp(&mut self) -> I2cPbxFmpW<Cfgr1Spec> {
        I2cPbxFmpW::new(self, 16)
    }
    #[doc = "Bit 17 - I2C_PB7_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2cPb7FmpW<Cfgr1Spec> {
        I2cPb7FmpW::new(self, 17)
    }
    #[doc = "Bit 18 - I2C_PB8_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2cPb8FmpW<Cfgr1Spec> {
        I2cPb8FmpW::new(self, 18)
    }
    #[doc = "Bit 19 - I2C_PB9_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2cPb9FmpW<Cfgr1Spec> {
        I2cPb9FmpW::new(self, 19)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2c1FmpW<Cfgr1Spec> {
        I2c1FmpW::new(self, 20)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2c2FmpW<Cfgr1Spec> {
        I2c2FmpW::new(self, 21)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa9_fmp(&mut self) -> I2cPa9FmpW<Cfgr1Spec> {
        I2cPa9FmpW::new(self, 22)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa10_fmp(&mut self) -> I2cPa10FmpW<Cfgr1Spec> {
        I2cPa10FmpW::new(self, 23)
    }
    #[doc = "Bit 24 - I2C3_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2c3FmpW<Cfgr1Spec> {
        I2c3FmpW::new(self, 24)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {
    const RESET_VALUE: u32 = 0;
}
