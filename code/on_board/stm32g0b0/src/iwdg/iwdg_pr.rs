#[doc = "Register `IWDG_PR` reader"]
pub type R = crate::R<IwdgPrSpec>;
#[doc = "Register `IWDG_PR` writer"]
pub type W = crate::W<IwdgPrSpec>;
#[doc = "Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pr {
    #[doc = "0: divider /4"]
    B0x0 = 0,
    #[doc = "1: divider /8"]
    B0x1 = 1,
    #[doc = "2: divider /16"]
    B0x2 = 2,
    #[doc = "3: divider /32"]
    B0x3 = 3,
    #[doc = "4: divider /64"]
    B0x4 = 4,
    #[doc = "5: divider /128"]
    B0x5 = 5,
    #[doc = "6: divider /256"]
    B0x6 = 6,
    #[doc = "7: divider /256"]
    B0x7 = 7,
}
impl From<Pr> for u8 {
    #[inline(always)]
    fn from(variant: Pr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pr {
    type Ux = u8;
}
impl crate::IsEnum for Pr {}
#[doc = "Field `PR` reader - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PrR = crate::FieldReader<Pr>;
impl PrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pr {
        match self.bits {
            0 => Pr::B0x0,
            1 => Pr::B0x1,
            2 => Pr::B0x2,
            3 => Pr::B0x3,
            4 => Pr::B0x4,
            5 => Pr::B0x5,
            6 => Pr::B0x6,
            7 => Pr::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "divider /4"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pr::B0x0
    }
    #[doc = "divider /8"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pr::B0x1
    }
    #[doc = "divider /16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pr::B0x2
    }
    #[doc = "divider /32"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pr::B0x3
    }
    #[doc = "divider /64"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pr::B0x4
    }
    #[doc = "divider /128"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pr::B0x5
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pr::B0x6
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pr::B0x7
    }
}
#[doc = "Field `PR` writer - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pr, crate::Safe>;
impl<'a, REG> PrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "divider /4"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x0)
    }
    #[doc = "divider /8"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x1)
    }
    #[doc = "divider /16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x2)
    }
    #[doc = "divider /32"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x3)
    }
    #[doc = "divider /64"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x4)
    }
    #[doc = "divider /128"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x5)
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x6)
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::B0x7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<IwdgPrSpec> {
        PrW::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgPrSpec;
impl crate::RegisterSpec for IwdgPrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_pr::R`](R) reader structure"]
impl crate::Readable for IwdgPrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_pr::W`](W) writer structure"]
impl crate::Writable for IwdgPrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_PR to value 0"]
impl crate::Resettable for IwdgPrSpec {
    const RESET_VALUE: u32 = 0;
}
