#[doc = "Register `I2C_CR1` reader"]
pub type R = crate::R<I2cCr1Spec>;
#[doc = "Register `I2C_CR1` writer"]
pub type W = crate::W<I2cCr1Spec>;
#[doc = "Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Peripheral disable"]
    B0x0 = 0,
    #[doc = "1: Peripheral enable"]
    B0x1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::B0x0,
            true => Pe::B0x1,
        }
    }
    #[doc = "Peripheral disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pe::B0x0
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pe::B0x1
    }
}
#[doc = "Field `PE` writer - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0x0)
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0x1)
    }
}
#[doc = "TX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txie {
    #[doc = "0: Transmit (TXIS) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Transmit (TXIS) interrupt enabled"]
    B0x1 = 1,
}
impl From<Txie> for bool {
    #[inline(always)]
    fn from(variant: Txie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - TX Interrupt enable"]
pub type TxieR = crate::BitReader<Txie>;
impl TxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txie {
        match self.bits {
            false => Txie::B0x0,
            true => Txie::B0x1,
        }
    }
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txie::B0x0
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txie::B0x1
    }
}
#[doc = "Field `TXIE` writer - TX Interrupt enable"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG, Txie>;
impl<'a, REG> TxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::B0x0)
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::B0x1)
    }
}
#[doc = "RX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxie {
    #[doc = "0: Receive (RXNE) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Receive (RXNE) interrupt enabled"]
    B0x1 = 1,
}
impl From<Rxie> for bool {
    #[inline(always)]
    fn from(variant: Rxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - RX Interrupt enable"]
pub type RxieR = crate::BitReader<Rxie>;
impl RxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxie {
        match self.bits {
            false => Rxie::B0x0,
            true => Rxie::B0x1,
        }
    }
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxie::B0x0
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxie::B0x1
    }
}
#[doc = "Field `RXIE` writer - RX Interrupt enable"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG, Rxie>;
impl<'a, REG> RxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::B0x0)
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::B0x1)
    }
}
#[doc = "Address match Interrupt enable (slave only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrie {
    #[doc = "0: Address match (ADDR) interrupts disabled"]
    B0x0 = 0,
    #[doc = "1: Address match (ADDR) interrupts enabled"]
    B0x1 = 1,
}
impl From<Addrie> for bool {
    #[inline(always)]
    fn from(variant: Addrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIE` reader - Address match Interrupt enable (slave only)"]
pub type AddrieR = crate::BitReader<Addrie>;
impl AddrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrie {
        match self.bits {
            false => Addrie::B0x0,
            true => Addrie::B0x1,
        }
    }
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Addrie::B0x0
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Addrie::B0x1
    }
}
#[doc = "Field `ADDRIE` writer - Address match Interrupt enable (slave only)"]
pub type AddrieW<'a, REG> = crate::BitWriter<'a, REG, Addrie>;
impl<'a, REG> AddrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Addrie::B0x0)
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Addrie::B0x1)
    }
}
#[doc = "Not acknowledge received Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nackie {
    #[doc = "0: Not acknowledge (NACKF) received interrupts disabled"]
    B0x0 = 0,
    #[doc = "1: Not acknowledge (NACKF) received interrupts enabled"]
    B0x1 = 1,
}
impl From<Nackie> for bool {
    #[inline(always)]
    fn from(variant: Nackie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIE` reader - Not acknowledge received Interrupt enable"]
pub type NackieR = crate::BitReader<Nackie>;
impl NackieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nackie {
        match self.bits {
            false => Nackie::B0x0,
            true => Nackie::B0x1,
        }
    }
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nackie::B0x0
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nackie::B0x1
    }
}
#[doc = "Field `NACKIE` writer - Not acknowledge received Interrupt enable"]
pub type NackieW<'a, REG> = crate::BitWriter<'a, REG, Nackie>;
impl<'a, REG> NackieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nackie::B0x0)
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nackie::B0x1)
    }
}
#[doc = "Stop detection Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopie {
    #[doc = "0: Stop detection (STOPF) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Stop detection (STOPF) interrupt enabled"]
    B0x1 = 1,
}
impl From<Stopie> for bool {
    #[inline(always)]
    fn from(variant: Stopie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIE` reader - Stop detection Interrupt enable"]
pub type StopieR = crate::BitReader<Stopie>;
impl StopieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopie {
        match self.bits {
            false => Stopie::B0x0,
            true => Stopie::B0x1,
        }
    }
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stopie::B0x0
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stopie::B0x1
    }
}
#[doc = "Field `STOPIE` writer - Stop detection Interrupt enable"]
pub type StopieW<'a, REG> = crate::BitWriter<'a, REG, Stopie>;
impl<'a, REG> StopieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopie::B0x0)
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopie::B0x1)
    }
}
#[doc = "Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Transfer Complete interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Transfer Complete interrupt enabled"]
    B0x1 = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::B0x0,
            true => Tcie::B0x1,
        }
    }
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcie::B0x0
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcie::B0x1
    }
}
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x0)
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x1)
    }
}
#[doc = "Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error detection interrupts disabled"]
    B0x0 = 0,
    #[doc = "1: Error detection interrupts enabled"]
    B0x1 = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::B0x0,
            true => Errie::B0x1,
        }
    }
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errie::B0x0
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errie::B0x1
    }
}
#[doc = "Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x0)
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x1)
    }
}
#[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dnf {
    #[doc = "0: Digital filter disabled"]
    B0x0 = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    B0x1 = 1,
    #[doc = "15: digital filter enabled and filtering capability up to15 tI2CCLK"]
    B0xF = 15,
}
impl From<Dnf> for u8 {
    #[inline(always)]
    fn from(variant: Dnf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dnf {
    type Ux = u8;
}
impl crate::IsEnum for Dnf {}
#[doc = "Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DnfR = crate::FieldReader<Dnf>;
impl DnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dnf> {
        match self.bits {
            0 => Some(Dnf::B0x0),
            1 => Some(Dnf::B0x1),
            15 => Some(Dnf::B0xF),
            _ => None,
        }
    }
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dnf::B0x0
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dnf::B0x1
    }
    #[doc = "digital filter enabled and filtering capability up to15 tI2CCLK"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Dnf::B0xF
    }
}
#[doc = "Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dnf>;
impl<'a, REG> DnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::B0x0)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::B0x1)
    }
    #[doc = "digital filter enabled and filtering capability up to15 tI2CCLK"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::B0xF)
    }
}
#[doc = "Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anfoff {
    #[doc = "0: Analog noise filter enabled"]
    B0x0 = 0,
    #[doc = "1: Analog noise filter disabled"]
    B0x1 = 1,
}
impl From<Anfoff> for bool {
    #[inline(always)]
    fn from(variant: Anfoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type AnfoffR = crate::BitReader<Anfoff>;
impl AnfoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anfoff {
        match self.bits {
            false => Anfoff::B0x0,
            true => Anfoff::B0x1,
        }
    }
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Anfoff::B0x0
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Anfoff::B0x1
    }
}
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type AnfoffW<'a, REG> = crate::BitWriter<'a, REG, Anfoff>;
impl<'a, REG> AnfoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfoff::B0x0)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfoff::B0x1)
    }
}
#[doc = "DMA transmission requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmaen {
    #[doc = "0: DMA mode disabled for transmission"]
    B0x0 = 0,
    #[doc = "1: DMA mode enabled for transmission"]
    B0x1 = 1,
}
impl From<Txdmaen> for bool {
    #[inline(always)]
    fn from(variant: Txdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TxdmaenR = crate::BitReader<Txdmaen>;
impl TxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmaen {
        match self.bits {
            false => Txdmaen::B0x0,
            true => Txdmaen::B0x1,
        }
    }
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txdmaen::B0x0
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txdmaen::B0x1
    }
}
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Txdmaen>;
impl<'a, REG> TxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::B0x0)
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::B0x1)
    }
}
#[doc = "DMA reception requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmaen {
    #[doc = "0: DMA mode disabled for reception"]
    B0x0 = 0,
    #[doc = "1: DMA mode enabled for reception"]
    B0x1 = 1,
}
impl From<Rxdmaen> for bool {
    #[inline(always)]
    fn from(variant: Rxdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RxdmaenR = crate::BitReader<Rxdmaen>;
impl RxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmaen {
        match self.bits {
            false => Rxdmaen::B0x0,
            true => Rxdmaen::B0x1,
        }
    }
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxdmaen::B0x0
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxdmaen::B0x1
    }
}
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Rxdmaen>;
impl<'a, REG> RxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::B0x0)
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::B0x1)
    }
}
#[doc = "Slave byte control This bit is used to enable hardware byte control in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbc {
    #[doc = "0: Slave byte control disabled"]
    B0x0 = 0,
    #[doc = "1: Slave byte control enabled"]
    B0x1 = 1,
}
impl From<Sbc> for bool {
    #[inline(always)]
    fn from(variant: Sbc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SbcR = crate::BitReader<Sbc>;
impl SbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbc {
        match self.bits {
            false => Sbc::B0x0,
            true => Sbc::B0x1,
        }
    }
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sbc::B0x0
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sbc::B0x1
    }
}
#[doc = "Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SbcW<'a, REG> = crate::BitWriter<'a, REG, Sbc>;
impl<'a, REG> SbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbc::B0x0)
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbc::B0x1)
    }
}
#[doc = "Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nostretch {
    #[doc = "0: Clock stretching enabled"]
    B0x0 = 0,
    #[doc = "1: Clock stretching disabled"]
    B0x1 = 1,
}
impl From<Nostretch> for bool {
    #[inline(always)]
    fn from(variant: Nostretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NostretchR = crate::BitReader<Nostretch>;
impl NostretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nostretch {
        match self.bits {
            false => Nostretch::B0x0,
            true => Nostretch::B0x1,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nostretch::B0x0
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nostretch::B0x1
    }
}
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NostretchW<'a, REG> = crate::BitWriter<'a, REG, Nostretch>;
impl<'a, REG> NostretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nostretch::B0x0)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nostretch::B0x1)
    }
}
#[doc = "Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupen {
    #[doc = "0: Wakeup from Stop mode disable."]
    B0x0 = 0,
    #[doc = "1: Wakeup from Stop mode enable."]
    B0x1 = 1,
}
impl From<Wupen> for bool {
    #[inline(always)]
    fn from(variant: Wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN` reader - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
pub type WupenR = crate::BitReader<Wupen>;
impl WupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupen {
        match self.bits {
            false => Wupen::B0x0,
            true => Wupen::B0x1,
        }
    }
    #[doc = "Wakeup from Stop mode disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wupen::B0x0
    }
    #[doc = "Wakeup from Stop mode enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wupen::B0x1
    }
}
#[doc = "Field `WUPEN` writer - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG, Wupen>;
impl<'a, REG> WupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup from Stop mode disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen::B0x0)
    }
    #[doc = "Wakeup from Stop mode enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen::B0x1)
    }
}
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcen {
    #[doc = "0: General call disabled. Address 0b00000000 is NACKed."]
    B0x0 = 0,
    #[doc = "1: General call enabled. Address 0b00000000 is ACKed."]
    B0x1 = 1,
}
impl From<Gcen> for bool {
    #[inline(always)]
    fn from(variant: Gcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GcenR = crate::BitReader<Gcen>;
impl GcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcen {
        match self.bits {
            false => Gcen::B0x0,
            true => Gcen::B0x1,
        }
    }
    #[doc = "General call disabled. Address 0b00000000 is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gcen::B0x0
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gcen::B0x1
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG, Gcen>;
impl<'a, REG> GcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call disabled. Address 0b00000000 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::B0x0)
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::B0x1)
    }
}
#[doc = "SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbhen {
    #[doc = "0: Host Address disabled. Address 0b0001000x is NACKed."]
    B0x0 = 0,
    #[doc = "1: Host Address enabled. Address 0b0001000x is ACKed."]
    B0x1 = 1,
}
impl From<Smbhen> for bool {
    #[inline(always)]
    fn from(variant: Smbhen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBHEN` reader - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SmbhenR = crate::BitReader<Smbhen>;
impl SmbhenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smbhen {
        match self.bits {
            false => Smbhen::B0x0,
            true => Smbhen::B0x1,
        }
    }
    #[doc = "Host Address disabled. Address 0b0001000x is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smbhen::B0x0
    }
    #[doc = "Host Address enabled. Address 0b0001000x is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smbhen::B0x1
    }
}
#[doc = "Field `SMBHEN` writer - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SmbhenW<'a, REG> = crate::BitWriter<'a, REG, Smbhen>;
impl<'a, REG> SmbhenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host Address disabled. Address 0b0001000x is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smbhen::B0x0)
    }
    #[doc = "Host Address enabled. Address 0b0001000x is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smbhen::B0x1)
    }
}
#[doc = "SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbden {
    #[doc = "0: Device Default Address disabled. Address 0b1100001x is NACKed."]
    B0x0 = 0,
    #[doc = "1: Device Default Address enabled. Address 0b1100001x is ACKed."]
    B0x1 = 1,
}
impl From<Smbden> for bool {
    #[inline(always)]
    fn from(variant: Smbden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBDEN` reader - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SmbdenR = crate::BitReader<Smbden>;
impl SmbdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smbden {
        match self.bits {
            false => Smbden::B0x0,
            true => Smbden::B0x1,
        }
    }
    #[doc = "Device Default Address disabled. Address 0b1100001x is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Smbden::B0x0
    }
    #[doc = "Device Default Address enabled. Address 0b1100001x is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Smbden::B0x1
    }
}
#[doc = "Field `SMBDEN` writer - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SmbdenW<'a, REG> = crate::BitWriter<'a, REG, Smbden>;
impl<'a, REG> SmbdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device Default Address disabled. Address 0b1100001x is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Smbden::B0x0)
    }
    #[doc = "Device Default Address enabled. Address 0b1100001x is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Smbden::B0x1)
    }
}
#[doc = "SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alerten {
    #[doc = "0: The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK)."]
    B0x0 = 0,
    #[doc = "1: The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK)."]
    B0x1 = 1,
}
impl From<Alerten> for bool {
    #[inline(always)]
    fn from(variant: Alerten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTEN` reader - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type AlertenR = crate::BitReader<Alerten>;
impl AlertenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alerten {
        match self.bits {
            false => Alerten::B0x0,
            true => Alerten::B0x1,
        }
    }
    #[doc = "The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alerten::B0x0
    }
    #[doc = "The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alerten::B0x1
    }
}
#[doc = "Field `ALERTEN` writer - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type AlertenW<'a, REG> = crate::BitWriter<'a, REG, Alerten>;
impl<'a, REG> AlertenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Alerten::B0x0)
    }
    #[doc = "The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Alerten::B0x1)
    }
}
#[doc = "PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecen {
    #[doc = "0: PEC calculation disabled"]
    B0x0 = 0,
    #[doc = "1: PEC calculation enabled"]
    B0x1 = 1,
}
impl From<Pecen> for bool {
    #[inline(always)]
    fn from(variant: Pecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PecenR = crate::BitReader<Pecen>;
impl PecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecen {
        match self.bits {
            false => Pecen::B0x0,
            true => Pecen::B0x1,
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pecen::B0x0
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pecen::B0x1
    }
}
#[doc = "Field `PECEN` writer - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG, Pecen>;
impl<'a, REG> PecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::B0x0)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    pub fn addrie(&self) -> AddrieR {
        AddrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NackieR {
        NackieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    pub fn stopie(&self) -> StopieR {
        StopieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn anfoff(&self) -> AnfoffR {
        AnfoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    pub fn sbc(&self) -> SbcR {
        SbcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn nostretch(&self) -> NostretchR {
        NostretchR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn smbhen(&self) -> SmbhenR {
        SmbhenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn smbden(&self) -> SmbdenR {
        SmbdenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn alerten(&self) -> AlertenR {
        AlertenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<I2cCr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TxieW<I2cCr1Spec> {
        TxieW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RxieW<I2cCr1Spec> {
        RxieW::new(self, 2)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> AddrieW<I2cCr1Spec> {
        AddrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NackieW<I2cCr1Spec> {
        NackieW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> StopieW<I2cCr1Spec> {
        StopieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<I2cCr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<I2cCr1Spec> {
        ErrieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DnfW<I2cCr1Spec> {
        DnfW::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> AnfoffW<I2cCr1Spec> {
        AnfoffW::new(self, 12)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<I2cCr1Spec> {
        TxdmaenW::new(self, 14)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<I2cCr1Spec> {
        RxdmaenW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SbcW<I2cCr1Spec> {
        SbcW::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NostretchW<I2cCr1Spec> {
        NostretchW::new(self, 17)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WupenW<I2cCr1Spec> {
        WupenW::new(self, 18)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GcenW<I2cCr1Spec> {
        GcenW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SmbhenW<I2cCr1Spec> {
        SmbhenW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SmbdenW<I2cCr1Spec> {
        SmbdenW::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> AlertenW<I2cCr1Spec> {
        AlertenW::new(self, 22)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PecenW<I2cCr1Spec> {
        PecenW::new(self, 23)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCr1Spec;
impl crate::RegisterSpec for I2cCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cr1::R`](R) reader structure"]
impl crate::Readable for I2cCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cr1::W`](W) writer structure"]
impl crate::Writable for I2cCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CR1 to value 0"]
impl crate::Resettable for I2cCr1Spec {
    const RESET_VALUE: u32 = 0;
}
