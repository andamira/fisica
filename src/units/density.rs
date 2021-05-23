//!
//!

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

/// # Constructors
impl Density {
    /// new Density
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
// impl_prefixes![Density, , ];
