#[doc = "Register `ISR_FIFO_DISABLED` reader"]
pub type R = crate::R<IsrFifoDisabledSpec>;
#[doc = "Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: No parity error"]
    B0x0 = 0,
    #[doc = "1: Parity error"]
    B0x1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register."]
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
    #[doc = "No parity error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pe::B0x0
    }
    #[doc = "Parity error"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pe::B0x1
    }
}
#[doc = "Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE=1 in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: No Framing error is detected"]
    B0x0 = 0,
    #[doc = "1: Framing error or break character is detected"]
    B0x1 = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE=1 in the USART_CR1 register."]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::B0x0,
            true => Fe::B0x1,
        }
    }
    #[doc = "No Framing error is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fe::B0x0
    }
    #[doc = "Framing error or break character is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fe::B0x1
    }
}
#[doc = "Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page861).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ne {
    #[doc = "0: No noise is detected"]
    B0x0 = 0,
    #[doc = "1: Noise is detected"]
    B0x1 = 1,
}
impl From<Ne> for bool {
    #[inline(always)]
    fn from(variant: Ne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NE` reader - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page861)."]
pub type NeR = crate::BitReader<Ne>;
impl NeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ne {
        match self.bits {
            false => Ne::B0x0,
            true => Ne::B0x1,
        }
    }
    #[doc = "No noise is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ne::B0x0
    }
    #[doc = "Noise is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ne::B0x1
    }
}
#[doc = "Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXNE=1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ore {
    #[doc = "0: No overrun error"]
    B0x0 = 0,
    #[doc = "1: Overrun error is detected"]
    B0x1 = 1,
}
impl From<Ore> for bool {
    #[inline(always)]
    fn from(variant: Ore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXNE=1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register."]
pub type OreR = crate::BitReader<Ore>;
impl OreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ore {
        match self.bits {
            false => Ore::B0x0,
            true => Ore::B0x1,
        }
    }
    #[doc = "No overrun error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ore::B0x0
    }
    #[doc = "Overrun error is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ore::B0x1
    }
}
#[doc = "Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: No Idle line is detected"]
    B0x0 = 0,
    #[doc = "1: Idle line is detected"]
    B0x1 = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::B0x0,
            true => Idle::B0x1,
        }
    }
    #[doc = "No Idle line is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Idle::B0x0
    }
    #[doc = "Idle line is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Idle::B0x1
    }
}
#[doc = "Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE=1 in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxne {
    #[doc = "0: Data is not received"]
    B0x0 = 0,
    #[doc = "1: Received data is ready to be read."]
    B0x1 = 1,
}
impl From<Rxne> for bool {
    #[inline(always)]
    fn from(variant: Rxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE=1 in the USART_CR1 register."]
pub type RxneR = crate::BitReader<Rxne>;
impl RxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxne {
        match self.bits {
            false => Rxne::B0x0,
            true => Rxne::B0x1,
        }
    }
    #[doc = "Data is not received"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxne::B0x0
    }
    #[doc = "Received data is ready to be read."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxne::B0x1
    }
}
#[doc = "Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    #[doc = "0: Transmission is not complete"]
    B0x0 = 0,
    #[doc = "1: Transmission is complete"]
    B0x1 = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately."]
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::B0x0,
            true => Tc::B0x1,
        }
    }
    #[doc = "Transmission is not complete"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tc::B0x0
    }
    #[doc = "Transmission is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tc::B0x1
    }
}
#[doc = "Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T=0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit =1 in the USART_CR1 register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    #[doc = "0: Data register full"]
    B0x0 = 0,
    #[doc = "1: Data register not full"]
    B0x1 = 1,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T=0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit =1 in the USART_CR1 register."]
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            false => Txe::B0x0,
            true => Txe::B0x1,
        }
    }
    #[doc = "Data register full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txe::B0x0
    }
    #[doc = "Data register not full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txe::B0x1
    }
}
#[doc = "LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdf {
    #[doc = "0: LIN Break not detected"]
    B0x0 = 0,
    #[doc = "1: LIN break detected"]
    B0x1 = 1,
}
impl From<Lbdf> for bool {
    #[inline(always)]
    fn from(variant: Lbdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDF` reader - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to ."]
pub type LbdfR = crate::BitReader<Lbdf>;
impl LbdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdf {
        match self.bits {
            false => Lbdf::B0x0,
            true => Lbdf::B0x1,
        }
    }
    #[doc = "LIN Break not detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lbdf::B0x0
    }
    #[doc = "LIN break detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lbdf::B0x1
    }
}
#[doc = "CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsif {
    #[doc = "0: No change occurred on the nCTS status line"]
    B0x0 = 0,
    #[doc = "1: A change occurred on the nCTS status line"]
    B0x1 = 1,
}
impl From<Ctsif> for bool {
    #[inline(always)]
    fn from(variant: Ctsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CtsifR = crate::BitReader<Ctsif>;
impl CtsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsif {
        match self.bits {
            false => Ctsif::B0x0,
            true => Ctsif::B0x1,
        }
    }
    #[doc = "No change occurred on the nCTS status line"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctsif::B0x0
    }
    #[doc = "A change occurred on the nCTS status line"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctsif::B0x1
    }
}
#[doc = "CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: nCTS line set"]
    B0x0 = 0,
    #[doc = "1: nCTS line reset"]
    B0x1 = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::B0x0,
            true => Cts::B0x1,
        }
    }
    #[doc = "nCTS line set"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cts::B0x0
    }
    #[doc = "nCTS line reset"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cts::B0x1
    }
}
#[doc = "Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtof {
    #[doc = "0: Timeout value not reached"]
    B0x0 = 0,
    #[doc = "1: Timeout value reached without any data reception"]
    B0x1 = 1,
}
impl From<Rtof> for bool {
    #[inline(always)]
    fn from(variant: Rtof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOF` reader - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value."]
pub type RtofR = crate::BitReader<Rtof>;
impl RtofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtof {
        match self.bits {
            false => Rtof::B0x0,
            true => Rtof::B0x1,
        }
    }
    #[doc = "Timeout value not reached"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtof::B0x0
    }
    #[doc = "Timeout value reached without any data reception"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtof::B0x1
    }
}
#[doc = "End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eobf {
    #[doc = "0: End of Block not reached"]
    B0x0 = 0,
    #[doc = "1: End of Block (number of characters) reached"]
    B0x1 = 1,
}
impl From<Eobf> for bool {
    #[inline(always)]
    fn from(variant: Eobf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBF` reader - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to ."]
pub type EobfR = crate::BitReader<Eobf>;
impl EobfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eobf {
        match self.bits {
            false => Eobf::B0x0,
            true => Eobf::B0x1,
        }
    }
    #[doc = "End of Block not reached"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eobf::B0x0
    }
    #[doc = "End of Block (number of characters) reached"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eobf::B0x1
    }
}
#[doc = "SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udr {
    #[doc = "0: No underrun error"]
    B0x0 = 0,
    #[doc = "1: underrun error"]
    B0x1 = 1,
}
impl From<Udr> for bool {
    #[inline(always)]
    fn from(variant: Udr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDR` reader - SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to ."]
pub type UdrR = crate::BitReader<Udr>;
impl UdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udr {
        match self.bits {
            false => Udr::B0x0,
            true => Udr::B0x1,
        }
    }
    #[doc = "No underrun error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Udr::B0x0
    }
    #[doc = "underrun error"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Udr::B0x1
    }
}
#[doc = "Field `ABRE` reader - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
pub type AbreR = crate::BitReader;
#[doc = "Field `ABRF` reader - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXNE is also set, generating an interrupt if RXNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
pub type AbrfR = crate::BitReader;
#[doc = "Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: USART is idle (no reception)"]
    B0x0 = 0,
    #[doc = "1: Reception on going"]
    B0x1 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::B0x0,
            true => Busy::B0x1,
        }
    }
    #[doc = "USART is idle (no reception)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Busy::B0x0
    }
    #[doc = "Reception on going"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Busy::B0x1
    }
}
#[doc = "Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmf {
    #[doc = "0: No Character match detected"]
    B0x0 = 0,
    #[doc = "1: Character Match detected"]
    B0x1 = 1,
}
impl From<Cmf> for bool {
    #[inline(always)]
    fn from(variant: Cmf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register."]
pub type CmfR = crate::BitReader<Cmf>;
impl CmfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmf {
        match self.bits {
            false => Cmf::B0x0,
            true => Cmf::B0x1,
        }
    }
    #[doc = "No Character match detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmf::B0x0
    }
    #[doc = "Character Match detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmf::B0x1
    }
}
#[doc = "Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbkf {
    #[doc = "0: Break character transmitted"]
    B0x0 = 0,
    #[doc = "1: Break character requested by setting SBKRQ bit in USART_RQR register"]
    B0x1 = 1,
}
impl From<Sbkf> for bool {
    #[inline(always)]
    fn from(variant: Sbkf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKF` reader - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
pub type SbkfR = crate::BitReader<Sbkf>;
impl SbkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbkf {
        match self.bits {
            false => Sbkf::B0x0,
            true => Sbkf::B0x1,
        }
    }
    #[doc = "Break character transmitted"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sbkf::B0x0
    }
    #[doc = "Break character requested by setting SBKRQ bit in USART_RQR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sbkf::B0x1
    }
}
#[doc = "Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwu {
    #[doc = "0: Receiver in active mode"]
    B0x0 = 0,
    #[doc = "1: Receiver in Mute mode"]
    B0x1 = 1,
}
impl From<Rwu> for bool {
    #[inline(always)]
    fn from(variant: Rwu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type RwuR = crate::BitReader<Rwu>;
impl RwuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwu {
        match self.bits {
            false => Rwu::B0x0,
            true => Rwu::B0x1,
        }
    }
    #[doc = "Receiver in active mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rwu::B0x0
    }
    #[doc = "Receiver in Mute mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rwu::B0x1
    }
}
#[doc = "Field `WUF` reader - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE=1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type WufR = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period."]
pub type TeackR = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type ReackR = crate::BitReader;
#[doc = "Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1'. Refer to on page835.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcbgt {
    #[doc = "0: Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)"]
    B0x0 = 0,
    #[doc = "1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)."]
    B0x1 = 1,
}
impl From<Tcbgt> for bool {
    #[inline(always)]
    fn from(variant: Tcbgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCBGT` reader - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1'. Refer to on page835."]
pub type TcbgtR = crate::BitReader<Tcbgt>;
impl TcbgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcbgt {
        match self.bits {
            false => Tcbgt::B0x0,
            true => Tcbgt::B0x1,
        }
    }
    #[doc = "Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcbgt::B0x0
    }
    #[doc = "Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcbgt::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE=1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page861)."]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXNE=1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register."]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE=1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T=0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit =1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdf(&self) -> LbdfR {
        LbdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn rtof(&self) -> RtofR {
        RtofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn eobf(&self) -> EobfR {
        EobfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn udr(&self) -> UdrR {
        UdrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn abre(&self) -> AbreR {
        AbreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXNE is also set, generating an interrupt if RXNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn abrf(&self) -> AbrfR {
        AbrfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register."]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
    #[inline(always)]
    pub fn sbkf(&self) -> SbkfR {
        SbkfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE=1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period."]
    #[inline(always)]
    pub fn teack(&self) -> TeackR {
        TeackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn reack(&self) -> ReackR {
        ReackR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1'. Refer to on page835."]
    #[inline(always)]
    pub fn tcbgt(&self) -> TcbgtR {
        TcbgtR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Interrupt &amp; status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr_fifo_disabled::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrFifoDisabledSpec;
impl crate::RegisterSpec for IsrFifoDisabledSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr_fifo_disabled::R`](R) reader structure"]
impl crate::Readable for IsrFifoDisabledSpec {}
#[doc = "`reset()` method sets ISR_FIFO_DISABLED to value 0xc0"]
impl crate::Resettable for IsrFifoDisabledSpec {
    const RESET_VALUE: u32 = 0xc0;
}
