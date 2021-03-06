//! Marker traits to allow types to be contained as secrets.

#![allow(unsafe_code)]

// `clippy` currently warns when trait functions could be `const fn`s, but this
// is not actually allowed by the language
#![cfg_attr(feature = "cargo-clippy", allow(clippy::missing_const_for_fn))]

/// Traits for types that are considered buckets of bytes.
mod bytes;

/// Traits for types that should be compared for equality in constant
/// time.
mod constant_eq;

/// Traits for types that can have their underlying storage safely set
/// to any arbitrary bytes.
mod randomizable;

/// Traits for types that can have their underlying storage safely
/// zeroed.
mod zeroable;

pub use bytes::{AsContiguousBytes, Bytes};
pub use constant_eq::ConstantEq;
pub use randomizable::Randomizable;
pub use zeroable::Zeroable;

macro_rules! impls {
    ($($ty:ty),* ; $ns:tt) => {$(
        impls!{prim  $ty}
        impls!{array $ty; $ns}
    )*};

    (prim $ty:ty) => {
        unsafe impl Bytes for $ty {}
    };

    (array $ty:ty; ($($n:tt)*)) => {$(
        #[allow(trivial_casts)]
        unsafe impl Bytes for [$ty; $n] {}
    )*};
}

impls! {
    (), // maybe not super useful, but good as a smoke test

    u8, u16, u32, u64, u128; (

     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37 38 39
    40 41 42 43 44 45 46 47 48 49
    50 51 52 53 54 55 56 57 58 59
    60 61 62 63 64

    // 521-bit (8 * 65.25) keys are a thing (ECDH / ECDSA)
    66

    // "million-bit keys ought to be enough for anybody"
    128 256 384 512 1024 2048 4096 8192
)}
