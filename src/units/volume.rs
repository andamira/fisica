//!
//!

#[allow(unused_imports)]
use crate::units::Length;
use crate::Magnitude;

/// `Volume`, in `m³` (cubic [`Length`]).
#[derive(Clone, Copy, Debug)]
pub struct Volume {
    pub m: Magnitude,
}

/// # Constructors
impl Volume {
    /// new Volume
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
// impl_prefixes![Volume, m3, metres, cubic];
