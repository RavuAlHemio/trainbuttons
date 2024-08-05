#[doc = "Register `RTC_ALRMBSSR` reader"]
pub type R = crate::R<RtcAlrmbssrSpec>;
#[doc = "Register `RTC_ALRMBSSR` writer"]
pub type W = crate::W<RtcAlrmbssrSpec>;
#[doc = "Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SsR = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maskss {
    #[doc = "0: No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    B0x0 = 0,
    #[doc = "1: SS\\[14:1\\]
are don't care in alarm B comparison. Only SS\\[0\\]
is compared."]
    B0x1 = 1,
    #[doc = "2: SS\\[14:2\\]
are don't care in alarm B comparison. Only SS\\[1:0\\]
are compared."]
    B0x2 = 2,
    #[doc = "3: SS\\[14:3\\]
are don't care in alarm B comparison. Only SS\\[2:0\\]
are compared."]
    B0x3 = 3,
    #[doc = "12: SS\\[14:12\\]
are don't care in alarm B comparison. SS\\[11:0\\]
are compared."]
    B0xC = 12,
    #[doc = "13: SS\\[14:13\\]
are don't care in alarm B comparison. SS\\[12:0\\]
are compared."]
    B0xD = 13,
    #[doc = "14: SS\\[14\\]
is don't care in alarm B comparison. SS\\[13:0\\]
are compared."]
    B0xE = 14,
    #[doc = "15: All 15 SS bits are compared and must match to activate alarm."]
    B0xF = 15,
}
impl From<Maskss> for u8 {
    #[inline(always)]
    fn from(variant: Maskss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maskss {
    type Ux = u8;
}
impl crate::IsEnum for Maskss {}
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssR = crate::FieldReader<Maskss>;
impl MaskssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maskss> {
        match self.bits {
            0 => Some(Maskss::B0x0),
            1 => Some(Maskss::B0x1),
            2 => Some(Maskss::B0x2),
            3 => Some(Maskss::B0x3),
            12 => Some(Maskss::B0xC),
            13 => Some(Maskss::B0xD),
            14 => Some(Maskss::B0xE),
            15 => Some(Maskss::B0xF),
            _ => None,
        }
    }
    #[doc = "No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Maskss::B0x0
    }
    #[doc = "SS\\[14:1\\]
are don't care in alarm B comparison. Only SS\\[0\\]
is compared."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Maskss::B0x1
    }
    #[doc = "SS\\[14:2\\]
are don't care in alarm B comparison. Only SS\\[1:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Maskss::B0x2
    }
    #[doc = "SS\\[14:3\\]
are don't care in alarm B comparison. Only SS\\[2:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Maskss::B0x3
    }
    #[doc = "SS\\[14:12\\]
are don't care in alarm B comparison. SS\\[11:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Maskss::B0xC
    }
    #[doc = "SS\\[14:13\\]
are don't care in alarm B comparison. SS\\[12:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Maskss::B0xD
    }
    #[doc = "SS\\[14\\]
is don't care in alarm B comparison. SS\\[13:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Maskss::B0xE
    }
    #[doc = "All 15 SS bits are compared and must match to activate alarm."]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Maskss::B0xF
    }
}
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Maskss>;
impl<'a, REG> MaskssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0x0)
    }
    #[doc = "SS\\[14:1\\]
are don't care in alarm B comparison. Only SS\\[0\\]
is compared."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0x1)
    }
    #[doc = "SS\\[14:2\\]
are don't care in alarm B comparison. Only SS\\[1:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0x2)
    }
    #[doc = "SS\\[14:3\\]
are don't care in alarm B comparison. Only SS\\[2:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0x3)
    }
    #[doc = "SS\\[14:12\\]
are don't care in alarm B comparison. SS\\[11:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0xC)
    }
    #[doc = "SS\\[14:13\\]
are don't care in alarm B comparison. SS\\[12:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0xD)
    }
    #[doc = "SS\\[14\\]
is don't care in alarm B comparison. SS\\[13:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0xE)
    }
    #[doc = "All 15 SS bits are compared and must match to activate alarm."]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0xF)
    }
}
impl R {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MaskssR {
        MaskssR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<RtcAlrmbssrSpec> {
        SsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MaskssW<RtcAlrmbssrSpec> {
        MaskssW::new(self, 24)
    }
}
#[doc = "RTC alarm B sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmbssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrmbssrSpec;
impl crate::RegisterSpec for RtcAlrmbssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmbssr::R`](R) reader structure"]
impl crate::Readable for RtcAlrmbssrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmbssr::W`](W) writer structure"]
impl crate::Writable for RtcAlrmbssrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ALRMBSSR to value 0"]
impl crate::Resettable for RtcAlrmbssrSpec {
    const RESET_VALUE: u32 = 0;
}
