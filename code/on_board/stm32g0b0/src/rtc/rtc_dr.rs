#[doc = "Register `RTC_DR` reader"]
pub type R = crate::R<RtcDrSpec>;
#[doc = "Register `RTC_DR` writer"]
pub type W = crate::W<RtcDrSpec>;
#[doc = "Field `DU` reader - Date units in BCD format"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units in BCD format"]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MU` reader - Month units in BCD format"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MU` writer - Month units in BCD format"]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub type MtR = crate::BitReader;
#[doc = "Field `MT` writer - Month tens in BCD format"]
pub type MtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Week day units ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdu {
    #[doc = "0: forbidden"]
    B0x0 = 0,
    #[doc = "1: Monday"]
    B0x1 = 1,
    #[doc = "7: Sunday"]
    B0x7 = 7,
}
impl From<Wdu> for u8 {
    #[inline(always)]
    fn from(variant: Wdu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdu {
    type Ux = u8;
}
impl crate::IsEnum for Wdu {}
#[doc = "Field `WDU` reader - Week day units ..."]
pub type WduR = crate::FieldReader<Wdu>;
impl WduR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdu> {
        match self.bits {
            0 => Some(Wdu::B0x0),
            1 => Some(Wdu::B0x1),
            7 => Some(Wdu::B0x7),
            _ => None,
        }
    }
    #[doc = "forbidden"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wdu::B0x0
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wdu::B0x1
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Wdu::B0x7
    }
}
#[doc = "Field `WDU` writer - Week day units ..."]
pub type WduW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdu>;
impl<'a, REG> WduW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "forbidden"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdu::B0x0)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdu::B0x1)
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Wdu::B0x7)
    }
}
#[doc = "Field `YU` reader - Year units in BCD format"]
pub type YuR = crate::FieldReader;
#[doc = "Field `YU` writer - Year units in BCD format"]
pub type YuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YT` reader - Year tens in BCD format"]
pub type YtR = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens in BCD format"]
pub type YtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units ..."]
    #[inline(always)]
    pub fn wdu(&self) -> WduR {
        WduR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn yu(&self) -> YuR {
        YuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn yt(&self) -> YtR {
        YtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DuW<RtcDrSpec> {
        DuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<RtcDrSpec> {
        DtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MuW<RtcDrSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MtW<RtcDrSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units ..."]
    #[inline(always)]
    #[must_use]
    pub fn wdu(&mut self) -> WduW<RtcDrSpec> {
        WduW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YuW<RtcDrSpec> {
        YuW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YtW<RtcDrSpec> {
        YtW::new(self, 20)
    }
}
#[doc = "RTC date register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcDrSpec;
impl crate::RegisterSpec for RtcDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_dr::R`](R) reader structure"]
impl crate::Readable for RtcDrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_dr::W`](W) writer structure"]
impl crate::Writable for RtcDrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_DR to value 0x2101"]
impl crate::Resettable for RtcDrSpec {
    const RESET_VALUE: u32 = 0x2101;
}
