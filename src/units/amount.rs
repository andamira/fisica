//!
//!

#![allow(unused_imports)]
use crate::{Direction, Magnitude};

/// The *amount of substance*, or *chemical amount*, is the number of discrete
/// atomic-scale particles in it, divided by the Avogadro constant.
///
/// A Mole is 6.02214076_e23 particles, which may be atoms, molecules, ions,
/// or electrons.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Amount_of_substance>
/// - <https://en.wikipedia.org/wiki/Mole_(unit)>
/// - <https://en.wikipedia.org/wiki/Avogadro_constant>
#[derive(Clone, Copy, Debug)]
pub struct Amount {
    pub m: Magnitude,
}

/// # Constructors
impl Amount {
    /// new Amount
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

impl_prefixes![Amount, mol, moles];
