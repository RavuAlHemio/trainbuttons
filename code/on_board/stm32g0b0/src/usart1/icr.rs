#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
pub type PecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register."]
pub type FecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register."]
pub type NecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
pub type OrecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
pub type IdlecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFECF` writer - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register."]
pub type TxfecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register."]
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBGTCF` writer - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register."]
pub type TcbgtcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDCF` writer - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LbdcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to page835."]
pub type RtocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBCF` writer - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type EobcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRCF` writer - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to"]
pub type UdrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register."]
pub type CmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835."]
pub type WucfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PecfW<IcrSpec> {
        PecfW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FecfW<IcrSpec> {
        FecfW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn necf(&mut self) -> NecfW<IcrSpec> {
        NecfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> OrecfW<IcrSpec> {
        OrecfW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IdlecfW<IcrSpec> {
        IdlecfW::new(self, 4)
    }
    #[doc = "Bit 5 - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn txfecf(&mut self) -> TxfecfW<IcrSpec> {
        TxfecfW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TccfW<IcrSpec> {
        TccfW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn tcbgtcf(&mut self) -> TcbgtcfW<IcrSpec> {
        TcbgtcfW::new(self, 7)
    }
    #[doc = "Bit 8 - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LbdcfW<IcrSpec> {
        LbdcfW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CtscfW<IcrSpec> {
        CtscfW::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to page835."]
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RtocfW<IcrSpec> {
        RtocfW::new(self, 11)
    }
    #[doc = "Bit 12 - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EobcfW<IcrSpec> {
        EobcfW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to"]
    #[inline(always)]
    #[must_use]
    pub fn udrcf(&mut self) -> UdrcfW<IcrSpec> {
        UdrcfW::new(self, 13)
    }
    #[doc = "Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CmcfW<IcrSpec> {
        CmcfW::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835."]
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WucfW<IcrSpec> {
        WucfW::new(self, 20)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
