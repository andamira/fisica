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

impl_vector_methods_3units_1base_kilo![
    Momentum,
    q1a = g,
    q2a = m,
    q3a = s,
    Q1a = grams,
    Q2a = metres,
    Q3a = second,
    Ja1 = "",
    Ja2 = per
];
