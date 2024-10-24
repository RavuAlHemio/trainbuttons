#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "global interrupt flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif1 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif1> for bool {
    #[inline(always)]
    fn from(variant: Gif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF1` reader - global interrupt flag for channel 1"]
pub type Gif1R = crate::BitReader<Gif1>;
impl Gif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif1 {
        match self.bits {
            false => Gif1::B0x0,
            true => Gif1::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif1::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif1::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif1 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif1> for bool {
    #[inline(always)]
    fn from(variant: Tcif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF1` reader - transfer complete (TC) flag for channel 1"]
pub type Tcif1R = crate::BitReader<Tcif1>;
impl Tcif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif1 {
        match self.bits {
            false => Tcif1::B0x0,
            true => Tcif1::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif1::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif1::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif1 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif1> for bool {
    #[inline(always)]
    fn from(variant: Htif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF1` reader - half transfer (HT) flag for channel 1"]
pub type Htif1R = crate::BitReader<Htif1>;
impl Htif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif1 {
        match self.bits {
            false => Htif1::B0x0,
            true => Htif1::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif1::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif1::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif1 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif1> for bool {
    #[inline(always)]
    fn from(variant: Teif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF1` reader - transfer error (TE) flag for channel 1"]
pub type Teif1R = crate::BitReader<Teif1>;
impl Teif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif1 {
        match self.bits {
            false => Teif1::B0x0,
            true => Teif1::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif1::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif1::B0x1
    }
}
#[doc = "global interrupt flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif2 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif2> for bool {
    #[inline(always)]
    fn from(variant: Gif2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF2` reader - global interrupt flag for channel 2"]
pub type Gif2R = crate::BitReader<Gif2>;
impl Gif2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif2 {
        match self.bits {
            false => Gif2::B0x0,
            true => Gif2::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif2::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif2::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif2 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif2> for bool {
    #[inline(always)]
    fn from(variant: Tcif2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF2` reader - transfer complete (TC) flag for channel 2"]
pub type Tcif2R = crate::BitReader<Tcif2>;
impl Tcif2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif2 {
        match self.bits {
            false => Tcif2::B0x0,
            true => Tcif2::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif2::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif2::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif2 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif2> for bool {
    #[inline(always)]
    fn from(variant: Htif2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF2` reader - half transfer (HT) flag for channel 2"]
pub type Htif2R = crate::BitReader<Htif2>;
impl Htif2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif2 {
        match self.bits {
            false => Htif2::B0x0,
            true => Htif2::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif2::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif2::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif2 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif2> for bool {
    #[inline(always)]
    fn from(variant: Teif2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF2` reader - transfer error (TE) flag for channel 2"]
pub type Teif2R = crate::BitReader<Teif2>;
impl Teif2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif2 {
        match self.bits {
            false => Teif2::B0x0,
            true => Teif2::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif2::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif2::B0x1
    }
}
#[doc = "global interrupt flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif3 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif3> for bool {
    #[inline(always)]
    fn from(variant: Gif3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF3` reader - global interrupt flag for channel 3"]
pub type Gif3R = crate::BitReader<Gif3>;
impl Gif3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif3 {
        match self.bits {
            false => Gif3::B0x0,
            true => Gif3::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif3::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif3::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif3 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif3> for bool {
    #[inline(always)]
    fn from(variant: Tcif3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF3` reader - transfer complete (TC) flag for channel 3"]
pub type Tcif3R = crate::BitReader<Tcif3>;
impl Tcif3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif3 {
        match self.bits {
            false => Tcif3::B0x0,
            true => Tcif3::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif3::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif3::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif3 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif3> for bool {
    #[inline(always)]
    fn from(variant: Htif3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF3` reader - half transfer (HT) flag for channel 3"]
pub type Htif3R = crate::BitReader<Htif3>;
impl Htif3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif3 {
        match self.bits {
            false => Htif3::B0x0,
            true => Htif3::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif3::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif3::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif3 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif3> for bool {
    #[inline(always)]
    fn from(variant: Teif3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF3` reader - transfer error (TE) flag for channel 3"]
pub type Teif3R = crate::BitReader<Teif3>;
impl Teif3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif3 {
        match self.bits {
            false => Teif3::B0x0,
            true => Teif3::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif3::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif3::B0x1
    }
}
#[doc = "global interrupt flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif4 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif4> for bool {
    #[inline(always)]
    fn from(variant: Gif4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF4` reader - global interrupt flag for channel 4"]
pub type Gif4R = crate::BitReader<Gif4>;
impl Gif4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif4 {
        match self.bits {
            false => Gif4::B0x0,
            true => Gif4::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif4::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif4::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif4 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif4> for bool {
    #[inline(always)]
    fn from(variant: Tcif4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF4` reader - transfer complete (TC) flag for channel 4"]
pub type Tcif4R = crate::BitReader<Tcif4>;
impl Tcif4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif4 {
        match self.bits {
            false => Tcif4::B0x0,
            true => Tcif4::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif4::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif4::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif4 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif4> for bool {
    #[inline(always)]
    fn from(variant: Htif4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF4` reader - half transfer (HT) flag for channel 4"]
pub type Htif4R = crate::BitReader<Htif4>;
impl Htif4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif4 {
        match self.bits {
            false => Htif4::B0x0,
            true => Htif4::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif4::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif4::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif4 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif4> for bool {
    #[inline(always)]
    fn from(variant: Teif4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF4` reader - transfer error (TE) flag for channel 4"]
pub type Teif4R = crate::BitReader<Teif4>;
impl Teif4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif4 {
        match self.bits {
            false => Teif4::B0x0,
            true => Teif4::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif4::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif4::B0x1
    }
}
#[doc = "global interrupt flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif5 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif5> for bool {
    #[inline(always)]
    fn from(variant: Gif5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF5` reader - global interrupt flag for channel 5"]
pub type Gif5R = crate::BitReader<Gif5>;
impl Gif5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif5 {
        match self.bits {
            false => Gif5::B0x0,
            true => Gif5::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif5::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif5::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif5 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif5> for bool {
    #[inline(always)]
    fn from(variant: Tcif5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF5` reader - transfer complete (TC) flag for channel 5"]
pub type Tcif5R = crate::BitReader<Tcif5>;
impl Tcif5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif5 {
        match self.bits {
            false => Tcif5::B0x0,
            true => Tcif5::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif5::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif5::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif5 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif5> for bool {
    #[inline(always)]
    fn from(variant: Htif5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF5` reader - half transfer (HT) flag for channel 5"]
pub type Htif5R = crate::BitReader<Htif5>;
impl Htif5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif5 {
        match self.bits {
            false => Htif5::B0x0,
            true => Htif5::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif5::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif5::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif5 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif5> for bool {
    #[inline(always)]
    fn from(variant: Teif5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF5` reader - transfer error (TE) flag for channel 5"]
pub type Teif5R = crate::BitReader<Teif5>;
impl Teif5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif5 {
        match self.bits {
            false => Teif5::B0x0,
            true => Teif5::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif5::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif5::B0x1
    }
}
#[doc = "global interrupt flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif6 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif6> for bool {
    #[inline(always)]
    fn from(variant: Gif6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF6` reader - global interrupt flag for channel 6"]
pub type Gif6R = crate::BitReader<Gif6>;
impl Gif6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif6 {
        match self.bits {
            false => Gif6::B0x0,
            true => Gif6::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif6::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif6::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif6 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif6> for bool {
    #[inline(always)]
    fn from(variant: Tcif6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF6` reader - transfer complete (TC) flag for channel 6"]
pub type Tcif6R = crate::BitReader<Tcif6>;
impl Tcif6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif6 {
        match self.bits {
            false => Tcif6::B0x0,
            true => Tcif6::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif6::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif6::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif6 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif6> for bool {
    #[inline(always)]
    fn from(variant: Htif6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF6` reader - half transfer (HT) flag for channel 6"]
pub type Htif6R = crate::BitReader<Htif6>;
impl Htif6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif6 {
        match self.bits {
            false => Htif6::B0x0,
            true => Htif6::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif6::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif6::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif6 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif6> for bool {
    #[inline(always)]
    fn from(variant: Teif6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF6` reader - transfer error (TE) flag for channel 6"]
pub type Teif6R = crate::BitReader<Teif6>;
impl Teif6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif6 {
        match self.bits {
            false => Teif6::B0x0,
            true => Teif6::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif6::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif6::B0x1
    }
}
#[doc = "global interrupt flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gif7 {
    #[doc = "0: no TE, HT or TC event"]
    B0x0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B0x1 = 1,
}
impl From<Gif7> for bool {
    #[inline(always)]
    fn from(variant: Gif7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF7` reader - global interrupt flag for channel 7"]
pub type Gif7R = crate::BitReader<Gif7>;
impl Gif7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gif7 {
        match self.bits {
            false => Gif7::B0x0,
            true => Gif7::B0x1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gif7::B0x0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gif7::B0x1
    }
}
#[doc = "transfer complete (TC) flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcif7 {
    #[doc = "0: no TC event"]
    B0x0 = 0,
    #[doc = "1: a TC event occurred"]
    B0x1 = 1,
}
impl From<Tcif7> for bool {
    #[inline(always)]
    fn from(variant: Tcif7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF7` reader - transfer complete (TC) flag for channel 7"]
pub type Tcif7R = crate::BitReader<Tcif7>;
impl Tcif7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcif7 {
        match self.bits {
            false => Tcif7::B0x0,
            true => Tcif7::B0x1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcif7::B0x0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcif7::B0x1
    }
}
#[doc = "half transfer (HT) flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htif7 {
    #[doc = "0: no HT event"]
    B0x0 = 0,
    #[doc = "1: a HT event occurred"]
    B0x1 = 1,
}
impl From<Htif7> for bool {
    #[inline(always)]
    fn from(variant: Htif7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF7` reader - half transfer (HT) flag for channel 7"]
pub type Htif7R = crate::BitReader<Htif7>;
impl Htif7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htif7 {
        match self.bits {
            false => Htif7::B0x0,
            true => Htif7::B0x1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htif7::B0x0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htif7::B0x1
    }
}
#[doc = "transfer error (TE) flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teif7 {
    #[doc = "0: no TE event"]
    B0x0 = 0,
    #[doc = "1: a TE event occurred"]
    B0x1 = 1,
}
impl From<Teif7> for bool {
    #[inline(always)]
    fn from(variant: Teif7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF7` reader - transfer error (TE) flag for channel 7"]
pub type Teif7R = crate::BitReader<Teif7>;
impl Teif7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teif7 {
        match self.bits {
            false => Teif7::B0x0,
            true => Teif7::B0x1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teif7::B0x0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teif7::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - global interrupt flag for channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer complete (TC) flag for channel 1"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer (HT) flag for channel 1"]
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error (TE) flag for channel 1"]
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - global interrupt flag for channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transfer complete (TC) flag for channel 2"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - half transfer (HT) flag for channel 2"]
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transfer error (TE) flag for channel 2"]
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - global interrupt flag for channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - transfer complete (TC) flag for channel 3"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - half transfer (HT) flag for channel 3"]
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - transfer error (TE) flag for channel 3"]
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - global interrupt flag for channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - transfer complete (TC) flag for channel 4"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - half transfer (HT) flag for channel 4"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transfer error (TE) flag for channel 4"]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - global interrupt flag for channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - transfer complete (TC) flag for channel 5"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - half transfer (HT) flag for channel 5"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - transfer error (TE) flag for channel 5"]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - global interrupt flag for channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - transfer complete (TC) flag for channel 6"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - half transfer (HT) flag for channel 6"]
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        Htif6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - transfer error (TE) flag for channel 6"]
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - global interrupt flag for channel 7"]
    #[inline(always)]
    pub fn gif7(&self) -> Gif7R {
        Gif7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - transfer complete (TC) flag for channel 7"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - half transfer (HT) flag for channel 7"]
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        Htif7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - transfer error (TE) flag for channel 7"]
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
