#[doc = "Register `SPI_CR1` reader"]
pub type R = crate::R<SpiCr1Spec>;
#[doc = "Register `SPI_CR1` writer"]
pub type W = crate::W<SpiCr1Spec>;
#[doc = "Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.\n\nValue on reset: 0"]
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
#[doc = "Field `CPHA` reader - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
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
#[doc = "Field `CPHA` writer - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
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
#[doc = "Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: CK to 0 when idle"]
    B0x0 = 0,
    #[doc = "1: CK to 1 when idle"]
    B0x1 = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
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
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cpol::B0x0
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cpol::B0x1
    }
}
#[doc = "Field `CPOL` writer - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::B0x0)
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::B0x1)
    }
}
#[doc = "Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstr {
    #[doc = "0: Slave configuration"]
    B0x0 = 0,
    #[doc = "1: Master configuration"]
    B0x1 = 1,
}
impl From<Mstr> for bool {
    #[inline(always)]
    fn from(variant: Mstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
pub type MstrR = crate::BitReader<Mstr>;
impl MstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstr {
        match self.bits {
            false => Mstr::B0x0,
            true => Mstr::B0x1,
        }
    }
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mstr::B0x0
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mstr::B0x1
    }
}
#[doc = "Field `MSTR` writer - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, Mstr>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::B0x0)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::B0x1)
    }
}
#[doc = "Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Br {
    #[doc = "0: fPCLK/2"]
    B0x0 = 0,
    #[doc = "1: fPCLK/4"]
    B0x1 = 1,
    #[doc = "2: fPCLK/8"]
    B0x2 = 2,
    #[doc = "3: fPCLK/16"]
    B0x3 = 3,
    #[doc = "4: fPCLK/32"]
    B0x4 = 4,
    #[doc = "5: fPCLK/64"]
    B0x5 = 5,
    #[doc = "6: fPCLK/128"]
    B0x6 = 6,
    #[doc = "7: fPCLK/256"]
    B0x7 = 7,
}
impl From<Br> for u8 {
    #[inline(always)]
    fn from(variant: Br) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Br {
    type Ux = u8;
}
impl crate::IsEnum for Br {}
#[doc = "Field `BR` reader - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
pub type BrR = crate::FieldReader<Br>;
impl BrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Br {
        match self.bits {
            0 => Br::B0x0,
            1 => Br::B0x1,
            2 => Br::B0x2,
            3 => Br::B0x3,
            4 => Br::B0x4,
            5 => Br::B0x5,
            6 => Br::B0x6,
            7 => Br::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "fPCLK/2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Br::B0x0
    }
    #[doc = "fPCLK/4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Br::B0x1
    }
    #[doc = "fPCLK/8"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Br::B0x2
    }
    #[doc = "fPCLK/16"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Br::B0x3
    }
    #[doc = "fPCLK/32"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Br::B0x4
    }
    #[doc = "fPCLK/64"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Br::B0x5
    }
    #[doc = "fPCLK/128"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Br::B0x6
    }
    #[doc = "fPCLK/256"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Br::B0x7
    }
}
#[doc = "Field `BR` writer - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Br, crate::Safe>;
impl<'a, REG> BrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fPCLK/2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x0)
    }
    #[doc = "fPCLK/4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x1)
    }
    #[doc = "fPCLK/8"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x2)
    }
    #[doc = "fPCLK/16"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x3)
    }
    #[doc = "fPCLK/32"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x4)
    }
    #[doc = "fPCLK/64"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x5)
    }
    #[doc = "fPCLK/128"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x6)
    }
    #[doc = "fPCLK/256"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Br::B0x7)
    }
}
#[doc = "SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spe {
    #[doc = "0: Peripheral disabled"]
    B0x0 = 0,
    #[doc = "1: Peripheral enabled"]
    B0x1 = 1,
}
impl From<Spe> for bool {
    #[inline(always)]
    fn from(variant: Spe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPE` reader - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
pub type SpeR = crate::BitReader<Spe>;
impl SpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spe {
        match self.bits {
            false => Spe::B0x0,
            true => Spe::B0x1,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spe::B0x0
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spe::B0x1
    }
}
#[doc = "Field `SPE` writer - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG, Spe>;
impl<'a, REG> SpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spe::B0x0)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spe::B0x1)
    }
}
#[doc = "Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbfirst {
    #[doc = "0: data is transmitted / received with the MSB first"]
    B0x0 = 0,
    #[doc = "1: data is transmitted / received with the LSB first"]
    B0x1 = 1,
}
impl From<Lsbfirst> for bool {
    #[inline(always)]
    fn from(variant: Lsbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFIRST` reader - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
pub type LsbfirstR = crate::BitReader<Lsbfirst>;
impl LsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbfirst {
        match self.bits {
            false => Lsbfirst::B0x0,
            true => Lsbfirst::B0x1,
        }
    }
    #[doc = "data is transmitted / received with the MSB first"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsbfirst::B0x0
    }
    #[doc = "data is transmitted / received with the LSB first"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsbfirst::B0x1
    }
}
#[doc = "Field `LSBFIRST` writer - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
pub type LsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Lsbfirst>;
impl<'a, REG> LsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted / received with the MSB first"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbfirst::B0x0)
    }
    #[doc = "data is transmitted / received with the LSB first"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbfirst::B0x1)
    }
}
#[doc = "Field `SSI` reader - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SsiR = crate::BitReader;
#[doc = "Field `SSI` writer - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssm {
    #[doc = "0: Software slave management disabled"]
    B0x0 = 0,
    #[doc = "1: Software slave management enabled"]
    B0x1 = 1,
}
impl From<Ssm> for bool {
    #[inline(always)]
    fn from(variant: Ssm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSM` reader - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SsmR = crate::BitReader<Ssm>;
impl SsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssm {
        match self.bits {
            false => Ssm::B0x0,
            true => Ssm::B0x1,
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ssm::B0x0
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ssm::B0x1
    }
}
#[doc = "Field `SSM` writer - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG, Ssm>;
impl<'a, REG> SsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssm::B0x0)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssm::B0x1)
    }
}
#[doc = "Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxonly {
    #[doc = "0: Full-duplex (Transmit and receive)"]
    B0x0 = 0,
    #[doc = "1: Output disabled (Receive-only mode)"]
    B0x1 = 1,
}
impl From<Rxonly> for bool {
    #[inline(always)]
    fn from(variant: Rxonly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXONLY` reader - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
pub type RxonlyR = crate::BitReader<Rxonly>;
impl RxonlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxonly {
        match self.bits {
            false => Rxonly::B0x0,
            true => Rxonly::B0x1,
        }
    }
    #[doc = "Full-duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxonly::B0x0
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxonly::B0x1
    }
}
#[doc = "Field `RXONLY` writer - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
pub type RxonlyW<'a, REG> = crate::BitWriter<'a, REG, Rxonly>;
impl<'a, REG> RxonlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full-duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxonly::B0x0)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxonly::B0x1)
    }
}
#[doc = "CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcl {
    #[doc = "0: 8-bit CRC length"]
    B0x0 = 0,
    #[doc = "1: 16-bit CRC length"]
    B0x1 = 1,
}
impl From<Crcl> for bool {
    #[inline(always)]
    fn from(variant: Crcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCL` reader - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CrclR = crate::BitReader<Crcl>;
impl CrclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcl {
        match self.bits {
            false => Crcl::B0x0,
            true => Crcl::B0x1,
        }
    }
    #[doc = "8-bit CRC length"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcl::B0x0
    }
    #[doc = "16-bit CRC length"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcl::B0x1
    }
}
#[doc = "Field `CRCL` writer - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CrclW<'a, REG> = crate::BitWriter<'a, REG, Crcl>;
impl<'a, REG> CrclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit CRC length"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcl::B0x0)
    }
    #[doc = "16-bit CRC length"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcl::B0x1)
    }
}
#[doc = "Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcnext {
    #[doc = "0: Next transmit value is from Tx buffer."]
    B0x0 = 0,
    #[doc = "1: Next transmit value is from Tx CRC register."]
    B0x1 = 1,
}
impl From<Crcnext> for bool {
    #[inline(always)]
    fn from(variant: Crcnext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCNEXT` reader - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
pub type CrcnextR = crate::BitReader<Crcnext>;
impl CrcnextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcnext {
        match self.bits {
            false => Crcnext::B0x0,
            true => Crcnext::B0x1,
        }
    }
    #[doc = "Next transmit value is from Tx buffer."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcnext::B0x0
    }
    #[doc = "Next transmit value is from Tx CRC register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcnext::B0x1
    }
}
#[doc = "Field `CRCNEXT` writer - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
pub type CrcnextW<'a, REG> = crate::BitWriter<'a, REG, Crcnext>;
impl<'a, REG> CrcnextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next transmit value is from Tx buffer."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcnext::B0x0)
    }
    #[doc = "Next transmit value is from Tx CRC register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcnext::B0x1)
    }
}
#[doc = "Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcen {
    #[doc = "0: CRC calculation disabled"]
    B0x0 = 0,
    #[doc = "1: CRC calculation enabled"]
    B0x1 = 1,
}
impl From<Crcen> for bool {
    #[inline(always)]
    fn from(variant: Crcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CrcenR = crate::BitReader<Crcen>;
impl CrcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcen {
        match self.bits {
            false => Crcen::B0x0,
            true => Crcen::B0x1,
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcen::B0x0
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcen::B0x1
    }
}
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG, Crcen>;
impl<'a, REG> CrcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcen::B0x0)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcen::B0x1)
    }
}
#[doc = "Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bidioe {
    #[doc = "0: Output disabled (receive-only mode)"]
    B0x0 = 0,
    #[doc = "1: Output enabled (transmit-only mode)"]
    B0x1 = 1,
}
impl From<Bidioe> for bool {
    #[inline(always)]
    fn from(variant: Bidioe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
pub type BidioeR = crate::BitReader<Bidioe>;
impl BidioeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bidioe {
        match self.bits {
            false => Bidioe::B0x0,
            true => Bidioe::B0x1,
        }
    }
    #[doc = "Output disabled (receive-only mode)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bidioe::B0x0
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bidioe::B0x1
    }
}
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
pub type BidioeW<'a, REG> = crate::BitWriter<'a, REG, Bidioe>;
impl<'a, REG> BidioeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output disabled (receive-only mode)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bidioe::B0x0)
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bidioe::B0x1)
    }
}
#[doc = "Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bidimode {
    #[doc = "0: 2-line unidirectional data mode selected"]
    B0x0 = 0,
    #[doc = "1: 1-line bidirectional data mode selected"]
    B0x1 = 1,
}
impl From<Bidimode> for bool {
    #[inline(always)]
    fn from(variant: Bidimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
pub type BidimodeR = crate::BitReader<Bidimode>;
impl BidimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bidimode {
        match self.bits {
            false => Bidimode::B0x0,
            true => Bidimode::B0x1,
        }
    }
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bidimode::B0x0
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bidimode::B0x1
    }
}
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
pub type BidimodeW<'a, REG> = crate::BitWriter<'a, REG, Bidimode>;
impl<'a, REG> BidimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bidimode::B0x0)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bidimode::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LsbfirstR {
        LsbfirstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn ssi(&self) -> SsiR {
        SsiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn rxonly(&self) -> RxonlyR {
        RxonlyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcl(&self) -> CrclR {
        CrclR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcnext(&self) -> CrcnextR {
        CrcnextR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn bidioe(&self) -> BidioeR {
        BidioeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn bidimode(&self) -> BidimodeR {
        BidimodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<SpiCr1Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<SpiCr1Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MstrW<SpiCr1Spec> {
        MstrW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BrW<SpiCr1Spec> {
        BrW::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SpeW<SpiCr1Spec> {
        SpeW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LsbfirstW<SpiCr1Spec> {
        LsbfirstW::new(self, 7)
    }
    #[doc = "Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SsiW<SpiCr1Spec> {
        SsiW::new(self, 8)
    }
    #[doc = "Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SsmW<SpiCr1Spec> {
        SsmW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxonly(&mut self) -> RxonlyW<SpiCr1Spec> {
        RxonlyW::new(self, 10)
    }
    #[doc = "Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcl(&mut self) -> CrclW<SpiCr1Spec> {
        CrclW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcnext(&mut self) -> CrcnextW<SpiCr1Spec> {
        CrcnextW::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<SpiCr1Spec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn bidioe(&mut self) -> BidioeW<SpiCr1Spec> {
        BidioeW::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn bidimode(&mut self) -> BidimodeW<SpiCr1Spec> {
        BidimodeW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCr1Spec;
impl crate::RegisterSpec for SpiCr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_cr1::R`](R) reader structure"]
impl crate::Readable for SpiCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_cr1::W`](W) writer structure"]
impl crate::Writable for SpiCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_CR1 to value 0"]
impl crate::Resettable for SpiCr1Spec {
    const RESET_VALUE: u16 = 0;
}
