#[doc = "Register `CCER` reader"]
pub type R = crate::R<CcerSpec>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CcerSpec>;
#[doc = "Capture/Compare 1 output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1e {
    #[doc = "0: Capture mode disabled / OC1 is not active"]
    B0x0 = 0,
    #[doc = "1: Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    B0x1 = 1,
}
impl From<Cc1e> for bool {
    #[inline(always)]
    fn from(variant: Cc1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable."]
pub type Cc1eR = crate::BitReader<Cc1e>;
impl Cc1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1e {
        match self.bits {
            false => Cc1e::B0x0,
            true => Cc1e::B0x1,
        }
    }
    #[doc = "Capture mode disabled / OC1 is not active"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1e::B0x0
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1e::B0x1
    }
}
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable."]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG, Cc1e>;
impl<'a, REG> Cc1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture mode disabled / OC1 is not active"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1e::B0x0)
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1e::B0x1)
    }
}
#[doc = "Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1p {
    #[doc = "0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    B0x0 = 0,
    #[doc = "1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    B0x1 = 1,
}
impl From<Cc1p> for bool {
    #[inline(always)]
    fn from(variant: Cc1p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1P` reader - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used."]
pub type Cc1pR = crate::BitReader<Cc1p>;
impl Cc1pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1p {
        match self.bits {
            false => Cc1p::B0x0,
            true => Cc1p::B0x1,
        }
    }
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1p::B0x0
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1p::B0x1
    }
}
#[doc = "Field `CC1P` writer - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used."]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG, Cc1p>;
impl<'a, REG> Cc1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x0)
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x1)
    }
}
#[doc = "Field `CC1NP` reader - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description."]
pub type Cc1npR = crate::BitReader;
#[doc = "Field `CC1NP` writer - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description."]
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable. Refer to CC1E description"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable. Refer to CC1E description"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output Polarity. refer to CC1P description"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output Polarity. refer to CC1P description"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable. Refer to CC1E description"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable. Refer to CC1E description"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Capture/Compare 3 output Polarity. Refer to CC1P description"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - Capture/Compare 3 output Polarity. Refer to CC1P description"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
pub type Cc3npR = crate::BitReader;
#[doc = "Field `CC3NP` writer - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable. refer to CC1E description"]
pub type Cc4eR = crate::BitReader;
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable. refer to CC1E description"]
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Capture/Compare 4 output Polarity. Refer to CC1P description"]
pub type Cc4pR = crate::BitReader;
#[doc = "Field `CC4P` writer - Capture/Compare 4 output Polarity. Refer to CC1P description"]
pub type Cc4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
pub type Cc4npR = crate::BitReader;
#[doc = "Field `CC4NP` writer - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
pub type Cc4npW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable."]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used."]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description."]
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable. refer to CC1E description"]
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc4np(&self) -> Cc4npR {
        Cc4npR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable."]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> Cc1eW<CcerSpec> {
        Cc1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used."]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> Cc1pW<CcerSpec> {
        Cc1pW::new(self, 1)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description."]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> Cc1npW<CcerSpec> {
        Cc1npW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> Cc2eW<CcerSpec> {
        Cc2eW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> Cc2pW<CcerSpec> {
        Cc2pW::new(self, 5)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> Cc2npW<CcerSpec> {
        Cc2npW::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> Cc3eW<CcerSpec> {
        Cc3eW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> Cc3pW<CcerSpec> {
        Cc3pW::new(self, 9)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> Cc3npW<CcerSpec> {
        Cc3npW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable. refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> Cc4eW<CcerSpec> {
        Cc4eW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> Cc4pW<CcerSpec> {
        Cc4pW::new(self, 13)
    }
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> Cc4npW<CcerSpec> {
        Cc4npW::new(self, 15)
    }
}
#[doc = "capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerSpec;
impl crate::RegisterSpec for CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CcerSpec {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CcerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CcerSpec {
    const RESET_VALUE: u32 = 0;
}
