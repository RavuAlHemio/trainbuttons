#[doc = "Register `WWDG_CR` reader"]
pub type R = crate::R<WwdgCrSpec>;
#[doc = "Register `WWDG_CR` writer"]
pub type W = crate::W<WwdgCrSpec>;
#[doc = "Field `T` reader - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type TR = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type TW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdga {
    #[doc = "0: Watchdog disabled"]
    B0x0 = 0,
    #[doc = "1: Watchdog enabled"]
    B0x1 = 1,
}
impl From<Wdga> for bool {
    #[inline(always)]
    fn from(variant: Wdga) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDGA` reader - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
pub type WdgaR = crate::BitReader<Wdga>;
impl WdgaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdga {
        match self.bits {
            false => Wdga::B0x0,
            true => Wdga::B0x1,
        }
    }
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wdga::B0x0
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wdga::B0x1
    }
}
#[doc = "Field `WDGA` writer - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
pub type WdgaW<'a, REG> = crate::BitWriter<'a, REG, Wdga>;
impl<'a, REG> WdgaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdga::B0x0)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdga::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn wdga(&self) -> WdgaR {
        WdgaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> TW<WwdgCrSpec> {
        TW::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WdgaW<WwdgCrSpec> {
        WdgaW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WwdgCrSpec;
impl crate::RegisterSpec for WwdgCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_cr::R`](R) reader structure"]
impl crate::Readable for WwdgCrSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdg_cr::W`](W) writer structure"]
impl crate::Writable for WwdgCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_CR to value 0x7f"]
impl crate::Resettable for WwdgCrSpec {
    const RESET_VALUE: u32 = 0x7f;
}
