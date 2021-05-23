//!
//!

#[allow(unused_imports)]
use crate::units::Length;

use crate::Magnitude;

/// `Area`, in `m²` (squared [`Length`]).
///
/// The quantity that expresses the extent of a two-dimensional region, shape,
/// or planar lamina, in the plane.
///
/// External links:
/// - <https://en.wikipedia.org/wiki/Area>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(area)>
#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub m: Magnitude,
}

/// # Constructors
impl Area {
    /// new Area
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

// TODO: prefixes
// impl_prefixes![Area, m2, metres, square];
