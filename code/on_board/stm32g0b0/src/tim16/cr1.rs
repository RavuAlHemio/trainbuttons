#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cen {
    #[doc = "0: Counter disabled"]
    B0x0 = 0,
    #[doc = "1: Counter enabled"]
    B0x1 = 1,
}
impl From<Cen> for bool {
    #[inline(always)]
    fn from(variant: Cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
pub type CenR = crate::BitReader<Cen>;
impl CenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cen {
        match self.bits {
            false => Cen::B0x0,
            true => Cen::B0x1,
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cen::B0x0
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cen::B0x1
    }
}
#[doc = "Field `CEN` writer - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG, Cen>;
impl<'a, REG> CenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::B0x0)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::B0x1)
    }
}
#[doc = "Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udis {
    #[doc = "0: UEV enabled. The Update (UEV) event is generated by one of the following events:"]
    B0x0 = 0,
    #[doc = "1: UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    B0x1 = 1,
}
impl From<Udis> for bool {
    #[inline(always)]
    fn from(variant: Udis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDIS` reader - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
pub type UdisR = crate::BitReader<Udis>;
impl UdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udis {
        match self.bits {
            false => Udis::B0x0,
            true => Udis::B0x1,
        }
    }
    #[doc = "UEV enabled. The Update (UEV) event is generated by one of the following events:"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Udis::B0x0
    }
    #[doc = "UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Udis::B0x1
    }
}
#[doc = "Field `UDIS` writer - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
pub type UdisW<'a, REG> = crate::BitWriter<'a, REG, Udis>;
impl<'a, REG> UdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UEV enabled. The Update (UEV) event is generated by one of the following events:"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Udis::B0x0)
    }
    #[doc = "UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Udis::B0x1)
    }
}
#[doc = "Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Urs {
    #[doc = "0: Any of the following events generate an update interrupt or DMA request if enabled. These events can be:"]
    B0x0 = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    B0x1 = 1,
}
impl From<Urs> for bool {
    #[inline(always)]
    fn from(variant: Urs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `URS` reader - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
pub type UrsR = crate::BitReader<Urs>;
impl UrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Urs {
        match self.bits {
            false => Urs::B0x0,
            true => Urs::B0x1,
        }
    }
    #[doc = "Any of the following events generate an update interrupt or DMA request if enabled. These events can be:"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Urs::B0x0
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Urs::B0x1
    }
}
#[doc = "Field `URS` writer - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG, Urs>;
impl<'a, REG> UrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any of the following events generate an update interrupt or DMA request if enabled. These events can be:"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Urs::B0x0)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Urs::B0x1)
    }
}
#[doc = "One pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opm {
    #[doc = "0: Counter is not stopped at update event"]
    B0x0 = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the bit CEN)"]
    B0x1 = 1,
}
impl From<Opm> for bool {
    #[inline(always)]
    fn from(variant: Opm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPM` reader - One pulse mode"]
pub type OpmR = crate::BitReader<Opm>;
impl OpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opm {
        match self.bits {
            false => Opm::B0x0,
            true => Opm::B0x1,
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opm::B0x0
    }
    #[doc = "Counter stops counting at the next update event (clearing the bit CEN)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opm::B0x1
    }
}
#[doc = "Field `OPM` writer - One pulse mode"]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG, Opm>;
impl<'a, REG> OpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opm::B0x0)
    }
    #[doc = "Counter stops counting at the next update event (clearing the bit CEN)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opm::B0x1)
    }
}
#[doc = "Auto-reload preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arpe {
    #[doc = "0: TIMx_ARR register is not buffered"]
    B0x0 = 0,
    #[doc = "1: TIMx_ARR register is buffered"]
    B0x1 = 1,
}
impl From<Arpe> for bool {
    #[inline(always)]
    fn from(variant: Arpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ArpeR = crate::BitReader<Arpe>;
impl ArpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arpe {
        match self.bits {
            false => Arpe::B0x0,
            true => Arpe::B0x1,
        }
    }
    #[doc = "TIMx_ARR register is not buffered"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Arpe::B0x0
    }
    #[doc = "TIMx_ARR register is buffered"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Arpe::B0x1
    }
}
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ArpeW<'a, REG> = crate::BitWriter<'a, REG, Arpe>;
impl<'a, REG> ArpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx_ARR register is not buffered"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Arpe::B0x0)
    }
    #[doc = "TIMx_ARR register is buffered"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Arpe::B0x1)
    }
}
#[doc = "Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and the dead-time and sampling clock (tDTS)used by the dead-time generators and the digital filters (TIx),\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckd {
    #[doc = "0: tDTS=tCK_INT"]
    B0x0 = 0,
    #[doc = "1: tDTS=2*tCK_INT"]
    B0x1 = 1,
    #[doc = "2: tDTS=4*tCK_INT"]
    B0x2 = 2,
    #[doc = "3: Reserved, do not program this value"]
    B0x3 = 3,
}
impl From<Ckd> for u8 {
    #[inline(always)]
    fn from(variant: Ckd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckd {
    type Ux = u8;
}
impl crate::IsEnum for Ckd {}
#[doc = "Field `CKD` reader - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and the dead-time and sampling clock (tDTS)used by the dead-time generators and the digital filters (TIx),"]
pub type CkdR = crate::FieldReader<Ckd>;
impl CkdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckd {
        match self.bits {
            0 => Ckd::B0x0,
            1 => Ckd::B0x1,
            2 => Ckd::B0x2,
            3 => Ckd::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "tDTS=tCK_INT"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ckd::B0x0
    }
    #[doc = "tDTS=2*tCK_INT"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ckd::B0x1
    }
    #[doc = "tDTS=4*tCK_INT"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ckd::B0x2
    }
    #[doc = "Reserved, do not program this value"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ckd::B0x3
    }
}
#[doc = "Field `CKD` writer - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and the dead-time and sampling clock (tDTS)used by the dead-time generators and the digital filters (TIx),"]
pub type CkdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckd, crate::Safe>;
impl<'a, REG> CkdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tDTS=tCK_INT"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::B0x0)
    }
    #[doc = "tDTS=2*tCK_INT"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::B0x1)
    }
    #[doc = "tDTS=4*tCK_INT"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::B0x2)
    }
    #[doc = "Reserved, do not program this value"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::B0x3)
    }
}
#[doc = "UIF status bit remapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uifremap {
    #[doc = "0: No remapping. UIF status bit is not copied to TIMx_CNT register bit 31."]
    B0x0 = 0,
    #[doc = "1: Remapping enabled. UIF status bit is copied to TIMx_CNT register bit 31."]
    B0x1 = 1,
}
impl From<Uifremap> for bool {
    #[inline(always)]
    fn from(variant: Uifremap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UifremapR = crate::BitReader<Uifremap>;
impl UifremapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uifremap {
        match self.bits {
            false => Uifremap::B0x0,
            true => Uifremap::B0x1,
        }
    }
    #[doc = "No remapping. UIF status bit is not copied to TIMx_CNT register bit 31."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uifremap::B0x0
    }
    #[doc = "Remapping enabled. UIF status bit is copied to TIMx_CNT register bit 31."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uifremap::B0x1
    }
}
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UifremapW<'a, REG> = crate::BitWriter<'a, REG, Uifremap>;
impl<'a, REG> UifremapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No remapping. UIF status bit is not copied to TIMx_CNT register bit 31."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uifremap::B0x0)
    }
    #[doc = "Remapping enabled. UIF status bit is copied to TIMx_CNT register bit 31."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uifremap::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
    #[inline(always)]
    pub fn udis(&self) -> UdisR {
        UdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
    #[inline(always)]
    pub fn urs(&self) -> UrsR {
        UrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ArpeR {
        ArpeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and the dead-time and sampling clock (tDTS)used by the dead-time generators and the digital filters (TIx),"]
    #[inline(always)]
    pub fn ckd(&self) -> CkdR {
        CkdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&self) -> UifremapR {
        UifremapR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<Cr1Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UdisW<Cr1Spec> {
        UdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> UrsW<Cr1Spec> {
        UrsW::new(self, 2)
    }
    #[doc = "Bit 3 - One pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OpmW<Cr1Spec> {
        OpmW::new(self, 3)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ArpeW<Cr1Spec> {
        ArpeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and the dead-time and sampling clock (tDTS)used by the dead-time generators and the digital filters (TIx),"]
    #[inline(always)]
    #[must_use]
    pub fn ckd(&mut self) -> CkdW<Cr1Spec> {
        CkdW::new(self, 8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UifremapW<Cr1Spec> {
        UifremapW::new(self, 11)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
