// fisica::units::density
//
//!
//

#[allow(unused_imports)]
use crate::units::{Mass, Volume};
use crate::Magnitude;

/// [`Mass`] per unit [`Volume`], in `kg/m³`.
///
/// ρ = m / V
#[derive(Clone, Copy, Debug)]
pub struct Density {
    pub m: Magnitude,
}

impl Density {
    /// new Density.
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    /// Returns the magnitude.
    #[inline]
    pub const fn m(&self) -> Magnitude {
        self.m
    }
}

impl_scalar_methods_2units_base_kilo![
    Density,
    q1a = g,
    q2a = m3,
    q1u = "g",
    q2u = "m³",
    Q1a = grams,
    Q2a = metre_cubed,
    Ja = per,
    Q1u = "grams",
    Q2u = "metre cubed"
];
