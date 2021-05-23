//!
//!

use crate::Magnitude;

/// `Charge`, in coulombs: `C`.
#[derive(Clone, Copy, Debug)]
pub struct Charge {
    pub m: Magnitude,
}

/// # Constructors
impl Charge {
    /// new Charge
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

/// `Charge` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(charge)>
impl Charge {}

impl_prefixes![Charge, C, coulombs];
