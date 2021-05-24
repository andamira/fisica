//!
//!

#[allow(unused_imports)]
use crate::units::{Length, Mass, Time};
use crate::{Direction, Magnitude};

/// `Momentum`, in [`Mass`] times [`Length`] per [`Time`]: `kg m/s`.
#[derive(Clone, Copy, Debug)]
pub struct Momentum {
    pub m: Magnitude,
    pub d: Direction,
}

/// # Constructors
impl Momentum {
    /// new Momentum
    #[inline]
    pub const fn new(m: Magnitude, d: Direction) -> Self {
        Self { m, d }
    }

    /// new Momentum with undefined direction
    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m, Direction::ZERO)
    }
}

// TODO: prefixes
// impl_prefixes![Momentum, , ];
