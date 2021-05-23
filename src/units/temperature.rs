//!
//!

use crate::Magnitude;

/// Absolute temperature, in `K` (kelvin).
#[derive(Clone, Copy, Debug)]
pub struct Temperature {
    pub m: Magnitude,
}

/// # Constructors
impl Temperature {
    /// new Temperature
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

impl_prefixes![Temperature, K, kelvins];
