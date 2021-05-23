//!
//!

#![allow(unused_imports)]
use crate::units::{Distance, Length, Speed, Time};
use crate::Direction;

/// How fast something is moving in a particular direction, in `m/s`.
///
/// This is a vector magnitude.
pub struct Velocity {
    /// module
    pub m: Speed,
    /// Vector
    pub d: Direction,
}

impl Velocity {
    /// New velocity from Speed and a direction
    pub const fn new(s: Speed, d: Direction) -> Self {
        Self { m: s, d }
    }

    /// New velocity from Speed and undefined direction
    pub const fn without_direction(s: Speed) -> Self {
        Self {
            m: s,
            d: Direction::ZERO,
        }
    }
}

/// # `Velocity` formulas
impl Speed {
    // /// Derives the `Speed` from the given [`Distance`] and [`Time`] (`s = d / t`).
    // #[inline]
    // pub fn from_distance_time(d: Distance, t: Time) -> Self {
    //     Self { m: d.m / t.m }
    // }
    //
    // /// (Alias of [from_distance_time][Speed::from_distance_time]).
    // #[inline]
    // pub fn from_time_distance(t: Time, d: Distance) -> Self {
    //     Self::from_distance_time(d, t)
    // }
    //
    // /// Calculates the [`Distance`] given the [`Time`] (`d = s × t`).
    // #[inline]
    // pub fn calc_distance(&self, t: Time) -> Distance {
    //     Length { m: self.m * t.m }
    // }
    //
    // /// Calculates the [`Time`] given the [`Distance`] (`t = d / s`).
    // pub fn calc_time(&self, d: Distance) -> Time {
    //     Time { m: d.m / self.m }
    // }
}

// TODO: prefixes
// impl_prefixes![Speed, km_s, kilometers_second];

// #[cfg(test)]
// mod tests {
//     use {super::*, float_eq::assert_float_eq};
//
//     /// Checks the constants are defined as expected.
//     #[test]
//     fn speed_constants() {
//         //assert_float_eq!(0.0000000000162, Length::PLANCK.as_ym(), r2nd <= Magnitude::EPSILON);
//     }
//
//     /// Checks the formulas behave as expected.
//     #[test]
//     fn speed_formulas() {
//         // Distance, Speed & Time
//         let speed = Speed::from_distance_time(Distance::in_m(300.), Time(25.));
//         assert_float_eq!(12., speed.m, r2nd <= Magnitude::EPSILON);
//         assert_float_eq!(25., speed.calc_time(Length(300.)).m, r2nd <= Magnitude::EPSILON);
//         assert_float_eq!(300., speed.calc_distance(Time(25.)).m, r2nd <= Magnitude::EPSILON);
//     }
// }
