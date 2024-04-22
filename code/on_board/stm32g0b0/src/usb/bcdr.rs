#[doc = "Register `BCDR` reader"]
pub type R = crate::R<BcdrSpec>;
#[doc = "Register `BCDR` writer"]
pub type W = crate::W<BcdrSpec>;
#[doc = "Field `BCDEN` reader - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
pub type BcdenR = crate::BitReader;
#[doc = "Field `BCDEN` writer - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
pub type BcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDEN` reader - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type DcdenR = crate::BitReader;
#[doc = "Field `DCDEN` writer - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type DcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type SdenR = crate::BitReader;
#[doc = "Field `SDEN` writer - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type SdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data contact detection (DCD) status Device mode This bit gives the result of DCD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcdet {
    #[doc = "0: data lines contact not detected."]
    B0x0 = 0,
    #[doc = "1: data lines contact detected."]
    B0x1 = 1,
}
impl From<Dcdet> for bool {
    #[inline(always)]
    fn from(variant: Dcdet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDET` reader - Data contact detection (DCD) status Device mode This bit gives the result of DCD."]
pub type DcdetR = crate::BitReader<Dcdet>;
impl DcdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcdet {
        match self.bits {
            false => Dcdet::B0x0,
            true => Dcdet::B0x1,
        }
    }
    #[doc = "data lines contact not detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dcdet::B0x0
    }
    #[doc = "data lines contact detected."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dcdet::B0x1
    }
}
#[doc = "Primary detection (PD) status Device mode This bit gives the result of PD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdet {
    #[doc = "0: no BCD support detected (connected to SDP or proprietary device)."]
    B0x0 = 0,
    #[doc = "1: BCD support detected (connected to ACA, CDP or DCP)."]
    B0x1 = 1,
}
impl From<Pdet> for bool {
    #[inline(always)]
    fn from(variant: Pdet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDET` reader - Primary detection (PD) status Device mode This bit gives the result of PD."]
pub type PdetR = crate::BitReader<Pdet>;
impl PdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdet {
        match self.bits {
            false => Pdet::B0x0,
            true => Pdet::B0x1,
        }
    }
    #[doc = "no BCD support detected (connected to SDP or proprietary device)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pdet::B0x0
    }
    #[doc = "BCD support detected (connected to ACA, CDP or DCP)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pdet::B0x1
    }
}
#[doc = "Secondary detection (SD) status Device mode This bit gives the result of SD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdet {
    #[doc = "0: CDP detected."]
    B0x0 = 0,
    #[doc = "1: DCP detected."]
    B0x1 = 1,
}
impl From<Sdet> for bool {
    #[inline(always)]
    fn from(variant: Sdet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDET` reader - Secondary detection (SD) status Device mode This bit gives the result of SD."]
pub type SdetR = crate::BitReader<Sdet>;
impl SdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdet {
        match self.bits {
            false => Sdet::B0x0,
            true => Sdet::B0x1,
        }
    }
    #[doc = "CDP detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sdet::B0x0
    }
    #[doc = "DCP detected."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sdet::B0x1
    }
}
#[doc = "DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps2det {
    #[doc = "0: Normal port detected (connected to SDP, ACA, CDP or DCP)."]
    B0x0 = 0,
    #[doc = "1: PS2 port or proprietary charger detected."]
    B0x1 = 1,
}
impl From<Ps2det> for bool {
    #[inline(always)]
    fn from(variant: Ps2det) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS2DET` reader - DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification."]
pub type Ps2detR = crate::BitReader<Ps2det>;
impl Ps2detR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps2det {
        match self.bits {
            false => Ps2det::B0x0,
            true => Ps2det::B0x1,
        }
    }
    #[doc = "Normal port detected (connected to SDP, ACA, CDP or DCP)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ps2det::B0x0
    }
    #[doc = "PS2 port or proprietary charger detected."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ps2det::B0x1
    }
}
#[doc = "Field `DPPU_DPD` reader - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
pub type DppuDpdR = crate::BitReader;
#[doc = "Field `DPPU_DPD` writer - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
pub type DppuDpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
    #[inline(always)]
    pub fn bcden(&self) -> BcdenR {
        BcdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn dcden(&self) -> DcdenR {
        DcdenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn sden(&self) -> SdenR {
        SdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data contact detection (DCD) status Device mode This bit gives the result of DCD."]
    #[inline(always)]
    pub fn dcdet(&self) -> DcdetR {
        DcdetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Primary detection (PD) status Device mode This bit gives the result of PD."]
    #[inline(always)]
    pub fn pdet(&self) -> PdetR {
        PdetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secondary detection (SD) status Device mode This bit gives the result of SD."]
    #[inline(always)]
    pub fn sdet(&self) -> SdetR {
        SdetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification."]
    #[inline(always)]
    pub fn ps2det(&self) -> Ps2detR {
        Ps2detR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
    #[inline(always)]
    pub fn dppu_dpd(&self) -> DppuDpdR {
        DppuDpdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BcdenW<BcdrSpec> {
        BcdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DcdenW<BcdrSpec> {
        DcdenW::new(self, 1)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PdenW<BcdrSpec> {
        PdenW::new(self, 2)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SdenW<BcdrSpec> {
        SdenW::new(self, 3)
    }
    #[doc = "Bit 15 - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
    #[inline(always)]
    #[must_use]
    pub fn dppu_dpd(&mut self) -> DppuDpdW<BcdrSpec> {
        DppuDpdW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdrSpec;
impl crate::RegisterSpec for BcdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdr::R`](R) reader structure"]
impl crate::Readable for BcdrSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdr::W`](W) writer structure"]
impl crate::Writable for BcdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BcdrSpec {
    const RESET_VALUE: u32 = 0;
}
