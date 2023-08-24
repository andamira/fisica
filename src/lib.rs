// fisica
//
//! A physics library
//

#![warn(clippy::all)]
#![cfg_attr(not(feature = "alloc"), allow(unused_macros))]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod auto_impls;

pub mod constants;
pub mod math;
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod units;

#[doc(inline)]
pub use math::{Direction, Magnitude, Orientation, Position};
