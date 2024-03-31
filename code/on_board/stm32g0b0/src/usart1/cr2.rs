#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slven {
    #[doc = "0: Slave mode disabled."]
    B0x0 = 0,
    #[doc = "1: Slave mode enabled."]
    B0x1 = 1,
}
impl From<Slven> for bool {
    #[inline(always)]
    fn from(variant: Slven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVEN` reader - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SlvenR = crate::BitReader<Slven>;
impl SlvenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slven {
        match self.bits {
            false => Slven::B0x0,
            true => Slven::B0x1,
        }
    }
    #[doc = "Slave mode disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Slven::B0x0
    }
    #[doc = "Slave mode enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Slven::B0x1
    }
}
#[doc = "Field `SLVEN` writer - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SlvenW<'a, REG> = crate::BitWriter<'a, REG, Slven>;
impl<'a, REG> SlvenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Slven::B0x0)
    }
    #[doc = "Slave mode enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Slven::B0x1)
    }
}
#[doc = "When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisNss {
    #[doc = "0: SPI slave selection depends on NSS input pin."]
    B0x0 = 0,
    #[doc = "1: SPI slave is always selected and NSS input pin is ignored."]
    B0x1 = 1,
}
impl From<DisNss> for bool {
    #[inline(always)]
    fn from(variant: DisNss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_NSS` reader - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DisNssR = crate::BitReader<DisNss>;
impl DisNssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisNss {
        match self.bits {
            false => DisNss::B0x0,
            true => DisNss::B0x1,
        }
    }
    #[doc = "SPI slave selection depends on NSS input pin."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DisNss::B0x0
    }
    #[doc = "SPI slave is always selected and NSS input pin is ignored."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DisNss::B0x1
    }
}
#[doc = "Field `DIS_NSS` writer - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DisNssW<'a, REG> = crate::BitWriter<'a, REG, DisNss>;
impl<'a, REG> DisNssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI slave selection depends on NSS input pin."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DisNss::B0x0)
    }
    #[doc = "SPI slave is always selected and NSS input pin is ignored."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DisNss::B0x1)
    }
}
#[doc = "7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addm7 {
    #[doc = "0: 4-bit address detection"]
    B0x0 = 0,
    #[doc = "1: 7-bit address detection (in 8-bit data mode)"]
    B0x1 = 1,
}
impl From<Addm7> for bool {
    #[inline(always)]
    fn from(variant: Addm7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type Addm7R = crate::BitReader<Addm7>;
impl Addm7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addm7 {
        match self.bits {
            false => Addm7::B0x0,
            true => Addm7::B0x1,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Addm7::B0x0
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Addm7::B0x1
    }
}
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG, Addm7>;
impl<'a, REG> Addm7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Addm7::B0x0)
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Addm7::B0x1)
    }
}
#[doc = "LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdl {
    #[doc = "0: 10-bit break detection"]
    B0x0 = 0,
    #[doc = "1: 11-bit break detection"]
    B0x1 = 1,
}
impl From<Lbdl> for bool {
    #[inline(always)]
    fn from(variant: Lbdl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDL` reader - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbdlR = crate::BitReader<Lbdl>;
impl LbdlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdl {
        match self.bits {
            false => Lbdl::B0x0,
            true => Lbdl::B0x1,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lbdl::B0x0
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lbdl::B0x1
    }
}
#[doc = "Field `LBDL` writer - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG, Lbdl>;
impl<'a, REG> LbdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdl::B0x0)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdl::B0x1)
    }
}
#[doc = "LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An interrupt is generated whenever LBDF = 1 in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Lbdie> for bool {
    #[inline(always)]
    fn from(variant: Lbdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbdieR = crate::BitReader<Lbdie>;
impl LbdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdie {
        match self.bits {
            false => Lbdie::B0x0,
            true => Lbdie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lbdie::B0x0
    }
    #[doc = "An interrupt is generated whenever LBDF = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lbdie::B0x1
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG, Lbdie>;
impl<'a, REG> LbdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::B0x0)
    }
    #[doc = "An interrupt is generated whenever LBDF = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::B0x1)
    }
}
#[doc = "Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbcl {
    #[doc = "0: The clock pulse of the last data bit is not output to the SCLK pin"]
    B0x0 = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the SCLK pin"]
    B0x1 = 1,
}
impl From<Lbcl> for bool {
    #[inline(always)]
    fn from(variant: Lbcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCL` reader - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbclR = crate::BitReader<Lbcl>;
impl LbclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbcl {
        match self.bits {
            false => Lbcl::B0x0,
            true => Lbcl::B0x1,
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the SCLK pin"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lbcl::B0x0
    }
    #[doc = "The clock pulse of the last data bit is output to the SCLK pin"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lbcl::B0x1
    }
}
#[doc = "Field `LBCL` writer - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbclW<'a, REG> = crate::BitWriter<'a, REG, Lbcl>;
impl<'a, REG> LbclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is not output to the SCLK pin"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbcl::B0x0)
    }
    #[doc = "The clock pulse of the last data bit is output to the SCLK pin"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbcl::B0x1)
    }
}
#[doc = "Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: The first clock transition is the first data capture edge"]
    B0x0 = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    B0x1 = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::B0x0,
            true => Cpha::B0x1,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cpha::B0x0
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cpha::B0x1
    }
}
#[doc = "Field `CPHA` writer - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::B0x0)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::B0x1)
    }
}
#[doc = "Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: Steady low value on SCLK pin outside transmission window"]
    B0x0 = 0,
    #[doc = "1: Steady high value on SCLK pin outside transmission window"]
    B0x1 = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::B0x0,
            true => Cpol::B0x1,
        }
    }
    #[doc = "Steady low value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cpol::B0x0
    }
    #[doc = "Steady high value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cpol::B0x1
    }
}
#[doc = "Field `CPOL` writer - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::B0x0)
    }
    #[doc = "Steady high value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::B0x1)
    }
}
#[doc = "Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "0: SCLK pin disabled"]
    B0x0 = 0,
    #[doc = "1: SCLK pin enabled"]
    B0x1 = 1,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            false => Clken::B0x0,
            true => Clken::B0x1,
        }
    }
    #[doc = "SCLK pin disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Clken::B0x0
    }
    #[doc = "SCLK pin enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Clken::B0x1
    }
}
#[doc = "Field `CLKEN` writer - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCLK pin disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::B0x0)
    }
    #[doc = "SCLK pin enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::B0x1)
    }
}
#[doc = "stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stop {
    #[doc = "0: 1 stop bit"]
    B0x0 = 0,
    #[doc = "1: 0.5 stop bit."]
    B0x1 = 1,
    #[doc = "2: 2 stop bits"]
    B0x2 = 2,
    #[doc = "3: 1.5 stop bits"]
    B0x3 = 3,
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stop {
    type Ux = u8;
}
impl crate::IsEnum for Stop {}
#[doc = "Field `STOP` reader - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type StopR = crate::FieldReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            0 => Stop::B0x0,
            1 => Stop::B0x1,
            2 => Stop::B0x2,
            3 => Stop::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stop::B0x0
    }
    #[doc = "0.5 stop bit."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stop::B0x1
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Stop::B0x2
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Stop::B0x3
    }
}
#[doc = "Field `STOP` writer - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stop, crate::Safe>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x0)
    }
    #[doc = "0.5 stop bit."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x1)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x2)
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x3)
    }
}
#[doc = "LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linen {
    #[doc = "0: LIN mode disabled"]
    B0x0 = 0,
    #[doc = "1: LIN mode enabled"]
    B0x1 = 1,
}
impl From<Linen> for bool {
    #[inline(always)]
    fn from(variant: Linen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LinenR = crate::BitReader<Linen>;
impl LinenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linen {
        match self.bits {
            false => Linen::B0x0,
            true => Linen::B0x1,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Linen::B0x0
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Linen::B0x1
    }
}
#[doc = "Field `LINEN` writer - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG, Linen>;
impl<'a, REG> LinenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Linen::B0x0)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Linen::B0x1)
    }
}
#[doc = "Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    B0x0 = 0,
    #[doc = "1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    B0x1 = 1,
}
impl From<Swap> for bool {
    #[inline(always)]
    fn from(variant: Swap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type SwapR = crate::BitReader<Swap>;
impl SwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap {
        match self.bits {
            false => Swap::B0x0,
            true => Swap::B0x1,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swap::B0x0
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swap::B0x1
    }
}
#[doc = "Field `SWAP` writer - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG, Swap>;
impl<'a, REG> SwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::B0x0)
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::B0x1)
    }
}
#[doc = "RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)"]
    B0x0 = 0,
    #[doc = "1: RX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle)."]
    B0x1 = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::B0x0,
            true => Rxinv::B0x1,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxinv::B0x0
    }
    #[doc = "RX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxinv::B0x1
    }
}
#[doc = "Field `RXINV` writer - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B0x0)
    }
    #[doc = "RX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B0x1)
    }
}
#[doc = "TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)"]
    B0x0 = 0,
    #[doc = "1: TX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle)."]
    B0x1 = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type TxinvR = crate::BitReader<Txinv>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinv {
        match self.bits {
            false => Txinv::B0x0,
            true => Txinv::B0x1,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txinv::B0x0
    }
    #[doc = "TX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txinv::B0x1
    }
}
#[doc = "Field `TXINV` writer - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B0x0)
    }
    #[doc = "TX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B0x1)
    }
}
#[doc = "Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datainv {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic. (1 = H, 0 = L)"]
    B0x0 = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic. (1 = L, 0 = H). The parity bit is also inverted."]
    B0x1 = 1,
}
impl From<Datainv> for bool {
    #[inline(always)]
    fn from(variant: Datainv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type DatainvR = crate::BitReader<Datainv>;
impl DatainvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datainv {
        match self.bits {
            false => Datainv::B0x0,
            true => Datainv::B0x1,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1 = H, 0 = L)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Datainv::B0x0
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1 = L, 0 = H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Datainv::B0x1
    }
}
#[doc = "Field `DATAINV` writer - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG, Datainv>;
impl<'a, REG> DatainvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1 = H, 0 = L)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Datainv::B0x0)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1 = L, 0 = H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Datainv::B0x1)
    }
}
#[doc = "Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbfirst {
    #[doc = "0: data is transmitted/received with data bit 0 first, following the start bit."]
    B0x0 = 0,
    #[doc = "1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    B0x1 = 1,
}
impl From<Msbfirst> for bool {
    #[inline(always)]
    fn from(variant: Msbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type MsbfirstR = crate::BitReader<Msbfirst>;
impl MsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbfirst {
        match self.bits {
            false => Msbfirst::B0x0,
            true => Msbfirst::B0x1,
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msbfirst::B0x0
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msbfirst::B0x1
    }
}
#[doc = "Field `MSBFIRST` writer - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Msbfirst>;
impl<'a, REG> MsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::B0x0)
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::B0x1)
    }
}
#[doc = "Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abren {
    #[doc = "0: Auto baud rate detection is disabled."]
    B0x0 = 0,
    #[doc = "1: Auto baud rate detection is enabled."]
    B0x1 = 1,
}
impl From<Abren> for bool {
    #[inline(always)]
    fn from(variant: Abren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABREN` reader - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type AbrenR = crate::BitReader<Abren>;
impl AbrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abren {
        match self.bits {
            false => Abren::B0x0,
            true => Abren::B0x1,
        }
    }
    #[doc = "Auto baud rate detection is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Abren::B0x0
    }
    #[doc = "Auto baud rate detection is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Abren::B0x1
    }
}
#[doc = "Field `ABREN` writer - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type AbrenW<'a, REG> = crate::BitWriter<'a, REG, Abren>;
impl<'a, REG> AbrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto baud rate detection is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Abren::B0x0)
    }
    #[doc = "Auto baud rate detection is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Abren::B0x1)
    }
}
#[doc = "Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Abrmod {
    #[doc = "0: Measurement of the start bit is used to detect the baud rate."]
    B0x0 = 0,
    #[doc = "1: Falling edge to falling edge measurement (the received frame must start with a single bit = 1 and Frame = Start10xxxxxx)"]
    B0x1 = 1,
    #[doc = "2: 0x7F frame detection."]
    B0x2 = 2,
    #[doc = "3: 0x55 frame detection"]
    B0x3 = 3,
}
impl From<Abrmod> for u8 {
    #[inline(always)]
    fn from(variant: Abrmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abrmod {
    type Ux = u8;
}
impl crate::IsEnum for Abrmod {}
#[doc = "Field `ABRMOD` reader - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type AbrmodR = crate::FieldReader<Abrmod>;
impl AbrmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abrmod {
        match self.bits {
            0 => Abrmod::B0x0,
            1 => Abrmod::B0x1,
            2 => Abrmod::B0x2,
            3 => Abrmod::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Abrmod::B0x0
    }
    #[doc = "Falling edge to falling edge measurement (the received frame must start with a single bit = 1 and Frame = Start10xxxxxx)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Abrmod::B0x1
    }
    #[doc = "0x7F frame detection."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Abrmod::B0x2
    }
    #[doc = "0x55 frame detection"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Abrmod::B0x3
    }
}
#[doc = "Field `ABRMOD` writer - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type AbrmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Abrmod, crate::Safe>;
impl<'a, REG> AbrmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Abrmod::B0x0)
    }
    #[doc = "Falling edge to falling edge measurement (the received frame must start with a single bit = 1 and Frame = Start10xxxxxx)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Abrmod::B0x1)
    }
    #[doc = "0x7F frame detection."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Abrmod::B0x2)
    }
    #[doc = "0x55 frame detection"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Abrmod::B0x3)
    }
}
#[doc = "Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtoen {
    #[doc = "0: Receiver timeout feature disabled."]
    B0x0 = 0,
    #[doc = "1: Receiver timeout feature enabled."]
    B0x1 = 1,
}
impl From<Rtoen> for bool {
    #[inline(always)]
    fn from(variant: Rtoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOEN` reader - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RtoenR = crate::BitReader<Rtoen>;
impl RtoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtoen {
        match self.bits {
            false => Rtoen::B0x0,
            true => Rtoen::B0x1,
        }
    }
    #[doc = "Receiver timeout feature disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtoen::B0x0
    }
    #[doc = "Receiver timeout feature enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtoen::B0x1
    }
}
#[doc = "Field `RTOEN` writer - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RtoenW<'a, REG> = crate::BitWriter<'a, REG, Rtoen>;
impl<'a, REG> RtoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver timeout feature disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtoen::B0x0)
    }
    #[doc = "Receiver timeout feature enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtoen::B0x1)
    }
}
#[doc = "Field `ADD` reader - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)."]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)."]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn slven(&self) -> SlvenR {
        SlvenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dis_nss(&self) -> DisNssR {
        DisNssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbcl(&self) -> LbclR {
        LbclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn abren(&self) -> AbrenR {
        AbrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn abrmod(&self) -> AbrmodR {
        AbrmodR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtoen(&self) -> RtoenR {
        RtoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn slven(&mut self) -> SlvenW<Cr2Spec> {
        SlvenW::new(self, 0)
    }
    #[doc = "Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn dis_nss(&mut self) -> DisNssW<Cr2Spec> {
        DisNssW::new(self, 3)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> Addm7W<Cr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LbdlW<Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LbdieW<Cr2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LbclW<Cr2Spec> {
        LbclW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<Cr2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<Cr2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<Cr2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LinenW<Cr2Spec> {
        LinenW::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SwapW<Cr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<Cr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<Cr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DatainvW<Cr2Spec> {
        DatainvW::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MsbfirstW<Cr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> AbrenW<Cr2Spec> {
        AbrenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn abrmod(&mut self) -> AbrmodW<Cr2Spec> {
        AbrmodW::new(self, 21)
    }
    #[doc = "Bit 23 - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn rtoen(&mut self) -> RtoenW<Cr2Spec> {
        RtoenW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<Cr2Spec> {
        AddW::new(self, 24)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
