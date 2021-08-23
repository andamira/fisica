//! A physics library
//!

// waiting for:
// - feature(const_fn_floating_point_arithmetic):
//   https://github.com/rust-lang/rust/issues/57241
// - handle impl section of type definitions:
//   https://github.com/rust-lang/rust/issues/32077

#[macro_use]
mod auto_impls;
pub mod constants;
pub mod engine;
pub mod math;
pub mod units;

pub use math::{Direction, Magnitude, Orientation, Position};
