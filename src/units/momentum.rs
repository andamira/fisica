//!
//!

#[allow(unused_imports)]
use crate::units::{Length, Mass, Time};
use crate::{Direction, Magnitude};

/// `Momentum`, in [`Mass`] times [`Length`] per [`Time`]: `kg m/s`.
#[derive(Clone, Copy, Debug)]
pub struct Momentum {
    pub d: Direction,
}

impl Momentum {
    /// New Momentum.
    #[inline]
    pub const fn new(d: Direction) -> Self {
        Self { d }
    }

    /// Returns the magnitude.
    #[inline]
    pub fn m(&self) -> Magnitude {
        self.d.magnitude()
    }
}

// TODO: impl_vector_prefixes![Momentum, , ];
