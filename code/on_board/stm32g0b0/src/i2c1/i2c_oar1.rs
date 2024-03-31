#[doc = "Register `I2C_OAR1` reader"]
pub type R = crate::R<I2cOar1Spec>;
#[doc = "Register `I2C_OAR1` writer"]
pub type W = crate::W<I2cOar1Spec>;
#[doc = "Field `OA1` reader - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type Oa1R = crate::FieldReader<u16>;
#[doc = "Field `OA1` writer - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type Oa1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oa1mode {
    #[doc = "0: Own address 1 is a 7-bit address."]
    B0x0 = 0,
    #[doc = "1: Own address 1 is a 10-bit address."]
    B0x1 = 1,
}
impl From<Oa1mode> for bool {
    #[inline(always)]
    fn from(variant: Oa1mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type Oa1modeR = crate::BitReader<Oa1mode>;
impl Oa1modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa1mode {
        match self.bits {
            false => Oa1mode::B0x0,
            true => Oa1mode::B0x1,
        }
    }
    #[doc = "Own address 1 is a 7-bit address."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oa1mode::B0x0
    }
    #[doc = "Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oa1mode::B0x1
    }
}
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type Oa1modeW<'a, REG> = crate::BitWriter<'a, REG, Oa1mode>;
impl<'a, REG> Oa1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 1 is a 7-bit address."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1mode::B0x0)
    }
    #[doc = "Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1mode::B0x1)
    }
}
#[doc = "Own Address 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oa1en {
    #[doc = "0: Own address 1 disabled. The received slave address OA1 is NACKed."]
    B0x0 = 0,
    #[doc = "1: Own address 1 enabled. The received slave address OA1 is ACKed."]
    B0x1 = 1,
}
impl From<Oa1en> for bool {
    #[inline(always)]
    fn from(variant: Oa1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub type Oa1enR = crate::BitReader<Oa1en>;
impl Oa1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa1en {
        match self.bits {
            false => Oa1en::B0x0,
            true => Oa1en::B0x1,
        }
    }
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oa1en::B0x0
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oa1en::B0x1
    }
}
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub type Oa1enW<'a, REG> = crate::BitWriter<'a, REG, Oa1en>;
impl<'a, REG> Oa1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1en::B0x0)
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1en::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&self) -> Oa1R {
        Oa1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&self) -> Oa1modeR {
        Oa1modeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> Oa1enR {
        Oa1enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa1(&mut self) -> Oa1W<I2cOar1Spec> {
        Oa1W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa1mode(&mut self) -> Oa1modeW<I2cOar1Spec> {
        Oa1modeW::new(self, 10)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn oa1en(&mut self) -> Oa1enW<I2cOar1Spec> {
        Oa1enW::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cOar1Spec;
impl crate::RegisterSpec for I2cOar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_oar1::R`](R) reader structure"]
impl crate::Readable for I2cOar1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_oar1::W`](W) writer structure"]
impl crate::Writable for I2cOar1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_OAR1 to value 0"]
impl crate::Resettable for I2cOar1Spec {
    const RESET_VALUE: u32 = 0;
}
