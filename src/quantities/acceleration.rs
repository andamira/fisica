//!
//!

use crate::{Force, Length, Mass, Time, Velocity, F};

/// The rate of change of [`Velocity`] of a body, in `m/s²` ([`Length`] per [`Time`] squared).
#[derive(Clone, Copy, Debug)]
pub struct Acceleration(pub F);

impl Acceleration {
    /// Derive `Acceleration` from [`Mass`] and [`Force`] (`a = F / m`).
    pub fn from_mass_force(m: Mass, f: Force) -> Self {
        Self(f.0 / m.0)
    }
}

// TODO: prefixes
// impl_prefixes![Acceleration, km_s2, kilometers_second_squared];
