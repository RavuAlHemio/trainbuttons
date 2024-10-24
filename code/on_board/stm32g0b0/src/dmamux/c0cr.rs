#[doc = "Register `C0CR` reader"]
pub type R = crate::R<C0crSpec>;
#[doc = "Register `C0CR` writer"]
pub type W = crate::W<C0crSpec>;
#[doc = "DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmareqId {
    #[doc = "0: DMA source NONE (0)."]
    None = 0,
    #[doc = "1: DMA source DMAMUX_REQ_GEN0 (1)."]
    DmamuxReqGen0 = 1,
    #[doc = "2: DMA source DMAMUX_REQ_GEN1 (2)."]
    DmamuxReqGen1 = 2,
    #[doc = "3: DMA source DMAMUX_REQ_GEN2 (3)."]
    DmamuxReqGen2 = 3,
    #[doc = "4: DMA source DMAMUX_REQ_GEN3 (4)."]
    DmamuxReqGen3 = 4,
    #[doc = "5: DMA source ADC (5)."]
    Adc = 5,
    #[doc = "10: DMA source I2C1_RX (10)."]
    I2c1Rx = 10,
    #[doc = "11: DMA source I2C1_TX (11)."]
    I2c1Tx = 11,
    #[doc = "12: DMA source I2C2_RX (12)."]
    I2c2Rx = 12,
    #[doc = "13: DMA source I2C2_TX (13)."]
    I2c2Tx = 13,
    #[doc = "16: DMA source SPI1_RX (16)."]
    Spi1Rx = 16,
    #[doc = "17: DMA source SPI1_TX (17)."]
    Spi1Tx = 17,
    #[doc = "18: DMA source SPI2_RX (18)."]
    Spi2Rx = 18,
    #[doc = "19: DMA source SPI2_TX (19)."]
    Spi2Tx = 19,
    #[doc = "20: DMA source TIM1_CH1 (20)."]
    Tim1Ch1 = 20,
    #[doc = "21: DMA source TIM1_CH2 (21)."]
    Tim1Ch2 = 21,
    #[doc = "22: DMA source TIM1_CH3 (22)."]
    Tim1Ch3 = 22,
    #[doc = "23: DMA source TIM1_CH4 (23)."]
    Tim1Ch4 = 23,
    #[doc = "24: DMA source TIM1_TRIG_COM (24)."]
    Tim1TrigCom = 24,
    #[doc = "25: DMA source TIM1_UP (25)."]
    Tim1Up = 25,
    #[doc = "32: DMA source TIM3_CH1 (32)."]
    Tim3Ch1 = 32,
    #[doc = "33: DMA source TIM3_CH2 (33)."]
    Tim3Ch2 = 33,
    #[doc = "34: DMA source TIM3_CH3 (34)."]
    Tim3Ch3 = 34,
    #[doc = "35: DMA source TIM3_CH4 (35)."]
    Tim3Ch4 = 35,
    #[doc = "36: DMA source TIM3_TRIG (36)."]
    Tim3Trig = 36,
    #[doc = "37: DMA source TIM3_UP (37)."]
    Tim3Up = 37,
    #[doc = "38: DMA source TIM6_UP (38)."]
    Tim6Up = 38,
    #[doc = "39: DMA source TIM7_UP (39)."]
    Tim7Up = 39,
    #[doc = "40: DMA source TIM15_CH1 (40)."]
    Tim15Ch1 = 40,
    #[doc = "41: DMA source TIM15_CH2 (41)."]
    Tim15Ch2 = 41,
    #[doc = "42: DMA source TIM15_TRIG_COM (42)."]
    Tim15TrigCom = 42,
    #[doc = "43: DMA source TIM15_UP (43)."]
    Tim15Up = 43,
    #[doc = "44: DMA source TIM16_CH1 (44)."]
    Tim16Ch1 = 44,
    #[doc = "45: DMA source TIM16_COM (45)."]
    Tim16Com = 45,
    #[doc = "46: DMA source TIM16_UP (46)."]
    Tim16Up = 46,
    #[doc = "47: DMA source TIM17_CH1 (47)."]
    Tim17Ch1 = 47,
    #[doc = "48: DMA source TIM17_COM (48)."]
    Tim17Com = 48,
    #[doc = "49: DMA source TIM17_UP (49)."]
    Tim17Up = 49,
    #[doc = "50: DMA source USART1_RX (50)."]
    Usart1Rx = 50,
    #[doc = "51: DMA source USART1_TX (51)."]
    Usart1Tx = 51,
    #[doc = "52: DMA source USART2_RX (52)."]
    Usart2Rx = 52,
    #[doc = "53: DMA source USART2_TX (53)."]
    Usart2Tx = 53,
    #[doc = "54: DMA source USART3_RX (54)."]
    Usart3Rx = 54,
    #[doc = "55: DMA source USART3_TX (55)."]
    Usart3Tx = 55,
    #[doc = "56: DMA source USART4_RX (56)."]
    Usart4Rx = 56,
    #[doc = "57: DMA source USART4_TX (57)."]
    Usart4Tx = 57,
    #[doc = "62: DMA source I2C3_RX (62)."]
    I2c3Rx = 62,
    #[doc = "63: DMA source I2C3_TX (63)."]
    I2c3Tx = 63,
    #[doc = "66: DMA source SPI3_RX (66)."]
    Spi3Rx = 66,
    #[doc = "67: DMA source SPI3_TX (67)."]
    Spi3Tx = 67,
    #[doc = "68: DMA source TIM4_CH1 (68)."]
    Tim4Ch1 = 68,
    #[doc = "69: DMA source TIM4_CH2 (69)."]
    Tim4Ch2 = 69,
    #[doc = "70: DMA source TIM4_CH3 (70)."]
    Tim4Ch3 = 70,
    #[doc = "71: DMA source TIM4_CH4 (71)."]
    Tim4Ch4 = 71,
    #[doc = "72: DMA source TIM4_TRIG (72)."]
    Tim4Trig = 72,
    #[doc = "73: DMA source TIM4_UP (73)."]
    Tim4Up = 73,
    #[doc = "74: DMA source USART5_RX (74)."]
    Usart5Rx = 74,
    #[doc = "75: DMA source USART5_TX (75)."]
    Usart5Tx = 75,
    #[doc = "76: DMA source USART6_RX (76)."]
    Usart6Rx = 76,
    #[doc = "77: DMA source USART6_TX (77)."]
    Usart6Tx = 77,
}
impl From<DmareqId> for u8 {
    #[inline(always)]
    fn from(variant: DmareqId) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmareqId {
    type Ux = u8;
}
impl crate::IsEnum for DmareqId {}
#[doc = "Field `DMAREQ_ID` reader - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DmareqIdR = crate::FieldReader<DmareqId>;
impl DmareqIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmareqId> {
        match self.bits {
            0 => Some(DmareqId::None),
            1 => Some(DmareqId::DmamuxReqGen0),
            2 => Some(DmareqId::DmamuxReqGen1),
            3 => Some(DmareqId::DmamuxReqGen2),
            4 => Some(DmareqId::DmamuxReqGen3),
            5 => Some(DmareqId::Adc),
            10 => Some(DmareqId::I2c1Rx),
            11 => Some(DmareqId::I2c1Tx),
            12 => Some(DmareqId::I2c2Rx),
            13 => Some(DmareqId::I2c2Tx),
            16 => Some(DmareqId::Spi1Rx),
            17 => Some(DmareqId::Spi1Tx),
            18 => Some(DmareqId::Spi2Rx),
            19 => Some(DmareqId::Spi2Tx),
            20 => Some(DmareqId::Tim1Ch1),
            21 => Some(DmareqId::Tim1Ch2),
            22 => Some(DmareqId::Tim1Ch3),
            23 => Some(DmareqId::Tim1Ch4),
            24 => Some(DmareqId::Tim1TrigCom),
            25 => Some(DmareqId::Tim1Up),
            32 => Some(DmareqId::Tim3Ch1),
            33 => Some(DmareqId::Tim3Ch2),
            34 => Some(DmareqId::Tim3Ch3),
            35 => Some(DmareqId::Tim3Ch4),
            36 => Some(DmareqId::Tim3Trig),
            37 => Some(DmareqId::Tim3Up),
            38 => Some(DmareqId::Tim6Up),
            39 => Some(DmareqId::Tim7Up),
            40 => Some(DmareqId::Tim15Ch1),
            41 => Some(DmareqId::Tim15Ch2),
            42 => Some(DmareqId::Tim15TrigCom),
            43 => Some(DmareqId::Tim15Up),
            44 => Some(DmareqId::Tim16Ch1),
            45 => Some(DmareqId::Tim16Com),
            46 => Some(DmareqId::Tim16Up),
            47 => Some(DmareqId::Tim17Ch1),
            48 => Some(DmareqId::Tim17Com),
            49 => Some(DmareqId::Tim17Up),
            50 => Some(DmareqId::Usart1Rx),
            51 => Some(DmareqId::Usart1Tx),
            52 => Some(DmareqId::Usart2Rx),
            53 => Some(DmareqId::Usart2Tx),
            54 => Some(DmareqId::Usart3Rx),
            55 => Some(DmareqId::Usart3Tx),
            56 => Some(DmareqId::Usart4Rx),
            57 => Some(DmareqId::Usart4Tx),
            62 => Some(DmareqId::I2c3Rx),
            63 => Some(DmareqId::I2c3Tx),
            66 => Some(DmareqId::Spi3Rx),
            67 => Some(DmareqId::Spi3Tx),
            68 => Some(DmareqId::Tim4Ch1),
            69 => Some(DmareqId::Tim4Ch2),
            70 => Some(DmareqId::Tim4Ch3),
            71 => Some(DmareqId::Tim4Ch4),
            72 => Some(DmareqId::Tim4Trig),
            73 => Some(DmareqId::Tim4Up),
            74 => Some(DmareqId::Usart5Rx),
            75 => Some(DmareqId::Usart5Tx),
            76 => Some(DmareqId::Usart6Rx),
            77 => Some(DmareqId::Usart6Tx),
            _ => None,
        }
    }
    #[doc = "DMA source NONE (0)."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DmareqId::None
    }
    #[doc = "DMA source DMAMUX_REQ_GEN0 (1)."]
    #[inline(always)]
    pub fn is_dmamux_req_gen0(&self) -> bool {
        *self == DmareqId::DmamuxReqGen0
    }
    #[doc = "DMA source DMAMUX_REQ_GEN1 (2)."]
    #[inline(always)]
    pub fn is_dmamux_req_gen1(&self) -> bool {
        *self == DmareqId::DmamuxReqGen1
    }
    #[doc = "DMA source DMAMUX_REQ_GEN2 (3)."]
    #[inline(always)]
    pub fn is_dmamux_req_gen2(&self) -> bool {
        *self == DmareqId::DmamuxReqGen2
    }
    #[doc = "DMA source DMAMUX_REQ_GEN3 (4)."]
    #[inline(always)]
    pub fn is_dmamux_req_gen3(&self) -> bool {
        *self == DmareqId::DmamuxReqGen3
    }
    #[doc = "DMA source ADC (5)."]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DmareqId::Adc
    }
    #[doc = "DMA source I2C1_RX (10)."]
    #[inline(always)]
    pub fn is_i2c1_rx(&self) -> bool {
        *self == DmareqId::I2c1Rx
    }
    #[doc = "DMA source I2C1_TX (11)."]
    #[inline(always)]
    pub fn is_i2c1_tx(&self) -> bool {
        *self == DmareqId::I2c1Tx
    }
    #[doc = "DMA source I2C2_RX (12)."]
    #[inline(always)]
    pub fn is_i2c2_rx(&self) -> bool {
        *self == DmareqId::I2c2Rx
    }
    #[doc = "DMA source I2C2_TX (13)."]
    #[inline(always)]
    pub fn is_i2c2_tx(&self) -> bool {
        *self == DmareqId::I2c2Tx
    }
    #[doc = "DMA source SPI1_RX (16)."]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == DmareqId::Spi1Rx
    }
    #[doc = "DMA source SPI1_TX (17)."]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == DmareqId::Spi1Tx
    }
    #[doc = "DMA source SPI2_RX (18)."]
    #[inline(always)]
    pub fn is_spi2_rx(&self) -> bool {
        *self == DmareqId::Spi2Rx
    }
    #[doc = "DMA source SPI2_TX (19)."]
    #[inline(always)]
    pub fn is_spi2_tx(&self) -> bool {
        *self == DmareqId::Spi2Tx
    }
    #[doc = "DMA source TIM1_CH1 (20)."]
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DmareqId::Tim1Ch1
    }
    #[doc = "DMA source TIM1_CH2 (21)."]
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DmareqId::Tim1Ch2
    }
    #[doc = "DMA source TIM1_CH3 (22)."]
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DmareqId::Tim1Ch3
    }
    #[doc = "DMA source TIM1_CH4 (23)."]
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DmareqId::Tim1Ch4
    }
    #[doc = "DMA source TIM1_TRIG_COM (24)."]
    #[inline(always)]
    pub fn is_tim1_trig_com(&self) -> bool {
        *self == DmareqId::Tim1TrigCom
    }
    #[doc = "DMA source TIM1_UP (25)."]
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DmareqId::Tim1Up
    }
    #[doc = "DMA source TIM3_CH1 (32)."]
    #[inline(always)]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == DmareqId::Tim3Ch1
    }
    #[doc = "DMA source TIM3_CH2 (33)."]
    #[inline(always)]
    pub fn is_tim3_ch2(&self) -> bool {
        *self == DmareqId::Tim3Ch2
    }
    #[doc = "DMA source TIM3_CH3 (34)."]
    #[inline(always)]
    pub fn is_tim3_ch3(&self) -> bool {
        *self == DmareqId::Tim3Ch3
    }
    #[doc = "DMA source TIM3_CH4 (35)."]
    #[inline(always)]
    pub fn is_tim3_ch4(&self) -> bool {
        *self == DmareqId::Tim3Ch4
    }
    #[doc = "DMA source TIM3_TRIG (36)."]
    #[inline(always)]
    pub fn is_tim3_trig(&self) -> bool {
        *self == DmareqId::Tim3Trig
    }
    #[doc = "DMA source TIM3_UP (37)."]
    #[inline(always)]
    pub fn is_tim3_up(&self) -> bool {
        *self == DmareqId::Tim3Up
    }
    #[doc = "DMA source TIM6_UP (38)."]
    #[inline(always)]
    pub fn is_tim6_up(&self) -> bool {
        *self == DmareqId::Tim6Up
    }
    #[doc = "DMA source TIM7_UP (39)."]
    #[inline(always)]
    pub fn is_tim7_up(&self) -> bool {
        *self == DmareqId::Tim7Up
    }
    #[doc = "DMA source TIM15_CH1 (40)."]
    #[inline(always)]
    pub fn is_tim15_ch1(&self) -> bool {
        *self == DmareqId::Tim15Ch1
    }
    #[doc = "DMA source TIM15_CH2 (41)."]
    #[inline(always)]
    pub fn is_tim15_ch2(&self) -> bool {
        *self == DmareqId::Tim15Ch2
    }
    #[doc = "DMA source TIM15_TRIG_COM (42)."]
    #[inline(always)]
    pub fn is_tim15_trig_com(&self) -> bool {
        *self == DmareqId::Tim15TrigCom
    }
    #[doc = "DMA source TIM15_UP (43)."]
    #[inline(always)]
    pub fn is_tim15_up(&self) -> bool {
        *self == DmareqId::Tim15Up
    }
    #[doc = "DMA source TIM16_CH1 (44)."]
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DmareqId::Tim16Ch1
    }
    #[doc = "DMA source TIM16_COM (45)."]
    #[inline(always)]
    pub fn is_tim16_com(&self) -> bool {
        *self == DmareqId::Tim16Com
    }
    #[doc = "DMA source TIM16_UP (46)."]
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DmareqId::Tim16Up
    }
    #[doc = "DMA source TIM17_CH1 (47)."]
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DmareqId::Tim17Ch1
    }
    #[doc = "DMA source TIM17_COM (48)."]
    #[inline(always)]
    pub fn is_tim17_com(&self) -> bool {
        *self == DmareqId::Tim17Com
    }
    #[doc = "DMA source TIM17_UP (49)."]
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DmareqId::Tim17Up
    }
    #[doc = "DMA source USART1_RX (50)."]
    #[inline(always)]
    pub fn is_usart1_rx(&self) -> bool {
        *self == DmareqId::Usart1Rx
    }
    #[doc = "DMA source USART1_TX (51)."]
    #[inline(always)]
    pub fn is_usart1_tx(&self) -> bool {
        *self == DmareqId::Usart1Tx
    }
    #[doc = "DMA source USART2_RX (52)."]
    #[inline(always)]
    pub fn is_usart2_rx(&self) -> bool {
        *self == DmareqId::Usart2Rx
    }
    #[doc = "DMA source USART2_TX (53)."]
    #[inline(always)]
    pub fn is_usart2_tx(&self) -> bool {
        *self == DmareqId::Usart2Tx
    }
    #[doc = "DMA source USART3_RX (54)."]
    #[inline(always)]
    pub fn is_usart3_rx(&self) -> bool {
        *self == DmareqId::Usart3Rx
    }
    #[doc = "DMA source USART3_TX (55)."]
    #[inline(always)]
    pub fn is_usart3_tx(&self) -> bool {
        *self == DmareqId::Usart3Tx
    }
    #[doc = "DMA source USART4_RX (56)."]
    #[inline(always)]
    pub fn is_usart4_rx(&self) -> bool {
        *self == DmareqId::Usart4Rx
    }
    #[doc = "DMA source USART4_TX (57)."]
    #[inline(always)]
    pub fn is_usart4_tx(&self) -> bool {
        *self == DmareqId::Usart4Tx
    }
    #[doc = "DMA source I2C3_RX (62)."]
    #[inline(always)]
    pub fn is_i2c3_rx(&self) -> bool {
        *self == DmareqId::I2c3Rx
    }
    #[doc = "DMA source I2C3_TX (63)."]
    #[inline(always)]
    pub fn is_i2c3_tx(&self) -> bool {
        *self == DmareqId::I2c3Tx
    }
    #[doc = "DMA source SPI3_RX (66)."]
    #[inline(always)]
    pub fn is_spi3_rx(&self) -> bool {
        *self == DmareqId::Spi3Rx
    }
    #[doc = "DMA source SPI3_TX (67)."]
    #[inline(always)]
    pub fn is_spi3_tx(&self) -> bool {
        *self == DmareqId::Spi3Tx
    }
    #[doc = "DMA source TIM4_CH1 (68)."]
    #[inline(always)]
    pub fn is_tim4_ch1(&self) -> bool {
        *self == DmareqId::Tim4Ch1
    }
    #[doc = "DMA source TIM4_CH2 (69)."]
    #[inline(always)]
    pub fn is_tim4_ch2(&self) -> bool {
        *self == DmareqId::Tim4Ch2
    }
    #[doc = "DMA source TIM4_CH3 (70)."]
    #[inline(always)]
    pub fn is_tim4_ch3(&self) -> bool {
        *self == DmareqId::Tim4Ch3
    }
    #[doc = "DMA source TIM4_CH4 (71)."]
    #[inline(always)]
    pub fn is_tim4_ch4(&self) -> bool {
        *self == DmareqId::Tim4Ch4
    }
    #[doc = "DMA source TIM4_TRIG (72)."]
    #[inline(always)]
    pub fn is_tim4_trig(&self) -> bool {
        *self == DmareqId::Tim4Trig
    }
    #[doc = "DMA source TIM4_UP (73)."]
    #[inline(always)]
    pub fn is_tim4_up(&self) -> bool {
        *self == DmareqId::Tim4Up
    }
    #[doc = "DMA source USART5_RX (74)."]
    #[inline(always)]
    pub fn is_usart5_rx(&self) -> bool {
        *self == DmareqId::Usart5Rx
    }
    #[doc = "DMA source USART5_TX (75)."]
    #[inline(always)]
    pub fn is_usart5_tx(&self) -> bool {
        *self == DmareqId::Usart5Tx
    }
    #[doc = "DMA source USART6_RX (76)."]
    #[inline(always)]
    pub fn is_usart6_rx(&self) -> bool {
        *self == DmareqId::Usart6Rx
    }
    #[doc = "DMA source USART6_TX (77)."]
    #[inline(always)]
    pub fn is_usart6_tx(&self) -> bool {
        *self == DmareqId::Usart6Tx
    }
}
#[doc = "Field `DMAREQ_ID` writer - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DmareqIdW<'a, REG> = crate::FieldWriter<'a, REG, 7, DmareqId>;
impl<'a, REG> DmareqIdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA source NONE (0)."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::None)
    }
    #[doc = "DMA source DMAMUX_REQ_GEN0 (1)."]
    #[inline(always)]
    pub fn dmamux_req_gen0(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::DmamuxReqGen0)
    }
    #[doc = "DMA source DMAMUX_REQ_GEN1 (2)."]
    #[inline(always)]
    pub fn dmamux_req_gen1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::DmamuxReqGen1)
    }
    #[doc = "DMA source DMAMUX_REQ_GEN2 (3)."]
    #[inline(always)]
    pub fn dmamux_req_gen2(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::DmamuxReqGen2)
    }
    #[doc = "DMA source DMAMUX_REQ_GEN3 (4)."]
    #[inline(always)]
    pub fn dmamux_req_gen3(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::DmamuxReqGen3)
    }
    #[doc = "DMA source ADC (5)."]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Adc)
    }
    #[doc = "DMA source I2C1_RX (10)."]
    #[inline(always)]
    pub fn i2c1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::I2c1Rx)
    }
    #[doc = "DMA source I2C1_TX (11)."]
    #[inline(always)]
    pub fn i2c1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::I2c1Tx)
    }
    #[doc = "DMA source I2C2_RX (12)."]
    #[inline(always)]
    pub fn i2c2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::I2c2Rx)
    }
    #[doc = "DMA source I2C2_TX (13)."]
    #[inline(always)]
    pub fn i2c2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::I2c2Tx)
    }
    #[doc = "DMA source SPI1_RX (16)."]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Spi1Rx)
    }
    #[doc = "DMA source SPI1_TX (17)."]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Spi1Tx)
    }
    #[doc = "DMA source SPI2_RX (18)."]
    #[inline(always)]
    pub fn spi2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Spi2Rx)
    }
    #[doc = "DMA source SPI2_TX (19)."]
    #[inline(always)]
    pub fn spi2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Spi2Tx)
    }
    #[doc = "DMA source TIM1_CH1 (20)."]
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim1Ch1)
    }
    #[doc = "DMA source TIM1_CH2 (21)."]
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim1Ch2)
    }
    #[doc = "DMA source TIM1_CH3 (22)."]
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim1Ch3)
    }
    #[doc = "DMA source TIM1_CH4 (23)."]
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim1Ch4)
    }
    #[doc = "DMA source TIM1_TRIG_COM (24)."]
    #[inline(always)]
    pub fn tim1_trig_com(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim1TrigCom)
    }
    #[doc = "DMA source TIM1_UP (25)."]
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim1Up)
    }
    #[doc = "DMA source TIM3_CH1 (32)."]
    #[inline(always)]
    pub fn tim3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim3Ch1)
    }
    #[doc = "DMA source TIM3_CH2 (33)."]
    #[inline(always)]
    pub fn tim3_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim3Ch2)
    }
    #[doc = "DMA source TIM3_CH3 (34)."]
    #[inline(always)]
    pub fn tim3_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim3Ch3)
    }
    #[doc = "DMA source TIM3_CH4 (35)."]
    #[inline(always)]
    pub fn tim3_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim3Ch4)
    }
    #[doc = "DMA source TIM3_TRIG (36)."]
    #[inline(always)]
    pub fn tim3_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim3Trig)
    }
    #[doc = "DMA source TIM3_UP (37)."]
    #[inline(always)]
    pub fn tim3_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim3Up)
    }
    #[doc = "DMA source TIM6_UP (38)."]
    #[inline(always)]
    pub fn tim6_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim6Up)
    }
    #[doc = "DMA source TIM7_UP (39)."]
    #[inline(always)]
    pub fn tim7_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim7Up)
    }
    #[doc = "DMA source TIM15_CH1 (40)."]
    #[inline(always)]
    pub fn tim15_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim15Ch1)
    }
    #[doc = "DMA source TIM15_CH2 (41)."]
    #[inline(always)]
    pub fn tim15_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim15Ch2)
    }
    #[doc = "DMA source TIM15_TRIG_COM (42)."]
    #[inline(always)]
    pub fn tim15_trig_com(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim15TrigCom)
    }
    #[doc = "DMA source TIM15_UP (43)."]
    #[inline(always)]
    pub fn tim15_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim15Up)
    }
    #[doc = "DMA source TIM16_CH1 (44)."]
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim16Ch1)
    }
    #[doc = "DMA source TIM16_COM (45)."]
    #[inline(always)]
    pub fn tim16_com(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim16Com)
    }
    #[doc = "DMA source TIM16_UP (46)."]
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim16Up)
    }
    #[doc = "DMA source TIM17_CH1 (47)."]
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim17Ch1)
    }
    #[doc = "DMA source TIM17_COM (48)."]
    #[inline(always)]
    pub fn tim17_com(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim17Com)
    }
    #[doc = "DMA source TIM17_UP (49)."]
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim17Up)
    }
    #[doc = "DMA source USART1_RX (50)."]
    #[inline(always)]
    pub fn usart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart1Rx)
    }
    #[doc = "DMA source USART1_TX (51)."]
    #[inline(always)]
    pub fn usart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart1Tx)
    }
    #[doc = "DMA source USART2_RX (52)."]
    #[inline(always)]
    pub fn usart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart2Rx)
    }
    #[doc = "DMA source USART2_TX (53)."]
    #[inline(always)]
    pub fn usart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart2Tx)
    }
    #[doc = "DMA source USART3_RX (54)."]
    #[inline(always)]
    pub fn usart3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart3Rx)
    }
    #[doc = "DMA source USART3_TX (55)."]
    #[inline(always)]
    pub fn usart3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart3Tx)
    }
    #[doc = "DMA source USART4_RX (56)."]
    #[inline(always)]
    pub fn usart4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart4Rx)
    }
    #[doc = "DMA source USART4_TX (57)."]
    #[inline(always)]
    pub fn usart4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart4Tx)
    }
    #[doc = "DMA source I2C3_RX (62)."]
    #[inline(always)]
    pub fn i2c3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::I2c3Rx)
    }
    #[doc = "DMA source I2C3_TX (63)."]
    #[inline(always)]
    pub fn i2c3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::I2c3Tx)
    }
    #[doc = "DMA source SPI3_RX (66)."]
    #[inline(always)]
    pub fn spi3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Spi3Rx)
    }
    #[doc = "DMA source SPI3_TX (67)."]
    #[inline(always)]
    pub fn spi3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Spi3Tx)
    }
    #[doc = "DMA source TIM4_CH1 (68)."]
    #[inline(always)]
    pub fn tim4_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim4Ch1)
    }
    #[doc = "DMA source TIM4_CH2 (69)."]
    #[inline(always)]
    pub fn tim4_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim4Ch2)
    }
    #[doc = "DMA source TIM4_CH3 (70)."]
    #[inline(always)]
    pub fn tim4_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim4Ch3)
    }
    #[doc = "DMA source TIM4_CH4 (71)."]
    #[inline(always)]
    pub fn tim4_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim4Ch4)
    }
    #[doc = "DMA source TIM4_TRIG (72)."]
    #[inline(always)]
    pub fn tim4_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim4Trig)
    }
    #[doc = "DMA source TIM4_UP (73)."]
    #[inline(always)]
    pub fn tim4_up(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Tim4Up)
    }
    #[doc = "DMA source USART5_RX (74)."]
    #[inline(always)]
    pub fn usart5_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart5Rx)
    }
    #[doc = "DMA source USART5_TX (75)."]
    #[inline(always)]
    pub fn usart5_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart5Tx)
    }
    #[doc = "DMA source USART6_RX (76)."]
    #[inline(always)]
    pub fn usart6_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart6Rx)
    }
    #[doc = "DMA source USART6_TX (77)."]
    #[inline(always)]
    pub fn usart6_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DmareqId::Usart6Tx)
    }
}
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soie {
    #[doc = "0: interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B0x1 = 1,
}
impl From<Soie> for bool {
    #[inline(always)]
    fn from(variant: Soie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOIE` reader - Synchronization overrun interrupt enable"]
pub type SoieR = crate::BitReader<Soie>;
impl SoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soie {
        match self.bits {
            false => Soie::B0x0,
            true => Soie::B0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Soie::B0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Soie::B0x1
    }
}
#[doc = "Field `SOIE` writer - Synchronization overrun interrupt enable"]
pub type SoieW<'a, REG> = crate::BitWriter<'a, REG, Soie>;
impl<'a, REG> SoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Soie::B0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Soie::B0x1)
    }
}
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ege {
    #[doc = "0: event generation disabled"]
    B0x0 = 0,
    #[doc = "1: event generation enabled"]
    B0x1 = 1,
}
impl From<Ege> for bool {
    #[inline(always)]
    fn from(variant: Ege) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGE` reader - Event generation enable"]
pub type EgeR = crate::BitReader<Ege>;
impl EgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ege {
        match self.bits {
            false => Ege::B0x0,
            true => Ege::B0x1,
        }
    }
    #[doc = "event generation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ege::B0x0
    }
    #[doc = "event generation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ege::B0x1
    }
}
#[doc = "Field `EGE` writer - Event generation enable"]
pub type EgeW<'a, REG> = crate::BitWriter<'a, REG, Ege>;
impl<'a, REG> EgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "event generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ege::B0x0)
    }
    #[doc = "event generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ege::B0x1)
    }
}
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Se {
    #[doc = "0: synchronization disabled"]
    B0x0 = 0,
    #[doc = "1: synchronization enabled"]
    B0x1 = 1,
}
impl From<Se> for bool {
    #[inline(always)]
    fn from(variant: Se) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Synchronization enable"]
pub type SeR = crate::BitReader<Se>;
impl SeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Se {
        match self.bits {
            false => Se::B0x0,
            true => Se::B0x1,
        }
    }
    #[doc = "synchronization disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Se::B0x0
    }
    #[doc = "synchronization enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Se::B0x1
    }
}
#[doc = "Field `SE` writer - Synchronization enable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG, Se>;
impl<'a, REG> SeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "synchronization disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Se::B0x0)
    }
    #[doc = "synchronization enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Se::B0x1)
    }
}
#[doc = "Synchronization polarity Defines the edge polarity of the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spol {
    #[doc = "0: no event, i.e. no synchronization nor detection."]
    B0x0 = 0,
    #[doc = "1: rising edge"]
    B0x1 = 1,
    #[doc = "2: falling edge"]
    B0x2 = 2,
    #[doc = "3: rising and falling edge"]
    B0x3 = 3,
}
impl From<Spol> for u8 {
    #[inline(always)]
    fn from(variant: Spol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spol {
    type Ux = u8;
}
impl crate::IsEnum for Spol {}
#[doc = "Field `SPOL` reader - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
pub type SpolR = crate::FieldReader<Spol>;
impl SpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol {
        match self.bits {
            0 => Spol::B0x0,
            1 => Spol::B0x1,
            2 => Spol::B0x2,
            3 => Spol::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no event, i.e. no synchronization nor detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spol::B0x0
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spol::B0x1
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Spol::B0x2
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Spol::B0x3
    }
}
#[doc = "Field `SPOL` writer - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
pub type SpolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spol, crate::Safe>;
impl<'a, REG> SpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no event, i.e. no synchronization nor detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x0)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x1)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x2)
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x3)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
pub type NbreqR = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
pub type NbreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
pub type SyncIdR = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
pub type SyncIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DmareqIdR {
        DmareqIdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SoieR {
        SoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&self) -> EgeR {
        EgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SpolR {
        SpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
    #[inline(always)]
    pub fn nbreq(&self) -> NbreqR {
        NbreqR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
    #[inline(always)]
    pub fn sync_id(&self) -> SyncIdR {
        SyncIdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DmareqIdW<C0crSpec> {
        DmareqIdW::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SoieW<C0crSpec> {
        SoieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EgeW<C0crSpec> {
        EgeW::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SeW<C0crSpec> {
        SeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SpolW<C0crSpec> {
        SpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NbreqW<C0crSpec> {
        NbreqW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SyncIdW<C0crSpec> {
        SyncIdW::new(self, 24)
    }
}
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0crSpec;
impl crate::RegisterSpec for C0crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0cr::R`](R) reader structure"]
impl crate::Readable for C0crSpec {}
#[doc = "`write(|w| ..)` method takes [`c0cr::W`](W) writer structure"]
impl crate::Writable for C0crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C0CR to value 0"]
impl crate::Resettable for C0crSpec {
    const RESET_VALUE: u32 = 0;
}
