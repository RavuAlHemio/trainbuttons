#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uie {
    #[doc = "0: Update interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Update interrupt enabled"]
    B0x1 = 1,
}
impl From<Uie> for bool {
    #[inline(always)]
    fn from(variant: Uie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UieR = crate::BitReader<Uie>;
impl UieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uie {
        match self.bits {
            false => Uie::B0x0,
            true => Uie::B0x1,
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uie::B0x0
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uie::B0x1
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG, Uie>;
impl<'a, REG> UieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uie::B0x0)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uie::B0x1)
    }
}
#[doc = "Capture/Compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1ie {
    #[doc = "0: CC1 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CC1 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc1ie> for bool {
    #[inline(always)]
    fn from(variant: Cc1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"]
pub type Cc1ieR = crate::BitReader<Cc1ie>;
impl Cc1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1ie {
        match self.bits {
            false => Cc1ie::B0x0,
            true => Cc1ie::B0x1,
        }
    }
    #[doc = "CC1 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1ie::B0x0
    }
    #[doc = "CC1 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1ie::B0x1
    }
}
#[doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG, Cc1ie>;
impl<'a, REG> Cc1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ie::B0x0)
    }
    #[doc = "CC1 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ie::B0x1)
    }
}
#[doc = "Capture/Compare 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2ie {
    #[doc = "0: CC2 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CC2 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc2ie> for bool {
    #[inline(always)]
    fn from(variant: Cc2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2IE` reader - Capture/Compare 2 interrupt enable"]
pub type Cc2ieR = crate::BitReader<Cc2ie>;
impl Cc2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2ie {
        match self.bits {
            false => Cc2ie::B0x0,
            true => Cc2ie::B0x1,
        }
    }
    #[doc = "CC2 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2ie::B0x0
    }
    #[doc = "CC2 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2ie::B0x1
    }
}
#[doc = "Field `CC2IE` writer - Capture/Compare 2 interrupt enable"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG, Cc2ie>;
impl<'a, REG> Cc2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC2 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2ie::B0x0)
    }
    #[doc = "CC2 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2ie::B0x1)
    }
}
#[doc = "COM interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comie {
    #[doc = "0: COM interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: COM interrupt enabled"]
    B0x1 = 1,
}
impl From<Comie> for bool {
    #[inline(always)]
    fn from(variant: Comie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type ComieR = crate::BitReader<Comie>;
impl ComieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comie {
        match self.bits {
            false => Comie::B0x0,
            true => Comie::B0x1,
        }
    }
    #[doc = "COM interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Comie::B0x0
    }
    #[doc = "COM interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Comie::B0x1
    }
}
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG, Comie>;
impl<'a, REG> ComieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COM interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Comie::B0x0)
    }
    #[doc = "COM interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Comie::B0x1)
    }
}
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: Trigger interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Trigger interrupt enabled"]
    B0x1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::B0x0,
            true => Tie::B0x1,
        }
    }
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tie::B0x0
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tie::B0x1
    }
}
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0x0)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0x1)
    }
}
#[doc = "Break interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bie {
    #[doc = "0: Break interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Break interrupt enabled"]
    B0x1 = 1,
}
impl From<Bie> for bool {
    #[inline(always)]
    fn from(variant: Bie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BieR = crate::BitReader<Bie>;
impl BieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bie {
        match self.bits {
            false => Bie::B0x0,
            true => Bie::B0x1,
        }
    }
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bie::B0x0
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bie::B0x1
    }
}
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG, Bie>;
impl<'a, REG> BieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bie::B0x0)
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bie::B0x1)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ude {
    #[doc = "0: Update DMA request disabled"]
    B0x0 = 0,
    #[doc = "1: Update DMA request enabled"]
    B0x1 = 1,
}
impl From<Ude> for bool {
    #[inline(always)]
    fn from(variant: Ude) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UdeR = crate::BitReader<Ude>;
impl UdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ude {
        match self.bits {
            false => Ude::B0x0,
            true => Ude::B0x1,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ude::B0x0
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ude::B0x1
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG, Ude>;
impl<'a, REG> UdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ude::B0x0)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ude::B0x1)
    }
}
#[doc = "Capture/Compare 1 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1de {
    #[doc = "0: CC1 DMA request disabled"]
    B0x0 = 0,
    #[doc = "1: CC1 DMA request enabled"]
    B0x1 = 1,
}
impl From<Cc1de> for bool {
    #[inline(always)]
    fn from(variant: Cc1de) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1DE` reader - Capture/Compare 1 DMA request enable"]
pub type Cc1deR = crate::BitReader<Cc1de>;
impl Cc1deR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1de {
        match self.bits {
            false => Cc1de::B0x0,
            true => Cc1de::B0x1,
        }
    }
    #[doc = "CC1 DMA request disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1de::B0x0
    }
    #[doc = "CC1 DMA request enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1de::B0x1
    }
}
#[doc = "Field `CC1DE` writer - Capture/Compare 1 DMA request enable"]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG, Cc1de>;
impl<'a, REG> Cc1deW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1de::B0x0)
    }
    #[doc = "CC1 DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1de::B0x1)
    }
}
#[doc = "Capture/Compare 2 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2de {
    #[doc = "0: CC2 DMA request disabled"]
    B0x0 = 0,
    #[doc = "1: CC2 DMA request enabled"]
    B0x1 = 1,
}
impl From<Cc2de> for bool {
    #[inline(always)]
    fn from(variant: Cc2de) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2DE` reader - Capture/Compare 2 DMA request enable"]
pub type Cc2deR = crate::BitReader<Cc2de>;
impl Cc2deR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2de {
        match self.bits {
            false => Cc2de::B0x0,
            true => Cc2de::B0x1,
        }
    }
    #[doc = "CC2 DMA request disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2de::B0x0
    }
    #[doc = "CC2 DMA request enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2de::B0x1
    }
}
#[doc = "Field `CC2DE` writer - Capture/Compare 2 DMA request enable"]
pub type Cc2deW<'a, REG> = crate::BitWriter<'a, REG, Cc2de>;
impl<'a, REG> Cc2deW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC2 DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2de::B0x0)
    }
    #[doc = "CC2 DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2de::B0x1)
    }
}
#[doc = "COM DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comde {
    #[doc = "0: COM DMA request disabled"]
    B0x0 = 0,
    #[doc = "1: COM DMA request enabled"]
    B0x1 = 1,
}
impl From<Comde> for bool {
    #[inline(always)]
    fn from(variant: Comde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMDE` reader - COM DMA request enable"]
pub type ComdeR = crate::BitReader<Comde>;
impl ComdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comde {
        match self.bits {
            false => Comde::B0x0,
            true => Comde::B0x1,
        }
    }
    #[doc = "COM DMA request disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Comde::B0x0
    }
    #[doc = "COM DMA request enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Comde::B0x1
    }
}
#[doc = "Field `COMDE` writer - COM DMA request enable"]
pub type ComdeW<'a, REG> = crate::BitWriter<'a, REG, Comde>;
impl<'a, REG> ComdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COM DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Comde::B0x0)
    }
    #[doc = "COM DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Comde::B0x1)
    }
}
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tde {
    #[doc = "0: Trigger DMA request disabled"]
    B0x0 = 0,
    #[doc = "1: Trigger DMA request enabled"]
    B0x1 = 1,
}
impl From<Tde> for bool {
    #[inline(always)]
    fn from(variant: Tde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Trigger DMA request enable"]
pub type TdeR = crate::BitReader<Tde>;
impl TdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tde {
        match self.bits {
            false => Tde::B0x0,
            true => Tde::B0x1,
        }
    }
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tde::B0x0
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tde::B0x1
    }
}
#[doc = "Field `TDE` writer - Trigger DMA request enable"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG, Tde>;
impl<'a, REG> TdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::B0x0)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> Cc2deR {
        Cc2deR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> ComdeR {
        ComdeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UieW<DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> Cc1ieW<DierSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> Cc2ieW<DierSpec> {
        Cc2ieW::new(self, 2)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> ComieW<DierSpec> {
        ComieW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BieW<DierSpec> {
        BieW::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UdeW<DierSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> Cc1deW<DierSpec> {
        Cc1deW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> Cc2deW<DierSpec> {
        Cc2deW::new(self, 10)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn comde(&mut self) -> ComdeW<DierSpec> {
        ComdeW::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TdeW<DierSpec> {
        TdeW::new(self, 14)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {
    const RESET_VALUE: u32 = 0;
}
