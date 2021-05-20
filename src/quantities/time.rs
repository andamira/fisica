//!
//!

use crate::F;

/// `Time`, in seconds: `s`.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Time_in_physics>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(time)>
#[derive(Clone, Copy, Debug)]
pub struct Time(pub F);

impl Time {
    #[inline]
    pub fn in_minutes(&self) -> F {
        self.0 / 60.
    }

    #[inline]
    pub fn in_hours(&self) -> F {
        self.0 / 3600.
    }

    #[inline]
    pub fn in_days(&self) -> F {
        self.0 / 86_400.
    }
}

impl_prefixes![Time, s, seconds];
