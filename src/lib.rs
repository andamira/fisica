//! A physics library
//!
//
// External links:
// - <https://en.wikipedia.org/wiki/International_System_of_Units>
// - <https://en.wikipedia.org/wiki/SI_unit>
// - <https://en.wikipedia.org/wiki/Order_of_magnitude>

#![allow(unused_imports, unused_macros, dead_code)]

// waiting for:
// - [feature(const_fn_floating_point_arithmetic)](https://github.com/rust-lang/rust/issues/57241)
// - [handle impl section of type definitions](https://github.com/rust-lang/rust/issues/32077)

mod constants;
#[macro_use]
mod prefixes;
pub(crate) mod quantities;
mod unit;

pub use constants::*;
pub use prefixes::*;
pub use quantities::*;
pub use unit::*;

// The floating point type used for magnitudes
pub(crate) type F = f64; // f32
