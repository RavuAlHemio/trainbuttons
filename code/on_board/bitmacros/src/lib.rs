#![no_std]

/// Generates a bit mask for a sequence of bits with the given index of the lowest bit and number of
/// bits.
///
/// ```
/// assert_eq!(bit_mask!(0, 2), 0b11);
/// assert_eq!(bit_mask!(3, 4), 0b1111000);
/// ```
#[macro_export]
macro_rules! bit_mask {
    ($lowest_bit_index:expr, $bit_count:expr) => {
        (((1 << $bit_count) - 1) << $lowest_bit_index)
    };
}

/// Extracts a sequence of bits from a value given the index of the lowest bit and the number of
/// bits: `extract_bits!(value, lowest_bit_index, bit_count)`
///
/// The extracted bit sequence is shifted down to bit index 0.
#[macro_export]
macro_rules! extract_bits {
    ($value:expr, $lowest_bit_index:expr, $bit_count:expr) => {
        (($value >> $lowest_bit_index) & $crate::bit_mask!(0, $bit_count))
    };
}

/// Replaces a sequence of bits at a specific location in a register.
///
/// The new value should be passed to this macro unshifted; the macro takes care of shifting it into
/// place before performing the requisite masking and bitwise operations.
#[macro_export]
macro_rules! emplace_bits {
    ($register:expr, $value:expr, $lowest_bit_index:expr, $bit_count:expr) => {
        $register = ($register & (!$crate::bit_mask!($lowest_bit_index, $bit_count))) | (($value << $lowest_bit_index) & $crate::bit_mask!($lowest_bit_index, $bit_count))
    };
}
