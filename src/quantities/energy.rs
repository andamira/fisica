//!
//!

use crate::{Distance, Force, Mass, Power, Speed, Time, F};

/// `Energy`, in joules: `J`.
#[derive(Clone, Copy, Debug)]
pub struct Energy(pub F);

/// (== [`Energy`]) transferred when a [`Force`] moves an object over a [`Distance`],
/// in `J`.
///
/// Also: The amount of force applied in the direction of motion.
pub type Work = Energy;

impl Work {
    /// Work (J) = [`Force`] (N) × [`Distance`] (m)
    pub fn from_force_length(f: Force, d: Distance) -> Self {
        Self(f.0 * d.0)
    }
}

impl Energy {
    /// From [`Power`] (in watts) times [`Time`] (in seconds) `J = W × s`.
    ///
    /// See also:
    ///
    /// - [Power::from_energy_time] `W = J/s`
    pub fn from_power_time(p: Power, t: Time) -> Self {
        Self(p.0 * t.0)
    }

    /// Returns the `Energy` [equivalent][0] to the given [`Mass`] (`E = mc²`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Mass%E2%80%93energy_equivalence
    pub fn from_mass(m: Mass) -> Self {
        Self(m.0 * Speed::LIGHT_SQUARED.0)
    }
}

impl_prefixes![Energy, J, joules];
