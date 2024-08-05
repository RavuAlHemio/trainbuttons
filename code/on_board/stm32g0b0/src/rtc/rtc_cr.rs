#[doc = "Register `RTC_CR` reader"]
pub type R = crate::R<RtcCrSpec>;
#[doc = "Register `RTC_CR` writer"]
pub type W = crate::W<RtcCrSpec>;
#[doc = "ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wucksel {
    #[doc = "0: RTC/16 clock is selected"]
    B0x0 = 0,
    #[doc = "1: RTC/8 clock is selected"]
    B0x1 = 1,
    #[doc = "2: RTC/4 clock is selected"]
    B0x2 = 2,
    #[doc = "3: RTC/2 clock is selected"]
    B0x3 = 3,
}
impl From<Wucksel> for u8 {
    #[inline(always)]
    fn from(variant: Wucksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wucksel {
    type Ux = u8;
}
impl crate::IsEnum for Wucksel {}
#[doc = "Field `WUCKSEL` reader - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
pub type WuckselR = crate::FieldReader<Wucksel>;
impl WuckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wucksel> {
        match self.bits {
            0 => Some(Wucksel::B0x0),
            1 => Some(Wucksel::B0x1),
            2 => Some(Wucksel::B0x2),
            3 => Some(Wucksel::B0x3),
            _ => None,
        }
    }
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wucksel::B0x0
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wucksel::B0x1
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Wucksel::B0x2
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Wucksel::B0x3
    }
}
#[doc = "Field `WUCKSEL` writer - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
pub type WuckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wucksel>;
impl<'a, REG> WuckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wucksel::B0x0)
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wucksel::B0x1)
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Wucksel::B0x2)
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Wucksel::B0x3)
    }
}
#[doc = "Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsedge {
    #[doc = "0: RTC_TS input rising edge generates a timestamp event"]
    B0x0 = 0,
    #[doc = "1: RTC_TS input falling edge generates a timestamp event"]
    B0x1 = 1,
}
impl From<Tsedge> for bool {
    #[inline(always)]
    fn from(variant: Tsedge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEDGE` reader - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TsedgeR = crate::BitReader<Tsedge>;
impl TsedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsedge {
        match self.bits {
            false => Tsedge::B0x0,
            true => Tsedge::B0x1,
        }
    }
    #[doc = "RTC_TS input rising edge generates a timestamp event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsedge::B0x0
    }
    #[doc = "RTC_TS input falling edge generates a timestamp event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsedge::B0x1
    }
}
#[doc = "Field `TSEDGE` writer - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TsedgeW<'a, REG> = crate::BitWriter<'a, REG, Tsedge>;
impl<'a, REG> TsedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_TS input rising edge generates a timestamp event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsedge::B0x0)
    }
    #[doc = "RTC_TS input falling edge generates a timestamp event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsedge::B0x1)
    }
}
#[doc = "RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refckon {
    #[doc = "0: RTC_REFIN detection disabled"]
    B0x0 = 0,
    #[doc = "1: RTC_REFIN detection enabled"]
    B0x1 = 1,
}
impl From<Refckon> for bool {
    #[inline(always)]
    fn from(variant: Refckon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
pub type RefckonR = crate::BitReader<Refckon>;
impl RefckonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refckon {
        match self.bits {
            false => Refckon::B0x0,
            true => Refckon::B0x1,
        }
    }
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Refckon::B0x0
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Refckon::B0x1
    }
}
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
pub type RefckonW<'a, REG> = crate::BitWriter<'a, REG, Refckon>;
impl<'a, REG> RefckonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Refckon::B0x0)
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Refckon::B0x1)
    }
}
#[doc = "Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypshad {
    #[doc = "0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles."]
    B0x0 = 0,
    #[doc = "1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    B0x1 = 1,
}
impl From<Bypshad> for bool {
    #[inline(always)]
    fn from(variant: Bypshad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BypshadR = crate::BitReader<Bypshad>;
impl BypshadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypshad {
        match self.bits {
            false => Bypshad::B0x0,
            true => Bypshad::B0x1,
        }
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bypshad::B0x0
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bypshad::B0x1
    }
}
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BypshadW<'a, REG> = crate::BitWriter<'a, REG, Bypshad>;
impl<'a, REG> BypshadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bypshad::B0x0)
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bypshad::B0x1)
    }
}
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmt {
    #[doc = "0: 24 hour/day format"]
    B0x0 = 0,
    #[doc = "1: AM/PM hour format"]
    B0x1 = 1,
}
impl From<Fmt> for bool {
    #[inline(always)]
    fn from(variant: Fmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMT` reader - Hour format"]
pub type FmtR = crate::BitReader<Fmt>;
impl FmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmt {
        match self.bits {
            false => Fmt::B0x0,
            true => Fmt::B0x1,
        }
    }
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmt::B0x0
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmt::B0x1
    }
}
#[doc = "Field `FMT` writer - Hour format"]
pub type FmtW<'a, REG> = crate::BitWriter<'a, REG, Fmt>;
impl<'a, REG> FmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmt::B0x0)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmt::B0x1)
    }
}
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alrae {
    #[doc = "0: Alarm A disabled"]
    B0x0 = 0,
    #[doc = "1: Alarm A enabled"]
    B0x1 = 1,
}
impl From<Alrae> for bool {
    #[inline(always)]
    fn from(variant: Alrae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type AlraeR = crate::BitReader<Alrae>;
impl AlraeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alrae {
        match self.bits {
            false => Alrae::B0x0,
            true => Alrae::B0x1,
        }
    }
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alrae::B0x0
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alrae::B0x1
    }
}
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type AlraeW<'a, REG> = crate::BitWriter<'a, REG, Alrae>;
impl<'a, REG> AlraeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Alrae::B0x0)
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Alrae::B0x1)
    }
}
#[doc = "Alarm B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alrbe {
    #[doc = "0: Alarm B disabled"]
    B0x0 = 0,
    #[doc = "1: Alarm B enabled"]
    B0x1 = 1,
}
impl From<Alrbe> for bool {
    #[inline(always)]
    fn from(variant: Alrbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type AlrbeR = crate::BitReader<Alrbe>;
impl AlrbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alrbe {
        match self.bits {
            false => Alrbe::B0x0,
            true => Alrbe::B0x1,
        }
    }
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alrbe::B0x0
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alrbe::B0x1
    }
}
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type AlrbeW<'a, REG> = crate::BitWriter<'a, REG, Alrbe>;
impl<'a, REG> AlrbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Alrbe::B0x0)
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Alrbe::B0x1)
    }
}
#[doc = "Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wute {
    #[doc = "0: Wakeup timer disabled"]
    B0x0 = 0,
    #[doc = "1: Wakeup timer enabled"]
    B0x1 = 1,
}
impl From<Wute> for bool {
    #[inline(always)]
    fn from(variant: Wute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTE` reader - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
pub type WuteR = crate::BitReader<Wute>;
impl WuteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wute {
        match self.bits {
            false => Wute::B0x0,
            true => Wute::B0x1,
        }
    }
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wute::B0x0
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wute::B0x1
    }
}
#[doc = "Field `WUTE` writer - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
pub type WuteW<'a, REG> = crate::BitWriter<'a, REG, Wute>;
impl<'a, REG> WuteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wute::B0x0)
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wute::B0x1)
    }
}
#[doc = "timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tse {
    #[doc = "0: timestamp disable"]
    B0x0 = 0,
    #[doc = "1: timestamp enable"]
    B0x1 = 1,
}
impl From<Tse> for bool {
    #[inline(always)]
    fn from(variant: Tse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TseR = crate::BitReader<Tse>;
impl TseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tse {
        match self.bits {
            false => Tse::B0x0,
            true => Tse::B0x1,
        }
    }
    #[doc = "timestamp disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tse::B0x0
    }
    #[doc = "timestamp enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tse::B0x1
    }
}
#[doc = "Field `TSE` writer - timestamp enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG, Tse>;
impl<'a, REG> TseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "timestamp disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tse::B0x0)
    }
    #[doc = "timestamp enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tse::B0x1)
    }
}
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alraie {
    #[doc = "0: Alarm A interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Alarm A interrupt enabled"]
    B0x1 = 1,
}
impl From<Alraie> for bool {
    #[inline(always)]
    fn from(variant: Alraie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type AlraieR = crate::BitReader<Alraie>;
impl AlraieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alraie {
        match self.bits {
            false => Alraie::B0x0,
            true => Alraie::B0x1,
        }
    }
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alraie::B0x0
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alraie::B0x1
    }
}
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type AlraieW<'a, REG> = crate::BitWriter<'a, REG, Alraie>;
impl<'a, REG> AlraieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Alraie::B0x0)
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Alraie::B0x1)
    }
}
#[doc = "Alarm B interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alrbie {
    #[doc = "0: Alarm B interrupt disable"]
    B0x0 = 0,
    #[doc = "1: Alarm B interrupt enable"]
    B0x1 = 1,
}
impl From<Alrbie> for bool {
    #[inline(always)]
    fn from(variant: Alrbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type AlrbieR = crate::BitReader<Alrbie>;
impl AlrbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alrbie {
        match self.bits {
            false => Alrbie::B0x0,
            true => Alrbie::B0x1,
        }
    }
    #[doc = "Alarm B interrupt disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alrbie::B0x0
    }
    #[doc = "Alarm B interrupt enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alrbie::B0x1
    }
}
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type AlrbieW<'a, REG> = crate::BitWriter<'a, REG, Alrbie>;
impl<'a, REG> AlrbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B interrupt disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Alrbie::B0x0)
    }
    #[doc = "Alarm B interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Alrbie::B0x1)
    }
}
#[doc = "Wakeup timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wutie {
    #[doc = "0: Wakeup timer interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Wakeup timer interrupt enabled"]
    B0x1 = 1,
}
impl From<Wutie> for bool {
    #[inline(always)]
    fn from(variant: Wutie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WutieR = crate::BitReader<Wutie>;
impl WutieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wutie {
        match self.bits {
            false => Wutie::B0x0,
            true => Wutie::B0x1,
        }
    }
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wutie::B0x0
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wutie::B0x1
    }
}
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WutieW<'a, REG> = crate::BitWriter<'a, REG, Wutie>;
impl<'a, REG> WutieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wutie::B0x0)
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wutie::B0x1)
    }
}
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsie {
    #[doc = "0: Timestamp interrupt disable"]
    B0x0 = 0,
    #[doc = "1: Timestamp interrupt enable"]
    B0x1 = 1,
}
impl From<Tsie> for bool {
    #[inline(always)]
    fn from(variant: Tsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIE` reader - Timestamp interrupt enable"]
pub type TsieR = crate::BitReader<Tsie>;
impl TsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsie {
        match self.bits {
            false => Tsie::B0x0,
            true => Tsie::B0x1,
        }
    }
    #[doc = "Timestamp interrupt disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsie::B0x0
    }
    #[doc = "Timestamp interrupt enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsie::B0x1
    }
}
#[doc = "Field `TSIE` writer - Timestamp interrupt enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG, Tsie>;
impl<'a, REG> TsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp interrupt disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsie::B0x0)
    }
    #[doc = "Timestamp interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsie::B0x1)
    }
}
#[doc = "Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Add1h {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Adds 1 hour to the current time. This can be used for summer time change"]
    B0x1 = 1,
}
impl From<Add1h> for bool {
    #[inline(always)]
    fn from(variant: Add1h) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
pub type Add1hW<'a, REG> = crate::BitWriter<'a, REG, Add1h>;
impl<'a, REG> Add1hW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Add1h::B0x0)
    }
    #[doc = "Adds 1 hour to the current time. This can be used for summer time change"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Add1h::B0x1)
    }
}
#[doc = "Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sub1h {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Subtracts 1 hour to the current time. This can be used for winter time change."]
    B0x1 = 1,
}
impl From<Sub1h> for bool {
    #[inline(always)]
    fn from(variant: Sub1h) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
pub type Sub1hW<'a, REG> = crate::BitWriter<'a, REG, Sub1h>;
impl<'a, REG> Sub1hW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1h::B0x0)
    }
    #[doc = "Subtracts 1 hour to the current time. This can be used for winter time change."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1h::B0x1)
    }
}
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cosel {
    #[doc = "0: Calibration output is 512Hz"]
    B0x0 = 0,
    #[doc = "1: Calibration output is 1Hz"]
    B0x1 = 1,
}
impl From<Cosel> for bool {
    #[inline(always)]
    fn from(variant: Cosel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEL` reader - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
pub type CoselR = crate::BitReader<Cosel>;
impl CoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cosel {
        match self.bits {
            false => Cosel::B0x0,
            true => Cosel::B0x1,
        }
    }
    #[doc = "Calibration output is 512Hz"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cosel::B0x0
    }
    #[doc = "Calibration output is 1Hz"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cosel::B0x1
    }
}
#[doc = "Field `COSEL` writer - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
pub type CoselW<'a, REG> = crate::BitWriter<'a, REG, Cosel>;
impl<'a, REG> CoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output is 512Hz"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cosel::B0x0)
    }
    #[doc = "Calibration output is 1Hz"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cosel::B0x1)
    }
}
#[doc = "Output polarity This bit is used to configure the polarity of TAMPALRM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol {
    #[doc = "0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    B0x0 = 0,
    #[doc = "1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    B0x1 = 1,
}
impl From<Pol> for bool {
    #[inline(always)]
    fn from(variant: Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL` reader - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type PolR = crate::BitReader<Pol>;
impl PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol {
        match self.bits {
            false => Pol::B0x0,
            true => Pol::B0x1,
        }
    }
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pol::B0x0
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pol::B0x1
    }
}
#[doc = "Field `POL` writer - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG, Pol>;
impl<'a, REG> PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol::B0x0)
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol::B0x1)
    }
}
#[doc = "Output selection These bits are used to select the flag to be routed to TAMPALRM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osel {
    #[doc = "0: Output disabled"]
    B0x0 = 0,
    #[doc = "1: Alarm A output enabled"]
    B0x1 = 1,
    #[doc = "2: Alarm B output enabled"]
    B0x2 = 2,
    #[doc = "3: Wakeup output enabled"]
    B0x3 = 3,
}
impl From<Osel> for u8 {
    #[inline(always)]
    fn from(variant: Osel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osel {
    type Ux = u8;
}
impl crate::IsEnum for Osel {}
#[doc = "Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OselR = crate::FieldReader<Osel>;
impl OselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osel {
        match self.bits {
            0 => Osel::B0x0,
            1 => Osel::B0x1,
            2 => Osel::B0x2,
            3 => Osel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Osel::B0x0
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Osel::B0x1
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Osel::B0x2
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Osel::B0x3
    }
}
#[doc = "Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Osel, crate::Safe>;
impl<'a, REG> OselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Osel::B0x0)
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Osel::B0x1)
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Osel::B0x2)
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Osel::B0x3)
    }
}
#[doc = "Calibration output enable This bit enables the CALIB output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coe {
    #[doc = "0: Calibration output disabled"]
    B0x0 = 0,
    #[doc = "1: Calibration output enabled"]
    B0x1 = 1,
}
impl From<Coe> for bool {
    #[inline(always)]
    fn from(variant: Coe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COE` reader - Calibration output enable This bit enables the CALIB output"]
pub type CoeR = crate::BitReader<Coe>;
impl CoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Coe {
        match self.bits {
            false => Coe::B0x0,
            true => Coe::B0x1,
        }
    }
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Coe::B0x0
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Coe::B0x1
    }
}
#[doc = "Field `COE` writer - Calibration output enable This bit enables the CALIB output"]
pub type CoeW<'a, REG> = crate::BitWriter<'a, REG, Coe>;
impl<'a, REG> CoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Coe::B0x0)
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Coe::B0x1)
    }
}
#[doc = "timestamp on internal event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itse {
    #[doc = "0: internal event timestamp disabled"]
    B0x0 = 0,
    #[doc = "1: internal event timestamp enabled"]
    B0x1 = 1,
}
impl From<Itse> for bool {
    #[inline(always)]
    fn from(variant: Itse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ItseR = crate::BitReader<Itse>;
impl ItseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itse {
        match self.bits {
            false => Itse::B0x0,
            true => Itse::B0x1,
        }
    }
    #[doc = "internal event timestamp disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itse::B0x0
    }
    #[doc = "internal event timestamp enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itse::B0x1
    }
}
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ItseW<'a, REG> = crate::BitWriter<'a, REG, Itse>;
impl<'a, REG> ItseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "internal event timestamp disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itse::B0x0)
    }
    #[doc = "internal event timestamp enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itse::B0x1)
    }
}
#[doc = "Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tampts {
    #[doc = "0: Tamper detection event does not cause a RTC timestamp to be saved"]
    B0x0 = 0,
    #[doc = "1: Save RTC timestamp on tamper detection event"]
    B0x1 = 1,
}
impl From<Tampts> for bool {
    #[inline(always)]
    fn from(variant: Tampts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
pub type TamptsR = crate::BitReader<Tampts>;
impl TamptsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tampts {
        match self.bits {
            false => Tampts::B0x0,
            true => Tampts::B0x1,
        }
    }
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tampts::B0x0
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tampts::B0x1
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
pub type TamptsW<'a, REG> = crate::BitWriter<'a, REG, Tampts>;
impl<'a, REG> TamptsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tampts::B0x0)
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tampts::B0x1)
    }
}
#[doc = "Tamper detection output enable on TAMPALRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tampoe {
    #[doc = "0: The tamper flag is not routed on TAMPALRM"]
    B0x0 = 0,
    #[doc = "1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL."]
    B0x1 = 1,
}
impl From<Tampoe> for bool {
    #[inline(always)]
    fn from(variant: Tampoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM"]
pub type TampoeR = crate::BitReader<Tampoe>;
impl TampoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tampoe {
        match self.bits {
            false => Tampoe::B0x0,
            true => Tampoe::B0x1,
        }
    }
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tampoe::B0x0
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tampoe::B0x1
    }
}
#[doc = "Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM"]
pub type TampoeW<'a, REG> = crate::BitWriter<'a, REG, Tampoe>;
impl<'a, REG> TampoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tampoe::B0x0)
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tampoe::B0x1)
    }
}
#[doc = "TAMPALRM pull-up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TampalrmPu {
    #[doc = "0: No pull-up is applied on TAMPALRM output"]
    B0x0 = 0,
    #[doc = "1: A pull-up is applied on TAMPALRM output"]
    B0x1 = 1,
}
impl From<TampalrmPu> for bool {
    #[inline(always)]
    fn from(variant: TampalrmPu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable"]
pub type TampalrmPuR = crate::BitReader<TampalrmPu>;
impl TampalrmPuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TampalrmPu {
        match self.bits {
            false => TampalrmPu::B0x0,
            true => TampalrmPu::B0x1,
        }
    }
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TampalrmPu::B0x0
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TampalrmPu::B0x1
    }
}
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable"]
pub type TampalrmPuW<'a, REG> = crate::BitWriter<'a, REG, TampalrmPu>;
impl<'a, REG> TampalrmPuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TampalrmPu::B0x0)
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TampalrmPu::B0x1)
    }
}
#[doc = "TAMPALRM output type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TampalrmType {
    #[doc = "0: TAMPALRM is push-pull output"]
    B0x0 = 0,
    #[doc = "1: TAMPALRM is open-drain output"]
    B0x1 = 1,
}
impl From<TampalrmType> for bool {
    #[inline(always)]
    fn from(variant: TampalrmType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM output type"]
pub type TampalrmTypeR = crate::BitReader<TampalrmType>;
impl TampalrmTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TampalrmType {
        match self.bits {
            false => TampalrmType::B0x0,
            true => TampalrmType::B0x1,
        }
    }
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TampalrmType::B0x0
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TampalrmType::B0x1
    }
}
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM output type"]
pub type TampalrmTypeW<'a, REG> = crate::BitWriter<'a, REG, TampalrmType>;
impl<'a, REG> TampalrmTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TampalrmType::B0x0)
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TampalrmType::B0x1)
    }
}
#[doc = "Field `OUT2EN` reader - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type Out2enR = crate::BitReader;
#[doc = "Field `OUT2EN` writer - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type Out2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
    #[inline(always)]
    pub fn wucksel(&self) -> WuckselR {
        WuckselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&self) -> TsedgeR {
        TsedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&self) -> RefckonR {
        RefckonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&self) -> BypshadR {
        BypshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> AlraeR {
        AlraeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> AlrbeR {
        AlrbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
    #[inline(always)]
    pub fn wute(&self) -> WuteR {
        WuteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> AlraieR {
        AlraieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> AlrbieR {
        AlrbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WutieR {
        WutieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
    #[inline(always)]
    pub fn cosel(&self) -> CoselR {
        CoselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    pub fn osel(&self) -> OselR {
        OselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    pub fn coe(&self) -> CoeR {
        CoeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ItseR {
        ItseR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
    #[inline(always)]
    pub fn tampts(&self) -> TamptsR {
        TamptsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&self) -> TampoeR {
        TampoeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TampalrmPuR {
        TampalrmPuR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TampalrmTypeR {
        TampalrmTypeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    pub fn out2en(&self) -> Out2enR {
        Out2enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WuckselW<RtcCrSpec> {
        WuckselW::new(self, 0)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TsedgeW<RtcCrSpec> {
        TsedgeW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> RefckonW<RtcCrSpec> {
        RefckonW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BypshadW<RtcCrSpec> {
        BypshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FmtW<RtcCrSpec> {
        FmtW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> AlraeW<RtcCrSpec> {
        AlraeW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> AlrbeW<RtcCrSpec> {
        AlrbeW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WuteW<RtcCrSpec> {
        WuteW::new(self, 10)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TseW<RtcCrSpec> {
        TseW::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> AlraieW<RtcCrSpec> {
        AlraieW::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> AlrbieW<RtcCrSpec> {
        AlrbieW::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WutieW<RtcCrSpec> {
        WutieW::new(self, 14)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TsieW<RtcCrSpec> {
        TsieW::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> Add1hW<RtcCrSpec> {
        Add1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> Sub1hW<RtcCrSpec> {
        Sub1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BkpW<RtcCrSpec> {
        BkpW::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> CoselW<RtcCrSpec> {
        CoselW::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<RtcCrSpec> {
        PolW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OselW<RtcCrSpec> {
        OselW::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> CoeW<RtcCrSpec> {
        CoeW::new(self, 23)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ItseW<RtcCrSpec> {
        ItseW::new(self, 24)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TamptsW<RtcCrSpec> {
        TamptsW::new(self, 25)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TampoeW<RtcCrSpec> {
        TampoeW::new(self, 26)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TampalrmPuW<RtcCrSpec> {
        TampalrmPuW::new(self, 29)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TampalrmTypeW<RtcCrSpec> {
        TampalrmTypeW::new(self, 30)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> Out2enW<RtcCrSpec> {
        Out2enW::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCrSpec;
impl crate::RegisterSpec for RtcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_cr::R`](R) reader structure"]
impl crate::Readable for RtcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cr::W`](W) writer structure"]
impl crate::Writable for RtcCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_CR to value 0"]
impl crate::Resettable for RtcCrSpec {
    const RESET_VALUE: u32 = 0;
}
