#[doc = "Register `I2C_ICR` writer"]
pub type W = crate::W<I2cIcrSpec>;
#[doc = "Field `ADDRCF` writer - Address Matched flag clear"]
pub type AddrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear"]
pub type NackcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCF` writer - Stop detection flag clear"]
pub type StopcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRCF` writer - Bus error flag clear"]
pub type BerrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOCF` writer - Arbitration lost flag clear"]
pub type ArlocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear"]
pub type OvrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECCF` writer - PEC Error flag clear"]
pub type PeccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear"]
pub type TimoutcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTCF` writer - Alert flag clear"]
pub type AlertcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - Address Matched flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn addrcf(&mut self) -> AddrcfW<I2cIcrSpec> {
        AddrcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn nackcf(&mut self) -> NackcfW<I2cIcrSpec> {
        NackcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn stopcf(&mut self) -> StopcfW<I2cIcrSpec> {
        StopcfW::new(self, 5)
    }
    #[doc = "Bit 8 - Bus error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn berrcf(&mut self) -> BerrcfW<I2cIcrSpec> {
        BerrcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn arlocf(&mut self) -> ArlocfW<I2cIcrSpec> {
        ArlocfW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OvrcfW<I2cIcrSpec> {
        OvrcfW::new(self, 10)
    }
    #[doc = "Bit 11 - PEC Error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn peccf(&mut self) -> PeccfW<I2cIcrSpec> {
        PeccfW::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn timoutcf(&mut self) -> TimoutcfW<I2cIcrSpec> {
        TimoutcfW::new(self, 12)
    }
    #[doc = "Bit 13 - Alert flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn alertcf(&mut self) -> AlertcfW<I2cIcrSpec> {
        AlertcfW::new(self, 13)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cIcrSpec;
impl crate::RegisterSpec for I2cIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_icr::W`](W) writer structure"]
impl crate::Writable for I2cIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_ICR to value 0"]
impl crate::Resettable for I2cIcrSpec {
    const RESET_VALUE: u32 = 0;
}
