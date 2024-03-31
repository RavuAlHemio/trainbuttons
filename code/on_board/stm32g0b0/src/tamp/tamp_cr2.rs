#[doc = "Register `TAMP_CR2` reader"]
pub type R = crate::R<TampCr2Spec>;
#[doc = "Register `TAMP_CR2` writer"]
pub type W = crate::W<TampCr2Spec>;
#[doc = "Tamper 1 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1noer {
    #[doc = "0: Tamper 1 event erases the backup registers."]
    B0x0 = 0,
    #[doc = "1: Tamper 1 event does not erase the backup registers."]
    B0x1 = 1,
}
impl From<Tamp1noer> for bool {
    #[inline(always)]
    fn from(variant: Tamp1noer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1NOER` reader - Tamper 1 no erase"]
pub type Tamp1noerR = crate::BitReader<Tamp1noer>;
impl Tamp1noerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1noer {
        match self.bits {
            false => Tamp1noer::B0x0,
            true => Tamp1noer::B0x1,
        }
    }
    #[doc = "Tamper 1 event erases the backup registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1noer::B0x0
    }
    #[doc = "Tamper 1 event does not erase the backup registers."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1noer::B0x1
    }
}
#[doc = "Field `TAMP1NOER` writer - Tamper 1 no erase"]
pub type Tamp1noerW<'a, REG> = crate::BitWriter<'a, REG, Tamp1noer>;
impl<'a, REG> Tamp1noerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 event erases the backup registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1noer::B0x0)
    }
    #[doc = "Tamper 1 event does not erase the backup registers."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1noer::B0x1)
    }
}
#[doc = "Tamper 2 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2noer {
    #[doc = "0: Tamper 2 event erases the backup registers."]
    B0x0 = 0,
    #[doc = "1: Tamper 2 event does not erase the backup registers."]
    B0x1 = 1,
}
impl From<Tamp2noer> for bool {
    #[inline(always)]
    fn from(variant: Tamp2noer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2NOER` reader - Tamper 2 no erase"]
pub type Tamp2noerR = crate::BitReader<Tamp2noer>;
impl Tamp2noerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2noer {
        match self.bits {
            false => Tamp2noer::B0x0,
            true => Tamp2noer::B0x1,
        }
    }
    #[doc = "Tamper 2 event erases the backup registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2noer::B0x0
    }
    #[doc = "Tamper 2 event does not erase the backup registers."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2noer::B0x1
    }
}
#[doc = "Field `TAMP2NOER` writer - Tamper 2 no erase"]
pub type Tamp2noerW<'a, REG> = crate::BitWriter<'a, REG, Tamp2noer>;
impl<'a, REG> Tamp2noerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 event erases the backup registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2noer::B0x0)
    }
    #[doc = "Tamper 2 event does not erase the backup registers."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2noer::B0x1)
    }
}
#[doc = "Tamper 3 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3noer {
    #[doc = "0: Tamper 3 event erases the backup registers."]
    B0x0 = 0,
    #[doc = "1: Tamper 3 event does not erase the backup registers."]
    B0x1 = 1,
}
impl From<Tamp3noer> for bool {
    #[inline(always)]
    fn from(variant: Tamp3noer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3NOER` reader - Tamper 3 no erase"]
pub type Tamp3noerR = crate::BitReader<Tamp3noer>;
impl Tamp3noerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3noer {
        match self.bits {
            false => Tamp3noer::B0x0,
            true => Tamp3noer::B0x1,
        }
    }
    #[doc = "Tamper 3 event erases the backup registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3noer::B0x0
    }
    #[doc = "Tamper 3 event does not erase the backup registers."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3noer::B0x1
    }
}
#[doc = "Field `TAMP3NOER` writer - Tamper 3 no erase"]
pub type Tamp3noerW<'a, REG> = crate::BitWriter<'a, REG, Tamp3noer>;
impl<'a, REG> Tamp3noerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 event erases the backup registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3noer::B0x0)
    }
    #[doc = "Tamper 3 event does not erase the backup registers."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3noer::B0x1)
    }
}
#[doc = "Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1msk {
    #[doc = "0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    B0x0 = 0,
    #[doc = "1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased."]
    B0x1 = 1,
}
impl From<Tamp1msk> for bool {
    #[inline(always)]
    fn from(variant: Tamp1msk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type Tamp1mskR = crate::BitReader<Tamp1msk>;
impl Tamp1mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1msk {
        match self.bits {
            false => Tamp1msk::B0x0,
            true => Tamp1msk::B0x1,
        }
    }
    #[doc = "Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1msk::B0x0
    }
    #[doc = "Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1msk::B0x1
    }
}
#[doc = "Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type Tamp1mskW<'a, REG> = crate::BitWriter<'a, REG, Tamp1msk>;
impl<'a, REG> Tamp1mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1msk::B0x0)
    }
    #[doc = "Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1msk::B0x1)
    }
}
#[doc = "Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2msk {
    #[doc = "0: Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    B0x0 = 0,
    #[doc = "1: Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased."]
    B0x1 = 1,
}
impl From<Tamp2msk> for bool {
    #[inline(always)]
    fn from(variant: Tamp2msk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type Tamp2mskR = crate::BitReader<Tamp2msk>;
impl Tamp2mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2msk {
        match self.bits {
            false => Tamp2msk::B0x0,
            true => Tamp2msk::B0x1,
        }
    }
    #[doc = "Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2msk::B0x0
    }
    #[doc = "Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2msk::B0x1
    }
}
#[doc = "Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type Tamp2mskW<'a, REG> = crate::BitWriter<'a, REG, Tamp2msk>;
impl<'a, REG> Tamp2mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2msk::B0x0)
    }
    #[doc = "Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2msk::B0x1)
    }
}
#[doc = "Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3msk {
    #[doc = "0: Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    B0x0 = 0,
    #[doc = "1: Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers are not erased."]
    B0x1 = 1,
}
impl From<Tamp3msk> for bool {
    #[inline(always)]
    fn from(variant: Tamp3msk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3MSK` reader - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
pub type Tamp3mskR = crate::BitReader<Tamp3msk>;
impl Tamp3mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3msk {
        match self.bits {
            false => Tamp3msk::B0x0,
            true => Tamp3msk::B0x1,
        }
    }
    #[doc = "Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3msk::B0x0
    }
    #[doc = "Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3msk::B0x1
    }
}
#[doc = "Field `TAMP3MSK` writer - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
pub type Tamp3mskW<'a, REG> = crate::BitWriter<'a, REG, Tamp3msk>;
impl<'a, REG> Tamp3mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3msk::B0x0)
    }
    #[doc = "Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3msk::B0x1)
    }
}
#[doc = "Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1trg {
    #[doc = "0: If TAMPFLT different 00 Tamper 1 input staying low triggers a tamper detection event."]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT = 00 Tamper 1 input staying high triggers a tamper detection event."]
    B0x1 = 1,
}
impl From<Tamp1trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp1trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type Tamp1trgR = crate::BitReader<Tamp1trg>;
impl Tamp1trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1trg {
        match self.bits {
            false => Tamp1trg::B0x0,
            true => Tamp1trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different 00 Tamper 1 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1trg::B0x0
    }
    #[doc = "If TAMPFLT = 00 Tamper 1 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1trg::B0x1
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type Tamp1trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp1trg>;
impl<'a, REG> Tamp1trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different 00 Tamper 1 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1trg::B0x0)
    }
    #[doc = "If TAMPFLT = 00 Tamper 1 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1trg::B0x1)
    }
}
#[doc = "Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2trg {
    #[doc = "0: If TAMPFLT different 00 Tamper 2 input staying low triggers a tamper detection event."]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT = 00 Tamper 2 input staying high triggers a tamper detection event."]
    B0x1 = 1,
}
impl From<Tamp2trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp2trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type Tamp2trgR = crate::BitReader<Tamp2trg>;
impl Tamp2trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2trg {
        match self.bits {
            false => Tamp2trg::B0x0,
            true => Tamp2trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different 00 Tamper 2 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2trg::B0x0
    }
    #[doc = "If TAMPFLT = 00 Tamper 2 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2trg::B0x1
    }
}
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type Tamp2trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp2trg>;
impl<'a, REG> Tamp2trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different 00 Tamper 2 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2trg::B0x0)
    }
    #[doc = "If TAMPFLT = 00 Tamper 2 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2trg::B0x1)
    }
}
#[doc = "Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3trg {
    #[doc = "0: If TAMPFLT different 00 Tamper 3 input staying low triggers a tamper detection event."]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT =00 Tamper 3 input staying high triggers a tamper detection event."]
    B0x1 = 1,
}
impl From<Tamp3trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp3trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
pub type Tamp3trgR = crate::BitReader<Tamp3trg>;
impl Tamp3trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3trg {
        match self.bits {
            false => Tamp3trg::B0x0,
            true => Tamp3trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different 00 Tamper 3 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3trg::B0x0
    }
    #[doc = "If TAMPFLT =00 Tamper 3 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3trg::B0x1
    }
}
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
pub type Tamp3trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp3trg>;
impl<'a, REG> Tamp3trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different 00 Tamper 3 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3trg::B0x0)
    }
    #[doc = "If TAMPFLT =00 Tamper 3 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3trg::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> Tamp1noerR {
        Tamp1noerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> Tamp2noerR {
        Tamp2noerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> Tamp3noerR {
        Tamp3noerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    pub fn tamp1msk(&self) -> Tamp1mskR {
        Tamp1mskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    pub fn tamp2msk(&self) -> Tamp2mskR {
        Tamp2mskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
    #[inline(always)]
    pub fn tamp3msk(&self) -> Tamp3mskR {
        Tamp3mskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&self) -> Tamp1trgR {
        Tamp1trgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp2trg(&self) -> Tamp2trgR {
        Tamp2trgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp3trg(&self) -> Tamp3trgR {
        Tamp3trgR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> Tamp1noerW<TampCr2Spec> {
        Tamp1noerW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> Tamp2noerW<TampCr2Spec> {
        Tamp2noerW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noer(&mut self) -> Tamp3noerW<TampCr2Spec> {
        Tamp3noerW::new(self, 2)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> Tamp1mskW<TampCr2Spec> {
        Tamp1mskW::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> Tamp2mskW<TampCr2Spec> {
        Tamp2mskW::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> Tamp3mskW<TampCr2Spec> {
        Tamp3mskW::new(self, 18)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> Tamp1trgW<TampCr2Spec> {
        Tamp1trgW::new(self, 24)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> Tamp2trgW<TampCr2Spec> {
        Tamp2trgW::new(self, 25)
    }
    #[doc = "Bit 26 - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> Tamp3trgW<TampCr2Spec> {
        Tamp3trgW::new(self, 26)
    }
}
#[doc = "TAMP control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampCr2Spec;
impl crate::RegisterSpec for TampCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_cr2::R`](R) reader structure"]
impl crate::Readable for TampCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tamp_cr2::W`](W) writer structure"]
impl crate::Writable for TampCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_CR2 to value 0"]
impl crate::Resettable for TampCr2Spec {
    const RESET_VALUE: u32 = 0;
}
