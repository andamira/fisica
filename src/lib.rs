//! A physics library
//!
// waiting for:
// - [feature(const_fn_floating_point_arithmetic)](https://github.com/rust-lang/rust/issues/57241)
// - [handle impl section of type definitions](https://github.com/rust-lang/rust/issues/32077)

#[macro_use]
pub mod auto_impls;
pub mod constants;
pub mod engine;
pub mod units;

// f64, Dvec3, Dquat, DMat3
// f32, Vec3, Quat, Mat3

/// The floating point type used for magnitudes
pub type Magnitude = f64;

/// Direction
pub type Direction = glam::DVec3;

/// Position
pub type Position = glam::DVec3;

/// Orientation
pub type Orientation = glam::DQuat;

/// Rotatation Matrix
pub type Matrix = glam::DMat3;
