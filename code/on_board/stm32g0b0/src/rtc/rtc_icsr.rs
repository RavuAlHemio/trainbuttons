#[doc = "Register `RTC_ICSR` reader"]
pub type R = crate::R<RtcIcsrSpec>;
#[doc = "Register `RTC_ICSR` writer"]
pub type W = crate::W<RtcIcsrSpec>;
#[doc = "Alarm A write flag This bit is set by hardware when alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alrawf {
    #[doc = "0: Alarm A update not allowed"]
    B0x0 = 0,
    #[doc = "1: Alarm A update allowed"]
    B0x1 = 1,
}
impl From<Alrawf> for bool {
    #[inline(always)]
    fn from(variant: Alrawf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAWF` reader - Alarm A write flag This bit is set by hardware when alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
pub type AlrawfR = crate::BitReader<Alrawf>;
impl AlrawfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alrawf {
        match self.bits {
            false => Alrawf::B0x0,
            true => Alrawf::B0x1,
        }
    }
    #[doc = "Alarm A update not allowed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alrawf::B0x0
    }
    #[doc = "Alarm A update allowed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alrawf::B0x1
    }
}
#[doc = "Alarm B write flag This bit is set by hardware when alarm B values can be changed, after the ALRBE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alrbwf {
    #[doc = "0: Alarm B update not allowed"]
    B0x0 = 0,
    #[doc = "1: Alarm B update allowed"]
    B0x1 = 1,
}
impl From<Alrbwf> for bool {
    #[inline(always)]
    fn from(variant: Alrbwf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBWF` reader - Alarm B write flag This bit is set by hardware when alarm B values can be changed, after the ALRBE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
pub type AlrbwfR = crate::BitReader<Alrbwf>;
impl AlrbwfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alrbwf {
        match self.bits {
            false => Alrbwf::B0x0,
            true => Alrbwf::B0x1,
        }
    }
    #[doc = "Alarm B update not allowed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alrbwf::B0x0
    }
    #[doc = "Alarm B update allowed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alrbwf::B0x1
    }
}
#[doc = "Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wutwf {
    #[doc = "0: Wakeup timer configuration update not allowed except in initialization mode"]
    B0x0 = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    B0x1 = 1,
}
impl From<Wutwf> for bool {
    #[inline(always)]
    fn from(variant: Wutwf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
pub type WutwfR = crate::BitReader<Wutwf>;
impl WutwfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wutwf {
        match self.bits {
            false => Wutwf::B0x0,
            true => Wutwf::B0x1,
        }
    }
    #[doc = "Wakeup timer configuration update not allowed except in initialization mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wutwf::B0x0
    }
    #[doc = "Wakeup timer configuration update allowed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wutwf::B0x1
    }
}
#[doc = "Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shpf {
    #[doc = "0: No shift operation is pending"]
    B0x0 = 0,
    #[doc = "1: A shift operation is pending"]
    B0x1 = 1,
}
impl From<Shpf> for bool {
    #[inline(always)]
    fn from(variant: Shpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
pub type ShpfR = crate::BitReader<Shpf>;
impl ShpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shpf {
        match self.bits {
            false => Shpf::B0x0,
            true => Shpf::B0x1,
        }
    }
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Shpf::B0x0
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Shpf::B0x1
    }
}
#[doc = "Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inits {
    #[doc = "0: Calendar has not been initialized"]
    B0x0 = 0,
    #[doc = "1: Calendar has been initialized"]
    B0x1 = 1,
}
impl From<Inits> for bool {
    #[inline(always)]
    fn from(variant: Inits) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
pub type InitsR = crate::BitReader<Inits>;
impl InitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inits {
        match self.bits {
            false => Inits::B0x0,
            true => Inits::B0x1,
        }
    }
    #[doc = "Calendar has not been initialized"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Inits::B0x0
    }
    #[doc = "Calendar has been initialized"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Inits::B0x1
    }
}
#[doc = "Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsf {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    B0x0 = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    B0x1 = 1,
}
impl From<Rsf> for bool {
    #[inline(always)]
    fn from(variant: Rsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RsfR = crate::BitReader<Rsf>;
impl RsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsf {
        match self.bits {
            false => Rsf::B0x0,
            true => Rsf::B0x1,
        }
    }
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rsf::B0x0
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rsf::B0x1
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG, Rsf>;
impl<'a, REG> RsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsf::B0x0)
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsf::B0x1)
    }
}
#[doc = "Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initf {
    #[doc = "0: Calendar registers update is not allowed"]
    B0x0 = 0,
    #[doc = "1: Calendar registers update is allowed"]
    B0x1 = 1,
}
impl From<Initf> for bool {
    #[inline(always)]
    fn from(variant: Initf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
pub type InitfR = crate::BitReader<Initf>;
impl InitfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initf {
        match self.bits {
            false => Initf::B0x0,
            true => Initf::B0x1,
        }
    }
    #[doc = "Calendar registers update is not allowed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Initf::B0x0
    }
    #[doc = "Calendar registers update is allowed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Initf::B0x1
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Init {
    #[doc = "0: Free running mode"]
    B0x0 = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    B0x1 = 1,
}
impl From<Init> for bool {
    #[inline(always)]
    fn from(variant: Init) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type InitR = crate::BitReader<Init>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Init {
        match self.bits {
            false => Init::B0x0,
            true => Init::B0x1,
        }
    }
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Init::B0x0
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Init::B0x1
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Init>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0x0)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0x1)
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to ."]
pub type RecalpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A write flag This bit is set by hardware when alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
    #[inline(always)]
    pub fn alrawf(&self) -> AlrawfR {
        AlrawfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag This bit is set by hardware when alarm B values can be changed, after the ALRBE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
    #[inline(always)]
    pub fn alrbwf(&self) -> AlrbwfR {
        AlrbwfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        WutwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
    #[inline(always)]
    pub fn shpf(&self) -> ShpfR {
        ShpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        InitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to ."]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        RecalpfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<RtcIcsrSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<RtcIcsrSpec> {
        InitW::new(self, 7)
    }
}
#[doc = "RTC initialization control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcIcsrSpec;
impl crate::RegisterSpec for RtcIcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_icsr::R`](R) reader structure"]
impl crate::Readable for RtcIcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_icsr::W`](W) writer structure"]
impl crate::Writable for RtcIcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ICSR to value 0x07"]
impl crate::Resettable for RtcIcsrSpec {
    const RESET_VALUE: u32 = 0x07;
}
