//!
//!

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

/// # Constructors
impl Current {
    /// new Current
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

impl_prefixes![Current, A, amperes];
