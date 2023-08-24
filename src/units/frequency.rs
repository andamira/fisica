// fisica::units::frequency
//
//!
//

use crate::Magnitude;

/// `Frequency`, in hertzs: `Hz` (1 per second).
#[derive(Clone, Copy, Debug)]
pub struct Frequency {
    pub m: Magnitude,
}

impl Frequency {
    /// new Frequency
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

impl_scalar_methods![Frequency, Hz, hertzs];
