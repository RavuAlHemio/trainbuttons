#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TiselSpec>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TiselSpec>;
#[doc = "TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti1sel {
    #[doc = "0: TIM2_CH1 input"]
    B0x0 = 0,
    #[doc = "1: COMP1 output"]
    B0x1 = 1,
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
#[doc = "Field `TI1SEL` reader - TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved"]
pub type Ti1selR = crate::FieldReader<Ti1sel>;
impl Ti1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti1sel> {
        match self.bits {
            0 => Some(Ti1sel::B0x0),
            1 => Some(Ti1sel::B0x1),
            _ => None,
        }
    }
    #[doc = "TIM2_CH1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti1sel::B0x0
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ti1sel::B0x1
    }
}
#[doc = "Field `TI1SEL` writer - TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti1sel>;
impl<'a, REG> Ti1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM2_CH1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x0)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x1)
    }
}
#[doc = "TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti2sel {
    #[doc = "0: TIM2_CH2 input"]
    B0x0 = 0,
    #[doc = "1: COMP2 output"]
    B0x1 = 1,
}
impl From<Ti2sel> for u8 {
    #[inline(always)]
    fn from(variant: Ti2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti2sel {
    type Ux = u8;
}
impl crate::IsEnum for Ti2sel {}
#[doc = "Field `TI2SEL` reader - TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved"]
pub type Ti2selR = crate::FieldReader<Ti2sel>;
impl Ti2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti2sel> {
        match self.bits {
            0 => Some(Ti2sel::B0x0),
            1 => Some(Ti2sel::B0x1),
            _ => None,
        }
    }
    #[doc = "TIM2_CH2 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti2sel::B0x0
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ti2sel::B0x1
    }
}
#[doc = "Field `TI2SEL` writer - TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved"]
pub type Ti2selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti2sel>;
impl<'a, REG> Ti2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM2_CH2 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti2sel::B0x0)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti2sel::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    pub fn ti2sel(&self) -> Ti2selR {
        Ti2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> Ti1selW<TiselSpec> {
        Ti1selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> Ti2selW<TiselSpec> {
        Ti2selW::new(self, 8)
    }
}
#[doc = "TIM alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
