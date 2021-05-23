//! A physics library
//!
//
// External links:
// - <https://en.wikipedia.org/wiki/International_System_of_Units>
// - <https://en.wikipedia.org/wiki/SI_unit>
// - <https://en.wikipedia.org/wiki/Order_of_magnitude>

// #![allow(unused_imports, unused_macros, dead_code)]

// waiting for:
// - [feature(const_fn_floating_point_arithmetic)](https://github.com/rust-lang/rust/issues/57241)
// - [handle impl section of type definitions](https://github.com/rust-lang/rust/issues/32077)

pub mod constants;
#[macro_use]
mod prefixes;
pub mod units;

// pub use constants::*;
// pub use prefixes::*;
// pub use quantities::*;
// pub use unit::*;

/// The floating point type used for magnitudes
pub type Magnitude = f64;

/// The vector type used for directions
///
/// 3 dimensional, Just ignore the unneeded dimensions.
pub type Direction = glam::DVec3; // == f64 (Vec3 == f32)

///
pub mod kinematics {
    pub use crate::units::{Distance, Length, Speed, Time};
}

///
pub mod dynamics {
    pub use crate::units::{Force, Velocity};
}
