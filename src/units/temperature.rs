//!
//!

use crate::Magnitude;

/// Absolute temperature, in `K` (kelvin).
#[derive(Clone, Copy, Debug)]
pub struct Temperature {
    pub m: Magnitude,
}

impl Temperature {
    /// New Temperature.
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    /// Returns the magnitude.
    #[inline]
    pub const fn m(&self) -> Magnitude {
        self.m
    }
}

impl_scalar_methods![Temperature, K, kelvins];
