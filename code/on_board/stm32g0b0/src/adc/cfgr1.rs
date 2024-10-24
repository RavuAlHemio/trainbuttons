#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA disabled"]
    B0x0 = 0,
    #[doc = "1: DMA enabled"]
    B0x1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::B0x0,
            true => Dmaen::B0x1,
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmaen::B0x0
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmaen::B0x1
    }
}
#[doc = "Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B0x0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B0x1)
    }
}
#[doc = "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmacfg {
    #[doc = "0: DMA one shot mode selected"]
    OneShot = 0,
    #[doc = "1: DMA circular mode selected"]
    Circular = 1,
}
impl From<Dmacfg> for bool {
    #[inline(always)]
    fn from(variant: Dmacfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DmacfgR = crate::BitReader<Dmacfg>;
impl DmacfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmacfg {
        match self.bits {
            false => Dmacfg::OneShot,
            true => Dmacfg::Circular,
        }
    }
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == Dmacfg::OneShot
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == Dmacfg::Circular
    }
}
#[doc = "Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DmacfgW<'a, REG> = crate::BitWriter<'a, REG, Dmacfg>;
impl<'a, REG> DmacfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(Dmacfg::OneShot)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut crate::W<REG> {
        self.variant(Dmacfg::Circular)
    }
}
#[doc = "Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scandir {
    #[doc = "0: Upward scan (from CHSEL0 to CHSEL18)"]
    Upward = 0,
    #[doc = "1: Backward scan (from CHSEL18 to CHSEL0)"]
    Backward = 1,
}
impl From<Scandir> for bool {
    #[inline(always)]
    fn from(variant: Scandir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCANDIR` reader - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type ScandirR = crate::BitReader<Scandir>;
impl ScandirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scandir {
        match self.bits {
            false => Scandir::Upward,
            true => Scandir::Backward,
        }
    }
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        *self == Scandir::Upward
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        *self == Scandir::Backward
    }
}
#[doc = "Field `SCANDIR` writer - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type ScandirW<'a, REG> = crate::BitWriter<'a, REG, Scandir>;
impl<'a, REG> ScandirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn upward(self) -> &'a mut crate::W<REG> {
        self.variant(Scandir::Upward)
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn backward(self) -> &'a mut crate::W<REG> {
        self.variant(Scandir::Backward)
    }
}
#[doc = "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Res {
    #[doc = "0: 12 bits"]
    Bits12 = 0,
    #[doc = "1: 10 bits"]
    Bits10 = 1,
    #[doc = "2: 8 bits"]
    Bits8 = 2,
    #[doc = "3: 6 bits"]
    Bits6 = 3,
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(variant: Res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Res {
    type Ux = u8;
}
impl crate::IsEnum for Res {}
#[doc = "Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
pub type ResR = crate::FieldReader<Res>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Res {
        match self.bits {
            0 => Res::Bits12,
            1 => Res::Bits10,
            2 => Res::Bits8,
            3 => Res::Bits6,
            _ => unreachable!(),
        }
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn is_bits_12(&self) -> bool {
        *self == Res::Bits12
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        *self == Res::Bits10
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == Res::Bits8
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_bits_6(&self) -> bool {
        *self == Res::Bits6
    }
}
#[doc = "Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, Res, crate::Safe>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn bits_12(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bits12)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bits10)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bits8)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn bits_6(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bits6)
    }
}
#[doc = "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
    Left = 1,
}
impl From<Align> for bool {
    #[inline(always)]
    fn from(variant: Align) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AlignR = crate::BitReader<Align>;
impl AlignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Align {
        match self.bits {
            false => Align::Right,
            true => Align::Left,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Align::Right
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Align::Left
    }
}
#[doc = "Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG, Align>;
impl<'a, REG> AlignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Align::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Align::Left)
    }
}
#[doc = "External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extsel {
    #[doc = "0: TRG0"]
    B0x0 = 0,
    #[doc = "1: TRG1"]
    B0x1 = 1,
    #[doc = "2: TRG2"]
    B0x2 = 2,
    #[doc = "3: TRG3"]
    B0x3 = 3,
    #[doc = "4: TRG4"]
    B0x4 = 4,
    #[doc = "5: TRG5"]
    B0x5 = 5,
    #[doc = "6: TRG6"]
    B0x6 = 6,
    #[doc = "7: TRG7"]
    B0x7 = 7,
}
impl From<Extsel> for u8 {
    #[inline(always)]
    fn from(variant: Extsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extsel {
    type Ux = u8;
}
impl crate::IsEnum for Extsel {}
#[doc = "Field `EXTSEL` reader - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ExtselR = crate::FieldReader<Extsel>;
impl ExtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extsel {
        match self.bits {
            0 => Extsel::B0x0,
            1 => Extsel::B0x1,
            2 => Extsel::B0x2,
            3 => Extsel::B0x3,
            4 => Extsel::B0x4,
            5 => Extsel::B0x5,
            6 => Extsel::B0x6,
            7 => Extsel::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TRG0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Extsel::B0x0
    }
    #[doc = "TRG1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Extsel::B0x1
    }
    #[doc = "TRG2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Extsel::B0x2
    }
    #[doc = "TRG3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Extsel::B0x3
    }
    #[doc = "TRG4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Extsel::B0x4
    }
    #[doc = "TRG5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Extsel::B0x5
    }
    #[doc = "TRG6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Extsel::B0x6
    }
    #[doc = "TRG7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Extsel::B0x7
    }
}
#[doc = "Field `EXTSEL` writer - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ExtselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Extsel, crate::Safe>;
impl<'a, REG> ExtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TRG0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x0)
    }
    #[doc = "TRG1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x1)
    }
    #[doc = "TRG2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x2)
    }
    #[doc = "TRG3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x3)
    }
    #[doc = "TRG4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x4)
    }
    #[doc = "TRG5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x5)
    }
    #[doc = "TRG6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x6)
    }
    #[doc = "TRG7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Extsel::B0x7)
    }
}
#[doc = "External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exten {
    #[doc = "0: Hardware trigger detection disabled (conversions can be started by software)"]
    Software = 0,
    #[doc = "1: Hardware trigger detection on the rising edge"]
    HardwareRising = 1,
    #[doc = "2: Hardware trigger detection on the falling edge"]
    HardwareFalling = 2,
    #[doc = "3: Hardware trigger detection on both the rising and falling edges"]
    HardwareBoth = 3,
}
impl From<Exten> for u8 {
    #[inline(always)]
    fn from(variant: Exten) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exten {
    type Ux = u8;
}
impl crate::IsEnum for Exten {}
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ExtenR = crate::FieldReader<Exten>;
impl ExtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exten {
        match self.bits {
            0 => Exten::Software,
            1 => Exten::HardwareRising,
            2 => Exten::HardwareFalling,
            3 => Exten::HardwareBoth,
            _ => unreachable!(),
        }
    }
    #[doc = "Hardware trigger detection disabled (conversions can be started by software)"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Exten::Software
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_hardware_rising(&self) -> bool {
        *self == Exten::HardwareRising
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_hardware_falling(&self) -> bool {
        *self == Exten::HardwareFalling
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_hardware_both(&self) -> bool {
        *self == Exten::HardwareBoth
    }
}
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ExtenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Exten, crate::Safe>;
impl<'a, REG> ExtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hardware trigger detection disabled (conversions can be started by software)"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Exten::Software)
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn hardware_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Exten::HardwareRising)
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn hardware_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Exten::HardwareFalling)
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn hardware_both(self) -> &'a mut crate::W<REG> {
        self.variant(Exten::HardwareBoth)
    }
}
#[doc = "Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrmod {
    #[doc = "0: ADC_DR register is preserved with the old data when an overrun is detected."]
    B0x0 = 0,
    #[doc = "1: ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    B0x1 = 1,
}
impl From<Ovrmod> for bool {
    #[inline(always)]
    fn from(variant: Ovrmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRMOD` reader - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OvrmodR = crate::BitReader<Ovrmod>;
impl OvrmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrmod {
        match self.bits {
            false => Ovrmod::B0x0,
            true => Ovrmod::B0x1,
        }
    }
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovrmod::B0x0
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovrmod::B0x1
    }
}
#[doc = "Field `OVRMOD` writer - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OvrmodW<'a, REG> = crate::BitWriter<'a, REG, Ovrmod>;
impl<'a, REG> OvrmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrmod::B0x0)
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrmod::B0x1)
    }
}
#[doc = "Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cont {
    #[doc = "0: Single conversion mode"]
    B0x0 = 0,
    #[doc = "1: Continuous conversion mode"]
    B0x1 = 1,
}
impl From<Cont> for bool {
    #[inline(always)]
    fn from(variant: Cont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ContR = crate::BitReader<Cont>;
impl ContR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cont {
        match self.bits {
            false => Cont::B0x0,
            true => Cont::B0x1,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cont::B0x0
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cont::B0x1
    }
}
#[doc = "Field `CONT` writer - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG, Cont>;
impl<'a, REG> ContW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cont::B0x0)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cont::B0x1)
    }
}
#[doc = "Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wait {
    #[doc = "0: Wait conversion mode off"]
    B0x0 = 0,
    #[doc = "1: Wait conversion mode on"]
    B0x1 = 1,
}
impl From<Wait> for bool {
    #[inline(always)]
    fn from(variant: Wait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT` reader - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WaitR = crate::BitReader<Wait>;
impl WaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wait {
        match self.bits {
            false => Wait::B0x0,
            true => Wait::B0x1,
        }
    }
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wait::B0x0
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wait::B0x1
    }
}
#[doc = "Field `WAIT` writer - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WaitW<'a, REG> = crate::BitWriter<'a, REG, Wait>;
impl<'a, REG> WaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::B0x0)
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::B0x1)
    }
}
#[doc = "Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoff {
    #[doc = "0: Auto-off mode disabled"]
    B0x0 = 0,
    #[doc = "1: Auto-off mode enabled"]
    B0x1 = 1,
}
impl From<Autoff> for bool {
    #[inline(always)]
    fn from(variant: Autoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOFF` reader - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AutoffR = crate::BitReader<Autoff>;
impl AutoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoff {
        match self.bits {
            false => Autoff::B0x0,
            true => Autoff::B0x1,
        }
    }
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Autoff::B0x0
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Autoff::B0x1
    }
}
#[doc = "Field `AUTOFF` writer - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AutoffW<'a, REG> = crate::BitWriter<'a, REG, Autoff>;
impl<'a, REG> AutoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Autoff::B0x0)
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Autoff::B0x1)
    }
}
#[doc = "Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Discen {
    #[doc = "0: Discontinuous mode disabled"]
    B0x0 = 0,
    #[doc = "1: Discontinuous mode enabled"]
    B0x1 = 1,
}
impl From<Discen> for bool {
    #[inline(always)]
    fn from(variant: Discen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DiscenR = crate::BitReader<Discen>;
impl DiscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Discen {
        match self.bits {
            false => Discen::B0x0,
            true => Discen::B0x1,
        }
    }
    #[doc = "Discontinuous mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Discen::B0x0
    }
    #[doc = "Discontinuous mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Discen::B0x1
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DiscenW<'a, REG> = crate::BitWriter<'a, REG, Discen>;
impl<'a, REG> DiscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Discen::B0x0)
    }
    #[doc = "Discontinuous mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Discen::B0x1)
    }
}
#[doc = "Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselrmod {
    #[doc = "0: Each bit of the ADC_CHSELR register enables an input"]
    NumericOrder = 0,
    #[doc = "1: ADC_CHSELR register is able to sequence up to 8 channels"]
    CustomOrder = 1,
}
impl From<Chselrmod> for bool {
    #[inline(always)]
    fn from(variant: Chselrmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type ChselrmodR = crate::BitReader<Chselrmod>;
impl ChselrmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselrmod {
        match self.bits {
            false => Chselrmod::NumericOrder,
            true => Chselrmod::CustomOrder,
        }
    }
    #[doc = "Each bit of the ADC_CHSELR register enables an input"]
    #[inline(always)]
    pub fn is_numeric_order(&self) -> bool {
        *self == Chselrmod::NumericOrder
    }
    #[doc = "ADC_CHSELR register is able to sequence up to 8 channels"]
    #[inline(always)]
    pub fn is_custom_order(&self) -> bool {
        *self == Chselrmod::CustomOrder
    }
}
#[doc = "Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type ChselrmodW<'a, REG> = crate::BitWriter<'a, REG, Chselrmod>;
impl<'a, REG> ChselrmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Each bit of the ADC_CHSELR register enables an input"]
    #[inline(always)]
    pub fn numeric_order(self) -> &'a mut crate::W<REG> {
        self.variant(Chselrmod::NumericOrder)
    }
    #[doc = "ADC_CHSELR register is able to sequence up to 8 channels"]
    #[inline(always)]
    pub fn custom_order(self) -> &'a mut crate::W<REG> {
        self.variant(Chselrmod::CustomOrder)
    }
}
#[doc = "Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd1sgl {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog 1 enabled on a single channel"]
    B0x1 = 1,
}
impl From<Awd1sgl> for bool {
    #[inline(always)]
    fn from(variant: Awd1sgl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1SGL` reader - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1sglR = crate::BitReader<Awd1sgl>;
impl Awd1sglR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd1sgl {
        match self.bits {
            false => Awd1sgl::B0x0,
            true => Awd1sgl::B0x1,
        }
    }
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd1sgl::B0x0
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd1sgl::B0x1
    }
}
#[doc = "Field `AWD1SGL` writer - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1sglW<'a, REG> = crate::BitWriter<'a, REG, Awd1sgl>;
impl<'a, REG> Awd1sglW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1sgl::B0x0)
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1sgl::B0x1)
    }
}
#[doc = "Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd1en {
    #[doc = "0: Analog watchdog 1 disabled"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog 1 enabled"]
    B0x1 = 1,
}
impl From<Awd1en> for bool {
    #[inline(always)]
    fn from(variant: Awd1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1EN` reader - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1enR = crate::BitReader<Awd1en>;
impl Awd1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd1en {
        match self.bits {
            false => Awd1en::B0x0,
            true => Awd1en::B0x1,
        }
    }
    #[doc = "Analog watchdog 1 disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd1en::B0x0
    }
    #[doc = "Analog watchdog 1 enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd1en::B0x1
    }
}
#[doc = "Field `AWD1EN` writer - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1enW<'a, REG> = crate::BitWriter<'a, REG, Awd1en>;
impl<'a, REG> Awd1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1en::B0x0)
    }
    #[doc = "Analog watchdog 1 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1en::B0x1)
    }
}
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Awd1ch {
    #[doc = "0: ADC analog input Channel 0 monitored by AWD"]
    B0x0 = 0,
    #[doc = "1: ADC analog input Channel 1 monitored by AWD"]
    B0x1 = 1,
    #[doc = "17: ADC analog input Channel 17 monitored by AWD"]
    B0x11 = 17,
    #[doc = "18: ADC analog input Channel 18 monitored by AWD"]
    B0x12 = 18,
}
impl From<Awd1ch> for u8 {
    #[inline(always)]
    fn from(variant: Awd1ch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Awd1ch {
    type Ux = u8;
}
impl crate::IsEnum for Awd1ch {}
#[doc = "Field `AWD1CH` reader - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1chR = crate::FieldReader<Awd1ch>;
impl Awd1chR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Awd1ch> {
        match self.bits {
            0 => Some(Awd1ch::B0x0),
            1 => Some(Awd1ch::B0x1),
            17 => Some(Awd1ch::B0x11),
            18 => Some(Awd1ch::B0x12),
            _ => None,
        }
    }
    #[doc = "ADC analog input Channel 0 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd1ch::B0x0
    }
    #[doc = "ADC analog input Channel 1 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd1ch::B0x1
    }
    #[doc = "ADC analog input Channel 17 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == Awd1ch::B0x11
    }
    #[doc = "ADC analog input Channel 18 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x12(&self) -> bool {
        *self == Awd1ch::B0x12
    }
}
#[doc = "Field `AWD1CH` writer - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type Awd1chW<'a, REG> = crate::FieldWriter<'a, REG, 5, Awd1ch>;
impl<'a, REG> Awd1chW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC analog input Channel 0 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1ch::B0x0)
    }
    #[doc = "ADC analog input Channel 1 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1ch::B0x1)
    }
    #[doc = "ADC analog input Channel 17 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1ch::B0x11)
    }
    #[doc = "ADC analog input Channel 18 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x12(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1ch::B0x12)
    }
}
impl R {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmacfg(&self) -> DmacfgR {
        DmacfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn scandir(&self) -> ScandirR {
        ScandirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel(&self) -> ExtselR {
        ExtselR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrmod(&self) -> OvrmodR {
        OvrmodR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn autoff(&self) -> AutoffR {
        AutoffR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn discen(&self) -> DiscenR {
        DiscenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chselrmod(&self) -> ChselrmodR {
        ChselrmodR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1sgl(&self) -> Awd1sglR {
        Awd1sglR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1en(&self) -> Awd1enR {
        Awd1enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ch(&self) -> Awd1chR {
        Awd1chR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<Cfgr1Spec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DmacfgW<Cfgr1Spec> {
        DmacfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> ScandirW<Cfgr1Spec> {
        ScandirW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Cfgr1Spec> {
        ResW::new(self, 3)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> AlignW<Cfgr1Spec> {
        AlignW::new(self, 5)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> ExtselW<Cfgr1Spec> {
        ExtselW::new(self, 6)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> ExtenW<Cfgr1Spec> {
        ExtenW::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OvrmodW<Cfgr1Spec> {
        OvrmodW::new(self, 12)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<Cfgr1Spec> {
        ContW::new(self, 13)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WaitW<Cfgr1Spec> {
        WaitW::new(self, 14)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AutoffW<Cfgr1Spec> {
        AutoffW::new(self, 15)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DiscenW<Cfgr1Spec> {
        DiscenW::new(self, 16)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chselrmod(&mut self) -> ChselrmodW<Cfgr1Spec> {
        ChselrmodW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> Awd1sglW<Cfgr1Spec> {
        Awd1sglW::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> Awd1enW<Cfgr1Spec> {
        Awd1enW::new(self, 23)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> Awd1chW<Cfgr1Spec> {
        Awd1chW::new(self, 26)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {
    const RESET_VALUE: u32 = 0;
}
