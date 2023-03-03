// fisica
//
//! A physics library
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
//
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[macro_use]
mod auto_impls;

pub mod constants;
pub mod math;
pub mod units;

#[doc(inline)]
pub use math::{Direction, Magnitude, Orientation, Position};
