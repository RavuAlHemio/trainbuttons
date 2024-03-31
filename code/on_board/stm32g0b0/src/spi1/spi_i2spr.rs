#[doc = "Register `SPI_I2SPR` reader"]
pub type R = crate::R<SpiI2sprSpec>;
#[doc = "Register `SPI_I2SPR` writer"]
pub type W = crate::W<SpiI2sprSpec>;
#[doc = "Field `I2SDIV` reader - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
pub type I2sdivR = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
pub type I2sdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Odd {
    #[doc = "0: Real divider value is = I2SDIV *2"]
    B0x0 = 0,
    #[doc = "1: Real divider value is = (I2SDIV * 2)+1"]
    B0x1 = 1,
}
impl From<Odd> for bool {
    #[inline(always)]
    fn from(variant: Odd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type OddR = crate::BitReader<Odd>;
impl OddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Odd {
        match self.bits {
            false => Odd::B0x0,
            true => Odd::B0x1,
        }
    }
    #[doc = "Real divider value is = I2SDIV *2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Odd::B0x0
    }
    #[doc = "Real divider value is = (I2SDIV * 2)+1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Odd::B0x1
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type OddW<'a, REG> = crate::BitWriter<'a, REG, Odd>;
impl<'a, REG> OddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real divider value is = I2SDIV *2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Odd::B0x0)
    }
    #[doc = "Real divider value is = (I2SDIV * 2)+1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Odd::B0x1)
    }
}
#[doc = "Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mckoe {
    #[doc = "0: Master clock output is disabled"]
    B0x0 = 0,
    #[doc = "1: Master clock output is enabled"]
    B0x1 = 1,
}
impl From<Mckoe> for bool {
    #[inline(always)]
    fn from(variant: Mckoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type MckoeR = crate::BitReader<Mckoe>;
impl MckoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mckoe {
        match self.bits {
            false => Mckoe::B0x0,
            true => Mckoe::B0x1,
        }
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mckoe::B0x0
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mckoe::B0x1
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type MckoeW<'a, REG> = crate::BitWriter<'a, REG, Mckoe>;
impl<'a, REG> MckoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mckoe::B0x0)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mckoe::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:7 - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2sdivR {
        I2sdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    pub fn mckoe(&self) -> MckoeR {
        MckoeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2sdivW<SpiI2sprSpec> {
        I2sdivW::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> OddW<SpiI2sprSpec> {
        OddW::new(self, 8)
    }
    #[doc = "Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MckoeW<SpiI2sprSpec> {
        MckoeW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2spr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2spr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiI2sprSpec;
impl crate::RegisterSpec for SpiI2sprSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_i2spr::R`](R) reader structure"]
impl crate::Readable for SpiI2sprSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_i2spr::W`](W) writer structure"]
impl crate::Writable for SpiI2sprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_I2SPR to value 0x02"]
impl crate::Resettable for SpiI2sprSpec {
    const RESET_VALUE: u16 = 0x02;
}
