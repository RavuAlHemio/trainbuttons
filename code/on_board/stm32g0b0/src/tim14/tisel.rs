#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TiselSpec>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TiselSpec>;
#[doc = "selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti1sel {
    #[doc = "0: TIM14_CH1 input"]
    B0x0 = 0,
    #[doc = "1: RTC CLK"]
    B0x1 = 1,
    #[doc = "2: HSE/32"]
    B0x2 = 2,
    #[doc = "3: MCO"]
    B0x3 = 3,
}
impl From<Ti1sel> for u8 {
    #[inline(always)]
    fn from(variant: Ti1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti1sel {
    type Ux = u8;
}
impl crate::IsEnum for Ti1sel {}
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
pub type Ti1selR = crate::FieldReader<Ti1sel>;
impl Ti1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti1sel> {
        match self.bits {
            0 => Some(Ti1sel::B0x0),
            1 => Some(Ti1sel::B0x1),
            2 => Some(Ti1sel::B0x2),
            3 => Some(Ti1sel::B0x3),
            _ => None,
        }
    }
    #[doc = "TIM14_CH1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti1sel::B0x0
    }
    #[doc = "RTC CLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ti1sel::B0x1
    }
    #[doc = "HSE/32"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ti1sel::B0x2
    }
    #[doc = "MCO"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ti1sel::B0x3
    }
}
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti1sel>;
impl<'a, REG> Ti1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM14_CH1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x0)
    }
    #[doc = "RTC CLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x1)
    }
    #[doc = "HSE/32"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x2)
    }
    #[doc = "MCO"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> Ti1selW<TiselSpec> {
        Ti1selW::new(self, 0)
    }
}
#[doc = "TIM timer input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiselSpec;
impl crate::RegisterSpec for TiselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TiselSpec {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TiselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TiselSpec {
    const RESET_VALUE: u32 = 0;
}
