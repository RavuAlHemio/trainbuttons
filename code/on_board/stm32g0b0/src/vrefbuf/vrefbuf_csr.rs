#[doc = "Register `VREFBUF_CSR` reader"]
pub type R = crate::R<VrefbufCsrSpec>;
#[doc = "Register `VREFBUF_CSR` writer"]
pub type W = crate::W<VrefbufCsrSpec>;
#[doc = "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Envr {
    #[doc = "0: Internal voltage reference mode disable (external voltage reference mode)."]
    B0x0 = 0,
    #[doc = "1: Internal voltage reference mode (reference buffer enable or hold mode) enable."]
    B0x1 = 1,
}
impl From<Envr> for bool {
    #[inline(always)]
    fn from(variant: Envr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENVR` reader - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
pub type EnvrR = crate::BitReader<Envr>;
impl EnvrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Envr {
        match self.bits {
            false => Envr::B0x0,
            true => Envr::B0x1,
        }
    }
    #[doc = "Internal voltage reference mode disable (external voltage reference mode)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Envr::B0x0
    }
    #[doc = "Internal voltage reference mode (reference buffer enable or hold mode) enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Envr::B0x1
    }
}
#[doc = "Field `ENVR` writer - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
pub type EnvrW<'a, REG> = crate::BitWriter<'a, REG, Envr>;
impl<'a, REG> EnvrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal voltage reference mode disable (external voltage reference mode)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Envr::B0x0)
    }
    #[doc = "Internal voltage reference mode (reference buffer enable or hold mode) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Envr::B0x1)
    }
}
#[doc = "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hiz {
    #[doc = "0: VREF+ pin is internally connected to the voltage reference buffer output."]
    B0x0 = 0,
    #[doc = "1: VREF+ pin is high impedance."]
    B0x1 = 1,
}
impl From<Hiz> for bool {
    #[inline(always)]
    fn from(variant: Hiz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIZ` reader - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
pub type HizR = crate::BitReader<Hiz>;
impl HizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hiz {
        match self.bits {
            false => Hiz::B0x0,
            true => Hiz::B0x1,
        }
    }
    #[doc = "VREF+ pin is internally connected to the voltage reference buffer output."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hiz::B0x0
    }
    #[doc = "VREF+ pin is high impedance."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hiz::B0x1
    }
}
#[doc = "Field `HIZ` writer - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
pub type HizW<'a, REG> = crate::BitWriter<'a, REG, Hiz>;
impl<'a, REG> HizW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREF+ pin is internally connected to the voltage reference buffer output."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hiz::B0x0)
    }
    #[doc = "VREF+ pin is high impedance."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hiz::B0x1)
    }
}
#[doc = "Voltage reference scale This bit selects the value generated by the voltage reference buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrs {
    #[doc = "0: Voltage reference set to VREF_OUT1 (around 2.048 V)."]
    B0x0 = 0,
    #[doc = "1: Voltage reference set to VREF_OUT2 (around 2.5 V)."]
    B0x1 = 1,
}
impl From<Vrs> for bool {
    #[inline(always)]
    fn from(variant: Vrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRS` reader - Voltage reference scale This bit selects the value generated by the voltage reference buffer."]
pub type VrsR = crate::BitReader<Vrs>;
impl VrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrs {
        match self.bits {
            false => Vrs::B0x0,
            true => Vrs::B0x1,
        }
    }
    #[doc = "Voltage reference set to VREF_OUT1 (around 2.048 V)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vrs::B0x0
    }
    #[doc = "Voltage reference set to VREF_OUT2 (around 2.5 V)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vrs::B0x1
    }
}
#[doc = "Field `VRS` writer - Voltage reference scale This bit selects the value generated by the voltage reference buffer."]
pub type VrsW<'a, REG> = crate::BitWriter<'a, REG, Vrs>;
impl<'a, REG> VrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage reference set to VREF_OUT1 (around 2.048 V)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrs::B0x0)
    }
    #[doc = "Voltage reference set to VREF_OUT2 (around 2.5 V)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrs::B0x1)
    }
}
#[doc = "Voltage reference buffer ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrr {
    #[doc = "0: the voltage reference buffer output is not ready."]
    B0x0 = 0,
    #[doc = "1: the voltage reference buffer output reached the requested level."]
    B0x1 = 1,
}
impl From<Vrr> for bool {
    #[inline(always)]
    fn from(variant: Vrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VrrR = crate::BitReader<Vrr>;
impl VrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrr {
        match self.bits {
            false => Vrr::B0x0,
            true => Vrr::B0x1,
        }
    }
    #[doc = "the voltage reference buffer output is not ready."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vrr::B0x0
    }
    #[doc = "the voltage reference buffer output reached the requested level."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vrr::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    pub fn envr(&self) -> EnvrR {
        EnvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    pub fn hiz(&self) -> HizR {
        HizR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage reference scale This bit selects the value generated by the voltage reference buffer."]
    #[inline(always)]
    pub fn vrs(&self) -> VrsR {
        VrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VrrR {
        VrrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> EnvrW<VrefbufCsrSpec> {
        EnvrW::new(self, 0)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HizW<VrefbufCsrSpec> {
        HizW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage reference scale This bit selects the value generated by the voltage reference buffer."]
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VrsW<VrefbufCsrSpec> {
        VrsW::new(self, 2)
    }
}
#[doc = "VREFBUF control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefbuf_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefbufCsrSpec;
impl crate::RegisterSpec for VrefbufCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefbuf_csr::R`](R) reader structure"]
impl crate::Readable for VrefbufCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vrefbuf_csr::W`](W) writer structure"]
impl crate::Writable for VrefbufCsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREFBUF_CSR to value 0x02"]
impl crate::Resettable for VrefbufCsrSpec {
    const RESET_VALUE: u32 = 0x02;
}
