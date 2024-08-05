#[doc = "Register `TIM1_CCR5` reader"]
pub type R = crate::R<Tim1Ccr5Spec>;
#[doc = "Register `TIM1_CCR5` writer"]
pub type W = crate::W<Tim1Ccr5Spec>;
#[doc = "Field `CCR5` reader - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
pub type Ccr5R = crate::FieldReader<u16>;
#[doc = "Field `CCR5` writer - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
pub type Ccr5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gc5c1 {
    #[doc = "0: No effect of OC5REF on OC1REFC5"]
    B0x0 = 0,
    #[doc = "1: OC1REFC is the logical AND of OC1REFC and OC5REF"]
    B0x1 = 1,
}
impl From<Gc5c1> for bool {
    #[inline(always)]
    fn from(variant: Gc5c1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC5C1` reader - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type Gc5c1R = crate::BitReader<Gc5c1>;
impl Gc5c1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gc5c1 {
        match self.bits {
            false => Gc5c1::B0x0,
            true => Gc5c1::B0x1,
        }
    }
    #[doc = "No effect of OC5REF on OC1REFC5"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gc5c1::B0x0
    }
    #[doc = "OC1REFC is the logical AND of OC1REFC and OC5REF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gc5c1::B0x1
    }
}
#[doc = "Field `GC5C1` writer - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type Gc5c1W<'a, REG> = crate::BitWriter<'a, REG, Gc5c1>;
impl<'a, REG> Gc5c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of OC5REF on OC1REFC5"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gc5c1::B0x0)
    }
    #[doc = "OC1REFC is the logical AND of OC1REFC and OC5REF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gc5c1::B0x1)
    }
}
#[doc = "Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gc5c2 {
    #[doc = "0: No effect of OC5REF on OC2REFC"]
    B0x0 = 0,
    #[doc = "1: OC2REFC is the logical AND of OC2REFC and OC5REF"]
    B0x1 = 1,
}
impl From<Gc5c2> for bool {
    #[inline(always)]
    fn from(variant: Gc5c2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC5C2` reader - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type Gc5c2R = crate::BitReader<Gc5c2>;
impl Gc5c2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gc5c2 {
        match self.bits {
            false => Gc5c2::B0x0,
            true => Gc5c2::B0x1,
        }
    }
    #[doc = "No effect of OC5REF on OC2REFC"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gc5c2::B0x0
    }
    #[doc = "OC2REFC is the logical AND of OC2REFC and OC5REF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gc5c2::B0x1
    }
}
#[doc = "Field `GC5C2` writer - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type Gc5c2W<'a, REG> = crate::BitWriter<'a, REG, Gc5c2>;
impl<'a, REG> Gc5c2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of OC5REF on OC2REFC"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gc5c2::B0x0)
    }
    #[doc = "OC2REFC is the logical AND of OC2REFC and OC5REF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gc5c2::B0x1)
    }
}
#[doc = "Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gc5c3 {
    #[doc = "0: No effect of OC5REF on OC3REFC"]
    B0x0 = 0,
    #[doc = "1: OC3REFC is the logical AND of OC3REFC and OC5REF"]
    B0x1 = 1,
}
impl From<Gc5c3> for bool {
    #[inline(always)]
    fn from(variant: Gc5c3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC5C3` reader - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type Gc5c3R = crate::BitReader<Gc5c3>;
impl Gc5c3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gc5c3 {
        match self.bits {
            false => Gc5c3::B0x0,
            true => Gc5c3::B0x1,
        }
    }
    #[doc = "No effect of OC5REF on OC3REFC"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gc5c3::B0x0
    }
    #[doc = "OC3REFC is the logical AND of OC3REFC and OC5REF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gc5c3::B0x1
    }
}
#[doc = "Field `GC5C3` writer - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type Gc5c3W<'a, REG> = crate::BitWriter<'a, REG, Gc5c3>;
impl<'a, REG> Gc5c3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of OC5REF on OC3REFC"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gc5c3::B0x0)
    }
    #[doc = "OC3REFC is the logical AND of OC3REFC and OC5REF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gc5c3::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
    #[inline(always)]
    pub fn ccr5(&self) -> Ccr5R {
        Ccr5R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c1(&self) -> Gc5c1R {
        Gc5c1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c2(&self) -> Gc5c2R {
        Gc5c2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c3(&self) -> Gc5c3R {
        Gc5c3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> Ccr5W<Tim1Ccr5Spec> {
        Ccr5W::new(self, 0)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> Gc5c1W<Tim1Ccr5Spec> {
        Gc5c1W::new(self, 29)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> Gc5c2W<Tim1Ccr5Spec> {
        Gc5c2W::new(self, 30)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> Gc5c3W<Tim1Ccr5Spec> {
        Gc5c3W::new(self, 31)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr5Spec;
impl crate::RegisterSpec for Tim1Ccr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccr5::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr5::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR5 to value 0"]
impl crate::Resettable for Tim1Ccr5Spec {
    const RESET_VALUE: u32 = 0;
}
