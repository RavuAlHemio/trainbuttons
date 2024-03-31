#[doc = "Register `DBG_APB_FZ2` reader"]
pub type R = crate::R<DbgApbFz2Spec>;
#[doc = "Register `DBG_APB_FZ2` writer"]
pub type W = crate::W<DbgApbFz2Spec>;
#[doc = "Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim1Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim1Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DbgTim1StopR = crate::BitReader<DbgTim1Stop>;
impl DbgTim1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim1Stop {
        match self.bits {
            false => DbgTim1Stop::B0x0,
            true => DbgTim1Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim1Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim1Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim1Stop>;
impl<'a, REG> DbgTim1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim1Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim1Stop::B0x1)
    }
}
#[doc = "Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim14Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim14Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim14Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DbgTim14StopR = crate::BitReader<DbgTim14Stop>;
impl DbgTim14StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim14Stop {
        match self.bits {
            false => DbgTim14Stop::B0x0,
            true => DbgTim14Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim14Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim14Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DbgTim14StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim14Stop>;
impl<'a, REG> DbgTim14StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim14Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim14Stop::B0x1)
    }
}
#[doc = "Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim15Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim15Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim15Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM15_STOP` reader - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
pub type DbgTim15StopR = crate::BitReader<DbgTim15Stop>;
impl DbgTim15StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim15Stop {
        match self.bits {
            false => DbgTim15Stop::B0x0,
            true => DbgTim15Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim15Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim15Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM15_STOP` writer - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
pub type DbgTim15StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim15Stop>;
impl<'a, REG> DbgTim15StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim15Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim15Stop::B0x1)
    }
}
#[doc = "Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim16Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim16Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim16Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DbgTim16StopR = crate::BitReader<DbgTim16Stop>;
impl DbgTim16StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim16Stop {
        match self.bits {
            false => DbgTim16Stop::B0x0,
            true => DbgTim16Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim16Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim16Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim16Stop>;
impl<'a, REG> DbgTim16StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim16Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim16Stop::B0x1)
    }
}
#[doc = "Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim17Stop {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<DbgTim17Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim17Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DbgTim17StopR = crate::BitReader<DbgTim17Stop>;
impl DbgTim17StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim17Stop {
        match self.bits {
            false => DbgTim17Stop::B0x0,
            true => DbgTim17Stop::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim17Stop::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim17Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DbgTim17StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim17Stop>;
impl<'a, REG> DbgTim17StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim17Stop::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim17Stop::B0x1)
    }
}
impl R {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DbgTim14StopR {
        DbgTim14StopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DbgTim15StopR {
        DbgTim15StopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DbgTim17StopR {
        DbgTim17StopR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<DbgApbFz2Spec> {
        DbgTim1StopW::new(self, 11)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DbgTim14StopW<DbgApbFz2Spec> {
        DbgTim14StopW::new(self, 15)
    }
    #[doc = "Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DbgTim15StopW<DbgApbFz2Spec> {
        DbgTim15StopW::new(self, 16)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<DbgApbFz2Spec> {
        DbgTim16StopW::new(self, 17)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DbgTim17StopW<DbgApbFz2Spec> {
        DbgTim17StopW::new(self, 18)
    }
}
#[doc = "DBG APB freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgApbFz2Spec;
impl crate::RegisterSpec for DbgApbFz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb_fz2::R`](R) reader structure"]
impl crate::Readable for DbgApbFz2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb_fz2::W`](W) writer structure"]
impl crate::Writable for DbgApbFz2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ2 to value 0"]
impl crate::Resettable for DbgApbFz2Spec {
    const RESET_VALUE: u32 = 0;
}
