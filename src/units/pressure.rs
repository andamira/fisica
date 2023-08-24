// fisica::units::pressure
//
//!
//

use crate::Magnitude;

/// `Pressure`, in pascals: `Pa`.
#[derive(Clone, Copy, Debug)]
pub struct Pressure {
    pub m: Magnitude,
}

impl Pressure {
    /// New Pressure.
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

impl_scalar_methods![Pressure, Pa, pascals];
