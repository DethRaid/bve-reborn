//! Random functions needed for various parts of the parser

pub use loose_numbers::*;
pub use numeric_bool::*;

mod loose_numbers;
mod numeric_bool;

pub(in crate::parse) const fn some_false() -> Option<LooseNumericBool> {
    Some(LooseNumericBool(false))
}

pub(in crate::parse) const fn some_zero_u8() -> Option<LooseNumber<u8>> {
    Some(LooseNumber(0))
}

pub(in crate::parse) const fn some_u8_max() -> Option<LooseNumber<u8>> {
    Some(LooseNumber(255))
}

pub(in crate::parse) const fn some_zero_u16() -> Option<LooseNumber<u16>> {
    Some(LooseNumber(0))
}

pub(in crate::parse) const fn some_zero_f32() -> Option<LooseNumber<f32>> {
    Some(LooseNumber(0.0))
}

pub(in crate::parse) const fn some_one_f32() -> Option<LooseNumber<f32>> {
    Some(LooseNumber(1.0))
}

pub(in crate::parse) fn some_string() -> Option<String> {
    Some(String::new())
}
