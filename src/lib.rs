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

// TODO:
// - katex for the docs
// - more ergonomic epsilon, error margin usage
// - maybe differentiate vector quantities (Force, Velocity, Acceleration, Momentum)
// - take frame of reference into account. e.g.: (p67)
// - energy efficiency pct (p59)
// - add less common base units (solid angle, …)
// - add many constants along multiple orders of magnitude

// TODO: finish adding formulas. E.g.:
//
// ## motion
//
// - kinetic energy (KE)
//   - E = ½ mv²
//   - 2E = mv²
//   - 2E/m = v²
//   - v = sqrt(2E/m)
//
// - gravitational potential energy (GPE)
//   (the height of something dropped, or lifted) (down or up)
//   - KE = GPE
//
// ## mechanics:
//
// - speed (average speed) (the rate that something moves at)
//   v = d/t  (distance divided by time (against time)) [m/s] [meters per second)
// - acceleration
//
// ## waves:
//
// wave equation
//   - v = fλ
//
