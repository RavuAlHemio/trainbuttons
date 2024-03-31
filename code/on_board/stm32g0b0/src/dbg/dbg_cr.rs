#[doc = "Register `DBG_CR` reader"]
pub type R = crate::R<DbgCrSpec>;
#[doc = "Register `DBG_CR` writer"]
pub type W = crate::W<DbgCrSpec>;
#[doc = "Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgStop {
    #[doc = "0: All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    B0x0 = 0,
    #[doc = "1: FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    B0x1 = 1,
}
impl From<DbgStop> for bool {
    #[inline(always)]
    fn from(variant: DbgStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STOP` reader - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
pub type DbgStopR = crate::BitReader<DbgStop>;
impl DbgStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgStop {
        match self.bits {
            false => DbgStop::B0x0,
            true => DbgStop::B0x1,
        }
    }
    #[doc = "All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgStop::B0x0
    }
    #[doc = "FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgStop::B0x1
    }
}
#[doc = "Field `DBG_STOP` writer - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG, DbgStop>;
impl<'a, REG> DbgStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgStop::B0x0)
    }
    #[doc = "FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgStop::B0x1)
    }
}
#[doc = "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgStandby {
    #[doc = "0: Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    B0x0 = 0,
    #[doc = "1: Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    B0x1 = 1,
}
impl From<DbgStandby> for bool {
    #[inline(always)]
    fn from(variant: DbgStandby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STANDBY` reader - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
pub type DbgStandbyR = crate::BitReader<DbgStandby>;
impl DbgStandbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgStandby {
        match self.bits {
            false => DbgStandby::B0x0,
            true => DbgStandby::B0x1,
        }
    }
    #[doc = "Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgStandby::B0x0
    }
    #[doc = "Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgStandby::B0x1
    }
}
#[doc = "Field `DBG_STANDBY` writer - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
pub type DbgStandbyW<'a, REG> = crate::BitWriter<'a, REG, DbgStandby>;
impl<'a, REG> DbgStandbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgStandby::B0x0)
    }
    #[doc = "Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgStandby::B0x1)
    }
}
impl R {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DbgStandbyR {
        DbgStandbyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DbgStopW<DbgCrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DbgStandbyW<DbgCrSpec> {
        DbgStandbyW::new(self, 2)
    }
}
#[doc = "DBG configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCrSpec;
impl crate::RegisterSpec for DbgCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cr::R`](R) reader structure"]
impl crate::Readable for DbgCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_cr::W`](W) writer structure"]
impl crate::Writable for DbgCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_CR to value 0"]
impl crate::Resettable for DbgCrSpec {
    const RESET_VALUE: u32 = 0;
}
