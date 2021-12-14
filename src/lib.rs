//! A physics library
//!

#[macro_use]
mod auto_impls;

pub mod constants;
pub mod math;
pub mod units;

#[doc(inline)]
pub use math::{Direction, Magnitude, Orientation, Position};
