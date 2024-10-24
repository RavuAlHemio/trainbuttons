#[doc = "Register `CHSELR_1` reader"]
pub type R = crate::R<Chselr1Spec>;
#[doc = "Register `CHSELR_1` writer"]
pub type W = crate::W<Chselr1Spec>;
#[doc = "Field `SQ1` reader - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq1R = crate::FieldReader;
#[doc = "Field `SQ1` writer - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ2` reader - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq2R = crate::FieldReader;
#[doc = "Field `SQ2` writer - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ3` reader - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq3R = crate::FieldReader;
#[doc = "Field `SQ3` writer - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ4` reader - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq4R = crate::FieldReader;
#[doc = "Field `SQ4` writer - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ5` reader - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq5R = crate::FieldReader;
#[doc = "Field `SQ5` writer - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ6` reader - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq6R = crate::FieldReader;
#[doc = "Field `SQ6` writer - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ7` reader - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq7R = crate::FieldReader;
#[doc = "Field `SQ7` writer - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sq8 {
    #[doc = "0: CH0"]
    B0x0 = 0,
    #[doc = "1: CH1"]
    B0x1 = 1,
    #[doc = "12: CH12"]
    B0xC = 12,
    #[doc = "13: CH13"]
    B0xD = 13,
    #[doc = "14: CH14"]
    B0xE = 14,
    #[doc = "15: No channel selected (End of sequence)"]
    B0xF = 15,
}
impl From<Sq8> for u8 {
    #[inline(always)]
    fn from(variant: Sq8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sq8 {
    type Ux = u8;
}
impl crate::IsEnum for Sq8 {}
#[doc = "Field `SQ8` reader - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq8R = crate::FieldReader<Sq8>;
impl Sq8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sq8> {
        match self.bits {
            0 => Some(Sq8::B0x0),
            1 => Some(Sq8::B0x1),
            12 => Some(Sq8::B0xC),
            13 => Some(Sq8::B0xD),
            14 => Some(Sq8::B0xE),
            15 => Some(Sq8::B0xF),
            _ => None,
        }
    }
    #[doc = "CH0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sq8::B0x0
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sq8::B0x1
    }
    #[doc = "CH12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Sq8::B0xC
    }
    #[doc = "CH13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Sq8::B0xD
    }
    #[doc = "CH14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Sq8::B0xE
    }
    #[doc = "No channel selected (End of sequence)"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Sq8::B0xF
    }
}
#[doc = "Field `SQ8` writer - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type Sq8W<'a, REG> = crate::FieldWriter<'a, REG, 4, Sq8>;
impl<'a, REG> Sq8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sq8::B0x0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sq8::B0x1)
    }
    #[doc = "CH12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Sq8::B0xC)
    }
    #[doc = "CH13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Sq8::B0xD)
    }
    #[doc = "CH14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Sq8::B0xE)
    }
    #[doc = "No channel selected (End of sequence)"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sq8::B0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq1(&self) -> Sq1R {
        Sq1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq2(&self) -> Sq2R {
        Sq2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq3(&self) -> Sq3R {
        Sq3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq4(&self) -> Sq4R {
        Sq4R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq5(&self) -> Sq5R {
        Sq5R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq6(&self) -> Sq6R {
        Sq6R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq7(&self) -> Sq7R {
        Sq7R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq8(&self) -> Sq8R {
        Sq8R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> Sq1W<Chselr1Spec> {
        Sq1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> Sq2W<Chselr1Spec> {
        Sq2W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> Sq3W<Chselr1Spec> {
        Sq3W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> Sq4W<Chselr1Spec> {
        Sq4W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> Sq5W<Chselr1Spec> {
        Sq5W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> Sq6W<Chselr1Spec> {
        Sq6W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> Sq7W<Chselr1Spec> {
        Sq7W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> Sq8W<Chselr1Spec> {
        Sq8W::new(self, 28)
    }
}
#[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`chselr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chselr1Spec;
impl crate::RegisterSpec for Chselr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr_1::R`](R) reader structure"]
impl crate::Readable for Chselr1Spec {}
#[doc = "`write(|w| ..)` method takes [`chselr_1::W`](W) writer structure"]
impl crate::Writable for Chselr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSELR_1 to value 0"]
impl crate::Resettable for Chselr1Spec {
    const RESET_VALUE: u32 = 0;
}
