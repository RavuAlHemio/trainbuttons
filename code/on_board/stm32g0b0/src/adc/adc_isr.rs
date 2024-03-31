#[doc = "Register `ADC_ISR` reader"]
pub type R = crate::R<AdcIsrSpec>;
#[doc = "Register `ADC_ISR` writer"]
pub type W = crate::W<AdcIsrSpec>;
#[doc = "ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrdy {
    #[doc = "0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: ADC is ready to start conversion"]
    B0x1 = 1,
}
impl From<Adrdy> for bool {
    #[inline(always)]
    fn from(variant: Adrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type AdrdyR = crate::BitReader<Adrdy>;
impl AdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrdy {
        match self.bits {
            false => Adrdy::B0x0,
            true => Adrdy::B0x1,
        }
    }
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adrdy::B0x0
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adrdy::B0x1
    }
}
#[doc = "Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type AdrdyW<'a, REG> = crate::BitWriter<'a, REG, Adrdy>;
impl<'a, REG> AdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrdy::B0x0)
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adrdy::B0x1)
    }
}
#[doc = "End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosmp {
    #[doc = "0: Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: End of sampling phase reached"]
    B0x1 = 1,
}
impl From<Eosmp> for bool {
    #[inline(always)]
    fn from(variant: Eosmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
pub type EosmpR = crate::BitReader<Eosmp>;
impl EosmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosmp {
        match self.bits {
            false => Eosmp::B0x0,
            true => Eosmp::B0x1,
        }
    }
    #[doc = "Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eosmp::B0x0
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eosmp::B0x1
    }
}
#[doc = "Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
pub type EosmpW<'a, REG> = crate::BitWriter<'a, REG, Eosmp>;
impl<'a, REG> EosmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosmp::B0x0)
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosmp::B0x1)
    }
}
#[doc = "End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoc {
    #[doc = "0: Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: Channel conversion complete"]
    B0x1 = 1,
}
impl From<Eoc> for bool {
    #[inline(always)]
    fn from(variant: Eoc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
pub type EocR = crate::BitReader<Eoc>;
impl EocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoc {
        match self.bits {
            false => Eoc::B0x0,
            true => Eoc::B0x1,
        }
    }
    #[doc = "Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eoc::B0x0
    }
    #[doc = "Channel conversion complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eoc::B0x1
    }
}
#[doc = "Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG, Eoc>;
impl<'a, REG> EocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoc::B0x0)
    }
    #[doc = "Channel conversion complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoc::B0x1)
    }
}
#[doc = "End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eos {
    #[doc = "0: Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: Conversion sequence complete"]
    B0x1 = 1,
}
impl From<Eos> for bool {
    #[inline(always)]
    fn from(variant: Eos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` reader - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
pub type EosR = crate::BitReader<Eos>;
impl EosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eos {
        match self.bits {
            false => Eos::B0x0,
            true => Eos::B0x1,
        }
    }
    #[doc = "Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eos::B0x0
    }
    #[doc = "Conversion sequence complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eos::B0x1
    }
}
#[doc = "Field `EOS` writer - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG, Eos>;
impl<'a, REG> EosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eos::B0x0)
    }
    #[doc = "Conversion sequence complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eos::B0x1)
    }
}
#[doc = "ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovr {
    #[doc = "0: No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: Overrun has occurred"]
    B0x1 = 1,
}
impl From<Ovr> for bool {
    #[inline(always)]
    fn from(variant: Ovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OvrR = crate::BitReader<Ovr>;
impl OvrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovr {
        match self.bits {
            false => Ovr::B0x0,
            true => Ovr::B0x1,
        }
    }
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovr::B0x0
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovr::B0x1
    }
}
#[doc = "Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG, Ovr>;
impl<'a, REG> OvrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovr::B0x0)
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovr::B0x1)
    }
}
#[doc = "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd1 {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B0x1 = 1,
}
impl From<Awd1> for bool {
    #[inline(always)]
    fn from(variant: Awd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
pub type Awd1R = crate::BitReader<Awd1>;
impl Awd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd1 {
        match self.bits {
            false => Awd1::B0x0,
            true => Awd1::B0x1,
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd1::B0x0
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd1::B0x1
    }
}
#[doc = "Field `AWD1` writer - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
pub type Awd1W<'a, REG> = crate::BitWriter<'a, REG, Awd1>;
impl<'a, REG> Awd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1::B0x0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd1::B0x1)
    }
}
#[doc = "Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd2 {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B0x1 = 1,
}
impl From<Awd2> for bool {
    #[inline(always)]
    fn from(variant: Awd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2` reader - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
pub type Awd2R = crate::BitReader<Awd2>;
impl Awd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd2 {
        match self.bits {
            false => Awd2::B0x0,
            true => Awd2::B0x1,
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd2::B0x0
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd2::B0x1
    }
}
#[doc = "Field `AWD2` writer - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
pub type Awd2W<'a, REG> = crate::BitWriter<'a, REG, Awd2>;
impl<'a, REG> Awd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2::B0x0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd2::B0x1)
    }
}
#[doc = "Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awd3 {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B0x0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B0x1 = 1,
}
impl From<Awd3> for bool {
    #[inline(always)]
    fn from(variant: Awd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3` reader - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
pub type Awd3R = crate::BitReader<Awd3>;
impl Awd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awd3 {
        match self.bits {
            false => Awd3::B0x0,
            true => Awd3::B0x1,
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Awd3::B0x0
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Awd3::B0x1
    }
}
#[doc = "Field `AWD3` writer - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
pub type Awd3W<'a, REG> = crate::BitWriter<'a, REG, Awd3>;
impl<'a, REG> Awd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3::B0x0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Awd3::B0x1)
    }
}
#[doc = "End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocal {
    #[doc = "0: Calibration is not complete"]
    B0x0 = 0,
    #[doc = "1: Calibration is complete"]
    B0x1 = 1,
}
impl From<Eocal> for bool {
    #[inline(always)]
    fn from(variant: Eocal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCAL` reader - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
pub type EocalR = crate::BitReader<Eocal>;
impl EocalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocal {
        match self.bits {
            false => Eocal::B0x0,
            true => Eocal::B0x1,
        }
    }
    #[doc = "Calibration is not complete"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eocal::B0x0
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eocal::B0x1
    }
}
#[doc = "Field `EOCAL` writer - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
pub type EocalW<'a, REG> = crate::BitWriter<'a, REG, Eocal>;
impl<'a, REG> EocalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration is not complete"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocal::B0x0)
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocal::B0x1)
    }
}
#[doc = "Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccrdy {
    #[doc = "0: Channel configuration update not applied."]
    B0x0 = 0,
    #[doc = "1: Channel configuration update is applied."]
    B0x1 = 1,
}
impl From<Ccrdy> for bool {
    #[inline(always)]
    fn from(variant: Ccrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDY` reader - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
pub type CcrdyR = crate::BitReader<Ccrdy>;
impl CcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccrdy {
        match self.bits {
            false => Ccrdy::B0x0,
            true => Ccrdy::B0x1,
        }
    }
    #[doc = "Channel configuration update not applied."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccrdy::B0x0
    }
    #[doc = "Channel configuration update is applied."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccrdy::B0x1
    }
}
#[doc = "Field `CCRDY` writer - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
pub type CcrdyW<'a, REG> = crate::BitWriter<'a, REG, Ccrdy>;
impl<'a, REG> CcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel configuration update not applied."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrdy::B0x0)
    }
    #[doc = "Channel configuration update is applied."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrdy::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn adrdy(&self) -> AdrdyR {
        AdrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
    #[inline(always)]
    pub fn eosmp(&self) -> EosmpR {
        EosmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd1(&self) -> Awd1R {
        Awd1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
    #[inline(always)]
    pub fn awd2(&self) -> Awd2R {
        Awd2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd3(&self) -> Awd3R {
        Awd3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eocal(&self) -> EocalR {
        EocalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
    #[inline(always)]
    pub fn ccrdy(&self) -> CcrdyR {
        CcrdyR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> AdrdyW<AdcIsrSpec> {
        AdrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EosmpW<AdcIsrSpec> {
        EosmpW::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EocW<AdcIsrSpec> {
        EocW::new(self, 2)
    }
    #[doc = "Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EosW<AdcIsrSpec> {
        EosW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OvrW<AdcIsrSpec> {
        OvrW::new(self, 4)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> Awd1W<AdcIsrSpec> {
        Awd1W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> Awd2W<AdcIsrSpec> {
        Awd2W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> Awd3W<AdcIsrSpec> {
        Awd3W::new(self, 9)
    }
    #[doc = "Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EocalW<AdcIsrSpec> {
        EocalW::new(self, 11)
    }
    #[doc = "Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
    #[inline(always)]
    #[must_use]
    pub fn ccrdy(&mut self) -> CcrdyW<AdcIsrSpec> {
        CcrdyW::new(self, 13)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcIsrSpec;
impl crate::RegisterSpec for AdcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_isr::R`](R) reader structure"]
impl crate::Readable for AdcIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_isr::W`](W) writer structure"]
impl crate::Writable for AdcIsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_ISR to value 0"]
impl crate::Resettable for AdcIsrSpec {
    const RESET_VALUE: u32 = 0;
}
