//!
//!

use crate::Magnitude;

/// `Pressure`, in pascals: `Pa`.
#[derive(Clone, Copy, Debug)]
pub struct Pressure {
    pub m: Magnitude,
}

/// # Constructors
impl Pressure {
    /// new Pressure
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

impl_prefixes![Pressure, Pa, pascals];
