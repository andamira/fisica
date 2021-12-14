//! A physics library
//!

#[macro_use]
mod auto_impls;

pub mod constants;
// pub mod engine;
pub mod math;
pub mod units;

pub use math::{Direction, Magnitude, Orientation, Position};
