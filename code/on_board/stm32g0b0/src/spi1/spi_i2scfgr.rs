#[doc = "Register `SPI_I2SCFGR` reader"]
pub type R = crate::R<SpiI2scfgrSpec>;
#[doc = "Register `SPI_I2SCFGR` writer"]
pub type W = crate::W<SpiI2scfgrSpec>;
#[doc = "Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chlen {
    #[doc = "0: 16-bit wide"]
    B0x0 = 0,
    #[doc = "1: 32-bit wide"]
    B0x1 = 1,
}
impl From<Chlen> for bool {
    #[inline(always)]
    fn from(variant: Chlen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type ChlenR = crate::BitReader<Chlen>;
impl ChlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chlen {
        match self.bits {
            false => Chlen::B0x0,
            true => Chlen::B0x1,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chlen::B0x0
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chlen::B0x1
    }
}
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG, Chlen>;
impl<'a, REG> ChlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chlen::B0x0)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chlen::B0x1)
    }
}
#[doc = "Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datlen {
    #[doc = "0: 16-bit data length"]
    B0x0 = 0,
    #[doc = "1: 24-bit data length"]
    B0x1 = 1,
    #[doc = "2: 32-bit data length"]
    B0x2 = 2,
    #[doc = "3: Not allowed"]
    B0x3 = 3,
}
impl From<Datlen> for u8 {
    #[inline(always)]
    fn from(variant: Datlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datlen {
    type Ux = u8;
}
impl crate::IsEnum for Datlen {}
#[doc = "Field `DATLEN` reader - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DatlenR = crate::FieldReader<Datlen>;
impl DatlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datlen {
        match self.bits {
            0 => Datlen::B0x0,
            1 => Datlen::B0x1,
            2 => Datlen::B0x2,
            3 => Datlen::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Datlen::B0x0
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Datlen::B0x1
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Datlen::B0x2
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Datlen::B0x3
    }
}
#[doc = "Field `DATLEN` writer - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DatlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datlen, crate::Safe>;
impl<'a, REG> DatlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Datlen::B0x0)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Datlen::B0x1)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Datlen::B0x2)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Datlen::B0x3)
    }
}
#[doc = "Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckpol {
    #[doc = "0: I2S clock inactive state is low level"]
    B0x0 = 0,
    #[doc = "1: I2S clock inactive state is high level"]
    B0x1 = 1,
}
impl From<Ckpol> for bool {
    #[inline(always)]
    fn from(variant: Ckpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CkpolR = crate::BitReader<Ckpol>;
impl CkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckpol {
        match self.bits {
            false => Ckpol::B0x0,
            true => Ckpol::B0x1,
        }
    }
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ckpol::B0x0
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ckpol::B0x1
    }
}
#[doc = "Field `CKPOL` writer - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CkpolW<'a, REG> = crate::BitWriter<'a, REG, Ckpol>;
impl<'a, REG> CkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::B0x0)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::B0x1)
    }
}
#[doc = "I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2sstd {
    #[doc = "0: I2S Philips standard"]
    B0x0 = 0,
    #[doc = "1: MSB justified standard (left justified)"]
    B0x1 = 1,
    #[doc = "2: LSB justified standard (right justified)"]
    B0x2 = 2,
    #[doc = "3: PCM standard"]
    B0x3 = 3,
}
impl From<I2sstd> for u8 {
    #[inline(always)]
    fn from(variant: I2sstd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2sstd {
    type Ux = u8;
}
impl crate::IsEnum for I2sstd {}
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2sstdR = crate::FieldReader<I2sstd>;
impl I2sstdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2sstd {
        match self.bits {
            0 => I2sstd::B0x0,
            1 => I2sstd::B0x1,
            2 => I2sstd::B0x2,
            3 => I2sstd::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2sstd::B0x0
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2sstd::B0x1
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2sstd::B0x2
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2sstd::B0x3
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2sstdW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2sstd, crate::Safe>;
impl<'a, REG> I2sstdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2sstd::B0x0)
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2sstd::B0x1)
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2sstd::B0x2)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2sstd::B0x3)
    }
}
#[doc = "PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcmsync {
    #[doc = "0: Short frame synchronization"]
    B0x0 = 0,
    #[doc = "1: Long frame synchronization"]
    B0x1 = 1,
}
impl From<Pcmsync> for bool {
    #[inline(always)]
    fn from(variant: Pcmsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PcmsyncR = crate::BitReader<Pcmsync>;
impl PcmsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcmsync {
        match self.bits {
            false => Pcmsync::B0x0,
            true => Pcmsync::B0x1,
        }
    }
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pcmsync::B0x0
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pcmsync::B0x1
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PcmsyncW<'a, REG> = crate::BitWriter<'a, REG, Pcmsync>;
impl<'a, REG> PcmsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmsync::B0x0)
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmsync::B0x1)
    }
}
#[doc = "I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2scfg {
    #[doc = "0: Slave - transmit"]
    B0x0 = 0,
    #[doc = "1: Slave - receive"]
    B0x1 = 1,
    #[doc = "2: Master - transmit"]
    B0x2 = 2,
    #[doc = "3: Master - receive"]
    B0x3 = 3,
}
impl From<I2scfg> for u8 {
    #[inline(always)]
    fn from(variant: I2scfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2scfg {
    type Ux = u8;
}
impl crate::IsEnum for I2scfg {}
#[doc = "Field `I2SCFG` reader - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2scfgR = crate::FieldReader<I2scfg>;
impl I2scfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2scfg {
        match self.bits {
            0 => I2scfg::B0x0,
            1 => I2scfg::B0x1,
            2 => I2scfg::B0x2,
            3 => I2scfg::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2scfg::B0x0
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2scfg::B0x1
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2scfg::B0x2
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2scfg::B0x3
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2scfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2scfg, crate::Safe>;
impl<'a, REG> I2scfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2scfg::B0x0)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2scfg::B0x1)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2scfg::B0x2)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2scfg::B0x3)
    }
}
#[doc = "I2S enable Note: This bit is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2se {
    #[doc = "0: I2S peripheral is disabled"]
    B0x0 = 0,
    #[doc = "1: I2S peripheral is enabled"]
    B0x1 = 1,
}
impl From<I2se> for bool {
    #[inline(always)]
    fn from(variant: I2se) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SE` reader - I2S enable Note: This bit is not used in SPI mode."]
pub type I2seR = crate::BitReader<I2se>;
impl I2seR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2se {
        match self.bits {
            false => I2se::B0x0,
            true => I2se::B0x1,
        }
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2se::B0x0
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2se::B0x1
    }
}
#[doc = "Field `I2SE` writer - I2S enable Note: This bit is not used in SPI mode."]
pub type I2seW<'a, REG> = crate::BitWriter<'a, REG, I2se>;
impl<'a, REG> I2seW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2se::B0x0)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2se::B0x1)
    }
}
#[doc = "I2S mode selection Note: This bit should be configured when the SPI is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2smod {
    #[doc = "0: SPI mode is selected"]
    B0x0 = 0,
    #[doc = "1: I2S mode is selected"]
    B0x1 = 1,
}
impl From<I2smod> for bool {
    #[inline(always)]
    fn from(variant: I2smod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2smodR = crate::BitReader<I2smod>;
impl I2smodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2smod {
        match self.bits {
            false => I2smod::B0x0,
            true => I2smod::B0x1,
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2smod::B0x0
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2smod::B0x1
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2smodW<'a, REG> = crate::BitWriter<'a, REG, I2smod>;
impl<'a, REG> I2smodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2smod::B0x0)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2smod::B0x1)
    }
}
#[doc = "Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Astrten {
    #[doc = "0: The Asynchronous start is disabled."]
    B0x0 = 0,
    #[doc = "1: The Asynchronous start is enabled."]
    B0x1 = 1,
}
impl From<Astrten> for bool {
    #[inline(always)]
    fn from(variant: Astrten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASTRTEN` reader - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type AstrtenR = crate::BitReader<Astrten>;
impl AstrtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Astrten {
        match self.bits {
            false => Astrten::B0x0,
            true => Astrten::B0x1,
        }
    }
    #[doc = "The Asynchronous start is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Astrten::B0x0
    }
    #[doc = "The Asynchronous start is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Astrten::B0x1
    }
}
#[doc = "Field `ASTRTEN` writer - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type AstrtenW<'a, REG> = crate::BitWriter<'a, REG, Astrten>;
impl<'a, REG> AstrtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Asynchronous start is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Astrten::B0x0)
    }
    #[doc = "The Asynchronous start is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Astrten::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn datlen(&self) -> DatlenR {
        DatlenR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2sstdR {
        I2sstdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    pub fn pcmsync(&self) -> PcmsyncR {
        PcmsyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2scfgR {
        I2scfgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn i2se(&self) -> I2seR {
        I2seR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    pub fn i2smod(&self) -> I2smodR {
        I2smodR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    pub fn astrten(&self) -> AstrtenR {
        AstrtenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> ChlenW<SpiI2scfgrSpec> {
        ChlenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DatlenW<SpiI2scfgrSpec> {
        DatlenW::new(self, 1)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CkpolW<SpiI2scfgrSpec> {
        CkpolW::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2sstdW<SpiI2scfgrSpec> {
        I2sstdW::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PcmsyncW<SpiI2scfgrSpec> {
        PcmsyncW::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2scfgW<SpiI2scfgrSpec> {
        I2scfgW::new(self, 8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2seW<SpiI2scfgrSpec> {
        I2seW::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2smodW<SpiI2scfgrSpec> {
        I2smodW::new(self, 11)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn astrten(&mut self) -> AstrtenW<SpiI2scfgrSpec> {
        AstrtenW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiI2scfgrSpec;
impl crate::RegisterSpec for SpiI2scfgrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_i2scfgr::R`](R) reader structure"]
impl crate::Readable for SpiI2scfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_i2scfgr::W`](W) writer structure"]
impl crate::Writable for SpiI2scfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_I2SCFGR to value 0"]
impl crate::Resettable for SpiI2scfgrSpec {
    const RESET_VALUE: u16 = 0;
}
