#[doc = "Register `TIM1_DCR` reader"]
pub type R = crate::R<Tim1DcrSpec>;
#[doc = "Register `TIM1_DCR` writer"]
pub type W = crate::W<Tim1DcrSpec>;
#[doc = "DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dba {
    #[doc = "0: TIMx_CR1,"]
    B0x0 = 0,
    #[doc = "1: TIMx_CR2,"]
    B0x1 = 1,
    #[doc = "2: TIMx_SMCR,"]
    B0x2 = 2,
}
impl From<Dba> for u8 {
    #[inline(always)]
    fn from(variant: Dba) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dba {
    type Ux = u8;
}
impl crate::IsEnum for Dba {}
#[doc = "Field `DBA` reader - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
pub type DbaR = crate::FieldReader<Dba>;
impl DbaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dba> {
        match self.bits {
            0 => Some(Dba::B0x0),
            1 => Some(Dba::B0x1),
            2 => Some(Dba::B0x2),
            _ => None,
        }
    }
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dba::B0x0
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dba::B0x1
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Dba::B0x2
    }
}
#[doc = "Field `DBA` writer - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
pub type DbaW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dba>;
impl<'a, REG> DbaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dba::B0x0)
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dba::B0x1)
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Dba::B0x2)
    }
}
#[doc = "DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes &amp; DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbl {
    #[doc = "0: 1 transfer"]
    B0x0 = 0,
    #[doc = "1: 2 transfers"]
    B0x1 = 1,
    #[doc = "2: 3 transfers"]
    B0x2 = 2,
    #[doc = "17: 18 transfers"]
    B0x11 = 17,
}
impl From<Dbl> for u8 {
    #[inline(always)]
    fn from(variant: Dbl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbl {
    type Ux = u8;
}
impl crate::IsEnum for Dbl {}
#[doc = "Field `DBL` reader - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes &amp; DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
pub type DblR = crate::FieldReader<Dbl>;
impl DblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dbl> {
        match self.bits {
            0 => Some(Dbl::B0x0),
            1 => Some(Dbl::B0x1),
            2 => Some(Dbl::B0x2),
            17 => Some(Dbl::B0x11),
            _ => None,
        }
    }
    #[doc = "1 transfer"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dbl::B0x0
    }
    #[doc = "2 transfers"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dbl::B0x1
    }
    #[doc = "3 transfers"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Dbl::B0x2
    }
    #[doc = "18 transfers"]
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == Dbl::B0x11
    }
}
#[doc = "Field `DBL` writer - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes &amp; DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
pub type DblW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dbl>;
impl<'a, REG> DblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 transfer"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbl::B0x0)
    }
    #[doc = "2 transfers"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbl::B0x1)
    }
    #[doc = "3 transfers"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Dbl::B0x2)
    }
    #[doc = "18 transfers"]
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Dbl::B0x11)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
    #[inline(always)]
    pub fn dba(&self) -> DbaR {
        DbaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes &amp; DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
    #[inline(always)]
    pub fn dbl(&self) -> DblR {
        DblR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DbaW<Tim1DcrSpec> {
        DbaW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes &amp; DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DblW<Tim1DcrSpec> {
        DblW::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1DcrSpec;
impl crate::RegisterSpec for Tim1DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_dcr::R`](R) reader structure"]
impl crate::Readable for Tim1DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_dcr::W`](W) writer structure"]
impl crate::Writable for Tim1DcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_DCR to value 0"]
impl crate::Resettable for Tim1DcrSpec {
    const RESET_VALUE: u32 = 0;
}
