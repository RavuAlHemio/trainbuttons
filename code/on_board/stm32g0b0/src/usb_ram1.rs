#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_double_buffered: [u8; 0x40],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Descriptors if double buffering is enabled."]
    #[inline(always)]
    pub const fn double_buffered(&self, n: usize) -> &DoubleBuffered {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Descriptors if double buffering is enabled."]
    #[inline(always)]
    pub fn double_buffered_iter(&self) -> impl Iterator<Item = &DoubleBuffered> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(8 * n).cast() })
    }
    #[doc = "0x00..0x40 - Descriptors if double buffering is disabled."]
    #[inline(always)]
    pub const fn single_buffered(&self, n: usize) -> &SingleBuffered {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Descriptors if double buffering is disabled."]
    #[inline(always)]
    pub fn single_buffered_iter(&self) -> impl Iterator<Item = &SingleBuffered> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(8 * n).cast() })
    }
}
#[doc = "Descriptors if double buffering is disabled."]
pub use self::single_buffered::SingleBuffered;
#[doc = r"Cluster"]
#[doc = "Descriptors if double buffering is disabled."]
pub mod single_buffered;
#[doc = "Descriptors if double buffering is enabled."]
pub use self::double_buffered::DoubleBuffered;
#[doc = r"Cluster"]
#[doc = "Descriptors if double buffering is enabled."]
pub mod double_buffered;
