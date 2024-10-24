#[doc = "Register `TAMP_CR1` reader"]
pub type R = crate::R<TampCr1Spec>;
#[doc = "Register `TAMP_CR1` writer"]
pub type W = crate::W<TampCr1Spec>;
#[doc = "Tamper detection on TAMP_IN1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1e {
    #[doc = "0: Tamper detection on TAMP_IN1 is disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN1 is enabled."]
    B0x1 = 1,
}
impl From<Tamp1e> for bool {
    #[inline(always)]
    fn from(variant: Tamp1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable"]
pub type Tamp1eR = crate::BitReader<Tamp1e>;
impl Tamp1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1e {
        match self.bits {
            false => Tamp1e::B0x0,
            true => Tamp1e::B0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN1 is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1e::B0x0
    }
    #[doc = "Tamper detection on TAMP_IN1 is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1e::B0x1
    }
}
#[doc = "Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable"]
pub type Tamp1eW<'a, REG> = crate::BitWriter<'a, REG, Tamp1e>;
impl<'a, REG> Tamp1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN1 is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1e::B0x0)
    }
    #[doc = "Tamper detection on TAMP_IN1 is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1e::B0x1)
    }
}
#[doc = "Tamper detection on TAMP_IN2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2e {
    #[doc = "0: Tamper detection on TAMP_IN2 is disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN2 is enabled."]
    B0x1 = 1,
}
impl From<Tamp2e> for bool {
    #[inline(always)]
    fn from(variant: Tamp2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable"]
pub type Tamp2eR = crate::BitReader<Tamp2e>;
impl Tamp2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2e {
        match self.bits {
            false => Tamp2e::B0x0,
            true => Tamp2e::B0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN2 is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2e::B0x0
    }
    #[doc = "Tamper detection on TAMP_IN2 is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2e::B0x1
    }
}
#[doc = "Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable"]
pub type Tamp2eW<'a, REG> = crate::BitWriter<'a, REG, Tamp2e>;
impl<'a, REG> Tamp2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN2 is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2e::B0x0)
    }
    #[doc = "Tamper detection on TAMP_IN2 is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2e::B0x1)
    }
}
#[doc = "Tamper detection on TAMP_IN3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3e {
    #[doc = "0: Tamper detection on TAMP_IN3 is disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN3 is enabled."]
    B0x1 = 1,
}
impl From<Tamp3e> for bool {
    #[inline(always)]
    fn from(variant: Tamp3e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3E` reader - Tamper detection on TAMP_IN3 enable"]
pub type Tamp3eR = crate::BitReader<Tamp3e>;
impl Tamp3eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3e {
        match self.bits {
            false => Tamp3e::B0x0,
            true => Tamp3e::B0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN3 is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3e::B0x0
    }
    #[doc = "Tamper detection on TAMP_IN3 is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3e::B0x1
    }
}
#[doc = "Field `TAMP3E` writer - Tamper detection on TAMP_IN3 enable"]
pub type Tamp3eW<'a, REG> = crate::BitWriter<'a, REG, Tamp3e>;
impl<'a, REG> Tamp3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN3 is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3e::B0x0)
    }
    #[doc = "Tamper detection on TAMP_IN3 is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3e::B0x1)
    }
}
#[doc = "Internal tamper 3 enable: LSE monitoring\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp3e {
    #[doc = "0: Internal tamper 3 disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds."]
    B0x1 = 1,
}
impl From<Itamp3e> for bool {
    #[inline(always)]
    fn from(variant: Itamp3e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3E` reader - Internal tamper 3 enable: LSE monitoring"]
pub type Itamp3eR = crate::BitReader<Itamp3e>;
impl Itamp3eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp3e {
        match self.bits {
            false => Itamp3e::B0x0,
            true => Itamp3e::B0x1,
        }
    }
    #[doc = "Internal tamper 3 disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp3e::B0x0
    }
    #[doc = "Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp3e::B0x1
    }
}
#[doc = "Field `ITAMP3E` writer - Internal tamper 3 enable: LSE monitoring"]
pub type Itamp3eW<'a, REG> = crate::BitWriter<'a, REG, Itamp3e>;
impl<'a, REG> Itamp3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp3e::B0x0)
    }
    #[doc = "Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp3e::B0x1)
    }
}
#[doc = "Internal tamper 4 enable: HSE monitoring\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp4e {
    #[doc = "0: Internal tamper 4 disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds."]
    B0x1 = 1,
}
impl From<Itamp4e> for bool {
    #[inline(always)]
    fn from(variant: Itamp4e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4E` reader - Internal tamper 4 enable: HSE monitoring"]
pub type Itamp4eR = crate::BitReader<Itamp4e>;
impl Itamp4eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp4e {
        match self.bits {
            false => Itamp4e::B0x0,
            true => Itamp4e::B0x1,
        }
    }
    #[doc = "Internal tamper 4 disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp4e::B0x0
    }
    #[doc = "Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp4e::B0x1
    }
}
#[doc = "Field `ITAMP4E` writer - Internal tamper 4 enable: HSE monitoring"]
pub type Itamp4eW<'a, REG> = crate::BitWriter<'a, REG, Itamp4e>;
impl<'a, REG> Itamp4eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp4e::B0x0)
    }
    #[doc = "Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp4e::B0x1)
    }
}
#[doc = "Internal tamper 5 enable: RTC calendar overflow\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp5e {
    #[doc = "0: Internal tamper 5 disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow."]
    B0x1 = 1,
}
impl From<Itamp5e> for bool {
    #[inline(always)]
    fn from(variant: Itamp5e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5E` reader - Internal tamper 5 enable: RTC calendar overflow"]
pub type Itamp5eR = crate::BitReader<Itamp5e>;
impl Itamp5eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp5e {
        match self.bits {
            false => Itamp5e::B0x0,
            true => Itamp5e::B0x1,
        }
    }
    #[doc = "Internal tamper 5 disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp5e::B0x0
    }
    #[doc = "Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp5e::B0x1
    }
}
#[doc = "Field `ITAMP5E` writer - Internal tamper 5 enable: RTC calendar overflow"]
pub type Itamp5eW<'a, REG> = crate::BitWriter<'a, REG, Itamp5e>;
impl<'a, REG> Itamp5eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp5e::B0x0)
    }
    #[doc = "Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp5e::B0x1)
    }
}
#[doc = "Internal tamper 6 enable: ST manufacturer readout\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp6e {
    #[doc = "0: Internal tamper 6 disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout."]
    B0x1 = 1,
}
impl From<Itamp6e> for bool {
    #[inline(always)]
    fn from(variant: Itamp6e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6E` reader - Internal tamper 6 enable: ST manufacturer readout"]
pub type Itamp6eR = crate::BitReader<Itamp6e>;
impl Itamp6eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp6e {
        match self.bits {
            false => Itamp6e::B0x0,
            true => Itamp6e::B0x1,
        }
    }
    #[doc = "Internal tamper 6 disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp6e::B0x0
    }
    #[doc = "Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp6e::B0x1
    }
}
#[doc = "Field `ITAMP6E` writer - Internal tamper 6 enable: ST manufacturer readout"]
pub type Itamp6eW<'a, REG> = crate::BitWriter<'a, REG, Itamp6e>;
impl<'a, REG> Itamp6eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp6e::B0x0)
    }
    #[doc = "Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp6e::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> Tamp1eR {
        Tamp1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> Tamp2eR {
        Tamp2eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection on TAMP_IN3 enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> Tamp3eR {
        Tamp3eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable: LSE monitoring"]
    #[inline(always)]
    pub fn itamp3e(&self) -> Itamp3eR {
        Itamp3eR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable: HSE monitoring"]
    #[inline(always)]
    pub fn itamp4e(&self) -> Itamp4eR {
        Itamp4eR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable: RTC calendar overflow"]
    #[inline(always)]
    pub fn itamp5e(&self) -> Itamp5eR {
        Itamp5eR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable: ST manufacturer readout"]
    #[inline(always)]
    pub fn itamp6e(&self) -> Itamp6eR {
        Itamp6eR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> Tamp1eW<TampCr1Spec> {
        Tamp1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> Tamp2eW<TampCr1Spec> {
        Tamp2eW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper detection on TAMP_IN3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3e(&mut self) -> Tamp3eW<TampCr1Spec> {
        Tamp3eW::new(self, 2)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable: LSE monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3e(&mut self) -> Itamp3eW<TampCr1Spec> {
        Itamp3eW::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable: HSE monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4e(&mut self) -> Itamp4eW<TampCr1Spec> {
        Itamp4eW::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable: RTC calendar overflow"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5e(&mut self) -> Itamp5eW<TampCr1Spec> {
        Itamp5eW::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable: ST manufacturer readout"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6e(&mut self) -> Itamp6eW<TampCr1Spec> {
        Itamp6eW::new(self, 21)
    }
}
#[doc = "TAMP control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tamp_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampCr1Spec;
impl crate::RegisterSpec for TampCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_cr1::R`](R) reader structure"]
impl crate::Readable for TampCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tamp_cr1::W`](W) writer structure"]
impl crate::Writable for TampCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_CR1 to value 0xffff_0000"]
impl crate::Resettable for TampCr1Spec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
