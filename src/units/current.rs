// fisica::units::current
//
//!
//

use crate::Magnitude;

/// `Current`, in amperes (amps): `A`.
///
/// Derived quantity.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Electric_current>
#[derive(Clone, Copy, Debug)]
pub struct Current {
    pub m: Magnitude,
}

impl Current {
    /// new Current
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

impl_scalar_methods![Current, A, amperes];
