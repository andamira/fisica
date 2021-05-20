//!
//!

use crate::{
    Acceleration, Energy, Force, GravitationalFieldStrength, Weight, F, SPEED_OF_LIGHT_SQUARED,
};

/// The amount of matter in an object, in `kg` (kilograms).
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Mass>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass)>
#[derive(Clone, Copy, Debug)]
pub struct Mass(pub F);

/// # Formulas
impl Mass {
    /// Returns the `Mass` [equivalent][0] to the given [`Energy`] (`m = E / c²`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Mass–energy_equivalence
    pub fn from_energy(e: Energy) -> Self {
        Self(SPEED_OF_LIGHT_SQUARED / e.0)
    }

    /// Derive `Mass` from [`Weight`] and [`GravitationalFieldStrength`] (`m = w / gfs`).
    pub fn from_weight_gfs(w: Weight, gfs: GravitationalFieldStrength) -> Self {
        Self(w.0 / gfs.0)
    }

    /// Derive `Mass` from [`Force`] and [`Acceleration`] (`m = F / a`).
    pub fn from_force_acceleration(f: Force, a: Acceleration) -> Self {
        Self(f.0 / a.0)
    }

    /// Calculates the [`Weight`] given the [`GravitationalFieldStrength`] (`W = m × g`).
    #[inline]
    pub fn calc_weight(&self, g: GravitationalFieldStrength) -> Weight {
        Force(self.0 * g.0)
    }

    /// Calculates the [`GravitationalFieldStrength`] given the [`Weight`] (`g = w / m`).
    #[inline]
    pub fn calc_gfs(&self, w: Weight) -> GravitationalFieldStrength {
        GravitationalFieldStrength(w.0 / self.0)
    }
}

impl_prefixes_base_kilo![Mass, g, grams];
