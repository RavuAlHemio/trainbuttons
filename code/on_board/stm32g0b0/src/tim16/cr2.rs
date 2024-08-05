#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpc {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    B0x0 = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when COM bit is set."]
    B0x1 = 1,
}
impl From<Ccpc> for bool {
    #[inline(always)]
    fn from(variant: Ccpc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CcpcR = crate::BitReader<Ccpc>;
impl CcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpc {
        match self.bits {
            false => Ccpc::B0x0,
            true => Ccpc::B0x1,
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccpc::B0x0
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when COM bit is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccpc::B0x1
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG, Ccpc>;
impl<'a, REG> CcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpc::B0x0)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when COM bit is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpc::B0x1)
    }
}
#[doc = "Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccus {
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only."]
    B0x0 = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI."]
    B0x1 = 1,
}
impl From<Ccus> for bool {
    #[inline(always)]
    fn from(variant: Ccus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CcusR = crate::BitReader<Ccus>;
impl CcusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccus {
        match self.bits {
            false => Ccus::B0x0,
            true => Ccus::B0x1,
        }
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccus::B0x0
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccus::B0x1
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG, Ccus>;
impl<'a, REG> CcusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccus::B0x0)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccus::B0x1)
    }
}
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccds {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    B0x0 = 0,
    #[doc = "1: CCx DMA requests sent when update event occurs"]
    B0x1 = 1,
}
impl From<Ccds> for bool {
    #[inline(always)]
    fn from(variant: Ccds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CcdsR = crate::BitReader<Ccds>;
impl CcdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccds {
        match self.bits {
            false => Ccds::B0x0,
            true => Ccds::B0x1,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccds::B0x0
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccds::B0x1
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG, Ccds>;
impl<'a, REG> CcdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccds::B0x0)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccds::B0x1)
    }
}
#[doc = "Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ois1 {
    #[doc = "0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    B0x0 = 0,
    #[doc = "1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    B0x1 = 1,
}
impl From<Ois1> for bool {
    #[inline(always)]
    fn from(variant: Ois1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1R = crate::BitReader<Ois1>;
impl Ois1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ois1 {
        match self.bits {
            false => Ois1::B0x0,
            true => Ois1::B0x1,
        }
    }
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ois1::B0x0
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ois1::B0x1
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG, Ois1>;
impl<'a, REG> Ois1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1::B0x0)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1::B0x1)
    }
}
#[doc = "Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ois1n {
    #[doc = "0: OC1N=0 after a dead-time when MOE=0"]
    B0x0 = 0,
    #[doc = "1: OC1N=1 after a dead-time when MOE=0"]
    B0x1 = 1,
}
impl From<Ois1n> for bool {
    #[inline(always)]
    fn from(variant: Ois1n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1nR = crate::BitReader<Ois1n>;
impl Ois1nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ois1n {
        match self.bits {
            false => Ois1n::B0x0,
            true => Ois1n::B0x1,
        }
    }
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ois1n::B0x0
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ois1n::B0x1
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG, Ois1n>;
impl<'a, REG> Ois1nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1n::B0x0)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ois1n::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CcpcW<Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CcusW<Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CcdsW<Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> Ois1W<Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> Ois1nW<Cr2Spec> {
        Ois1nW::new(self, 9)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
