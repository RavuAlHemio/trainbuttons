#[doc = "Register `I2C_ISR` reader"]
pub type R = crate::R<I2cIsrSpec>;
#[doc = "Register `I2C_ISR` writer"]
pub type W = crate::W<I2cIsrSpec>;
#[doc = "Field `TXE` reader - Transmit data register empty (transmitters)"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Transmit data register empty (transmitters)"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIS` reader - Transmit interrupt status (transmitters)"]
pub type TxisR = crate::BitReader;
#[doc = "Field `TXIS` writer - Transmit interrupt status (transmitters)"]
pub type TxisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - Receive data register not empty (receivers)"]
pub type RxneR = crate::BitReader;
#[doc = "Field `ADDR` reader - Address matched (slave mode)"]
pub type AddrR = crate::BitReader;
#[doc = "Field `NACKF` reader - Not acknowledge received flag"]
pub type NackfR = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop detection flag"]
pub type StopfR = crate::BitReader;
#[doc = "Field `TC` reader - Transfer Complete (master mode)"]
pub type TcR = crate::BitReader;
#[doc = "Field `TCR` reader - Transfer Complete Reload"]
pub type TcrR = crate::BitReader;
#[doc = "Field `BERR` reader - Bus error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `ARLO` reader - Arbitration lost"]
pub type ArloR = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun/Underrun (slave mode)"]
pub type OvrR = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Timeout or t_low detection flag"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `ALERT` reader - SMBus alert"]
pub type AlertR = crate::BitReader;
#[doc = "Field `BUSY` reader - Bus busy"]
pub type BusyR = crate::BitReader;
#[doc = "Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Write transfer, slave enters receiver mode."]
    B0x0 = 0,
    #[doc = "1: Read transfer, slave enters transmitter mode."]
    B0x1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1)."]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::B0x0,
            true => Dir::B0x1,
        }
    }
    #[doc = "Write transfer, slave enters receiver mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dir::B0x0
    }
    #[doc = "Read transfer, slave enters transmitter mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dir::B0x1
    }
}
#[doc = "Field `ADDCODE` reader - Address match code (Slave mode)"]
pub type AddcodeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline(always)]
    pub fn txis(&self) -> TxisR {
        TxisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data register not empty (receivers)"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received flag"]
    #[inline(always)]
    pub fn nackf(&self) -> NackfR {
        NackfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection flag"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete (master mode)"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Reload"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost"]
    #[inline(always)]
    pub fn arlo(&self) -> ArloR {
        ArloR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun/Underrun (slave mode)"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timeout or t_low detection flag"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> AlertR {
        AlertR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1)."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Address match code (Slave mode)"]
    #[inline(always)]
    pub fn addcode(&self) -> AddcodeR {
        AddcodeR::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TxeW<I2cIsrSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TxisW<I2cIsrSpec> {
        TxisW::new(self, 1)
    }
}
#[doc = "Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cIsrSpec;
impl crate::RegisterSpec for I2cIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_isr::R`](R) reader structure"]
impl crate::Readable for I2cIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_isr::W`](W) writer structure"]
impl crate::Writable for I2cIsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_ISR to value 0x01"]
impl crate::Resettable for I2cIsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
