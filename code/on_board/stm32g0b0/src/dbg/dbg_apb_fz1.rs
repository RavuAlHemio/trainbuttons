#[doc = "Register `DBG_APB_FZ1` reader"]
pub type R = crate::R<DbgApbFz1Spec>;
#[doc = "Register `DBG_APB_FZ1` writer"]
pub type W = crate::W<DbgApbFz1Spec>;
#[doc = "Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim2Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim2Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim2Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM2_STOP` reader - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
pub type DbgTim2StopR = crate::BitReader<DbgTim2Stop>;
impl DbgTim2StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim2Stop {
        match self.bits {
            false => DbgTim2Stop::B0x0,
            true => DbgTim2Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim2Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim2Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM2_STOP` writer - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
pub type DbgTim2StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim2Stop>;
impl<'a, REG> DbgTim2StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim2Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim2Stop::B0x1)
    }
}
#[doc = "Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim3Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim3Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim3Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DbgTim3StopR = crate::BitReader<DbgTim3Stop>;
impl DbgTim3StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim3Stop {
        match self.bits {
            false => DbgTim3Stop::B0x0,
            true => DbgTim3Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim3Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim3Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim3Stop>;
impl<'a, REG> DbgTim3StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim3Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim3Stop::B0x1)
    }
}
#[doc = "Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim6Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim6Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim6Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM6_STOP` reader - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
pub type DbgTim6StopR = crate::BitReader<DbgTim6Stop>;
impl DbgTim6StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim6Stop {
        match self.bits {
            false => DbgTim6Stop::B0x0,
            true => DbgTim6Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim6Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim6Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM6_STOP` writer - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
pub type DbgTim6StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim6Stop>;
impl<'a, REG> DbgTim6StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim6Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim6Stop::B0x1)
    }
}
#[doc = "Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim7Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim7Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim7Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM7_STOP` reader - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
pub type DbgTim7StopR = crate::BitReader<DbgTim7Stop>;
impl DbgTim7StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim7Stop {
        match self.bits {
            false => DbgTim7Stop::B0x0,
            true => DbgTim7Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim7Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim7Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM7_STOP` writer - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim7Stop>;
impl<'a, REG> DbgTim7StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim7Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim7Stop::B0x1)
    }
}
#[doc = "Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgRtcStop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgRtcStop> for bool {
    #[inline(always)]
    fn from(variant: DbgRtcStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DbgRtcStopR = crate::BitReader<DbgRtcStop>;
impl DbgRtcStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgRtcStop {
        match self.bits {
            false => DbgRtcStop::B0x0,
            true => DbgRtcStop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgRtcStop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgRtcStop::B0x1
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG, DbgRtcStop>;
impl<'a, REG> DbgRtcStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgRtcStop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgRtcStop::B0x1)
    }
}
#[doc = "Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgWwdgStop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgWwdgStop> for bool {
    #[inline(always)]
    fn from(variant: DbgWwdgStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DbgWwdgStopR = crate::BitReader<DbgWwdgStop>;
impl DbgWwdgStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgWwdgStop {
        match self.bits {
            false => DbgWwdgStop::B0x0,
            true => DbgWwdgStop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgWwdgStop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgWwdgStop::B0x1
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG, DbgWwdgStop>;
impl<'a, REG> DbgWwdgStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgWwdgStop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgWwdgStop::B0x1)
    }
}
#[doc = "Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgIwdgStop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgIwdgStop> for bool {
    #[inline(always)]
    fn from(variant: DbgIwdgStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DbgIwdgStopR = crate::BitReader<DbgIwdgStop>;
impl DbgIwdgStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgIwdgStop {
        match self.bits {
            false => DbgIwdgStop::B0x0,
            true => DbgIwdgStop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgIwdgStop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgIwdgStop::B0x1
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG, DbgIwdgStop>;
impl<'a, REG> DbgIwdgStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgIwdgStop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgIwdgStop::B0x1)
    }
}
#[doc = "SMBUS timeout when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgI2c1SmbusTimeout {
    #[doc = "0: Same behavior as in normal mode"]
    B0x0 = 0,
    #[doc = "1: The SMBUS timeout is frozen"]
    B0x1 = 1,
}
impl From<DbgI2c1SmbusTimeout> for bool {
    #[inline(always)]
    fn from(variant: DbgI2c1SmbusTimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted"]
pub type DbgI2c1SmbusTimeoutR = crate::BitReader<DbgI2c1SmbusTimeout>;
impl DbgI2c1SmbusTimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgI2c1SmbusTimeout {
        match self.bits {
            false => DbgI2c1SmbusTimeout::B0x0,
            true => DbgI2c1SmbusTimeout::B0x1,
        }
    }
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgI2c1SmbusTimeout::B0x0
    }
    #[doc = "The SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgI2c1SmbusTimeout::B0x1
    }
}
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted"]
pub type DbgI2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG, DbgI2c1SmbusTimeout>;
impl<'a, REG> DbgI2c1SmbusTimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgI2c1SmbusTimeout::B0x0)
    }
    #[doc = "The SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgI2c1SmbusTimeout::B0x1)
    }
}
#[doc = "Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgLptim2Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgLptim2Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgLptim2Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
pub type DbgLptim2StopR = crate::BitReader<DbgLptim2Stop>;
impl DbgLptim2StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgLptim2Stop {
        match self.bits {
            false => DbgLptim2Stop::B0x0,
            true => DbgLptim2Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgLptim2Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgLptim2Stop::B0x1
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
pub type DbgLptim2StopW<'a, REG> = crate::BitWriter<'a, REG, DbgLptim2Stop>;
impl<'a, REG> DbgLptim2StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim2Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim2Stop::B0x1)
    }
}
#[doc = "Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgLptim1Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgLptim1Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgLptim1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` reader - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
pub type DbgLptim1StopR = crate::BitReader<DbgLptim1Stop>;
impl DbgLptim1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgLptim1Stop {
        match self.bits {
            false => DbgLptim1Stop::B0x0,
            true => DbgLptim1Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgLptim1Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgLptim1Stop::B0x1
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
pub type DbgLptim1StopW<'a, REG> = crate::BitWriter<'a, REG, DbgLptim1Stop>;
impl<'a, REG> DbgLptim1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim1Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim1Stop::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DbgTim2StopR {
        DbgTim2StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DbgTim6StopR {
        DbgTim6StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DbgI2c1SmbusTimeoutR {
        DbgI2c1SmbusTimeoutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DbgLptim2StopR {
        DbgLptim2StopR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DbgLptim1StopR {
        DbgLptim1StopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DbgTim2StopW<DbgApbFz1Spec> {
        DbgTim2StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<DbgApbFz1Spec> {
        DbgTim3StopW::new(self, 1)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DbgTim6StopW<DbgApbFz1Spec> {
        DbgTim6StopW::new(self, 4)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<DbgApbFz1Spec> {
        DbgTim7StopW::new(self, 5)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<DbgApbFz1Spec> {
        DbgRtcStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<DbgApbFz1Spec> {
        DbgWwdgStopW::new(self, 11)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<DbgApbFz1Spec> {
        DbgIwdgStopW::new(self, 12)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DbgI2c1SmbusTimeoutW<DbgApbFz1Spec> {
        DbgI2c1SmbusTimeoutW::new(self, 21)
    }
    #[doc = "Bit 30 - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DbgLptim2StopW<DbgApbFz1Spec> {
        DbgLptim2StopW::new(self, 30)
    }
    #[doc = "Bit 31 - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DbgLptim1StopW<DbgApbFz1Spec> {
        DbgLptim1StopW::new(self, 31)
    }
}
#[doc = "DBG APB freeze register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgApbFz1Spec;
impl crate::RegisterSpec for DbgApbFz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb_fz1::R`](R) reader structure"]
impl crate::Readable for DbgApbFz1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb_fz1::W`](W) writer structure"]
impl crate::Writable for DbgApbFz1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ1 to value 0"]
impl crate::Resettable for DbgApbFz1Spec {
    const RESET_VALUE: u32 = 0;
}
