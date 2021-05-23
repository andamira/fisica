//!
//!

use crate::Magnitude;

/// `Frequency`, in hertzs: `Hz` (1 per second).
#[derive(Clone, Copy, Debug)]
pub struct Frequency {
    pub m: Magnitude,
}

/// # Constructors
impl Frequency {
    /// new Frequency
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

impl_prefixes![Frequency, Hz, hertzs];
