//!
//!

use crate::{Acceleration, Energy, Force, GravitationalFieldStrength, Speed, Weight, F};

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
    #[inline]
    pub fn from_energy(e: Energy) -> Self {
        Self(Speed::LIGHT_SQUARED.0 / e.0)
    }

    /// Derive `Mass` from [`Force`] and [`Acceleration`] (`m = F / a`).
    #[inline]
    pub fn from_force_acceleration(f: Force, a: Acceleration) -> Self {
        Self(f.0 / a.0)
    }

    /// (Alias of [from_mass_force][Acceleration::from_mass_force]).
    #[inline]
    pub fn from_acceleration_force(a: Acceleration, f: Force) -> Self {
        Self::from_force_acceleration(f, a)
    }

    /// Calculates the [`Force`] given the [`Acceleration`] (`F = m × a`).
    #[inline]
    pub fn calc_force(&self, a: Acceleration) -> Force {
        Force(self.0 * a.0)
    }

    /// Calculates the `Acceleration` given the [`Force`] (`a = F / m`).
    pub fn calc_acceleration(&self, f: Force) -> Acceleration {
        Acceleration(f.0 / self.0)
    }

    /// Derive `Mass` from [`Weight`] and [`GravitationalFieldStrength`] (`m = w / gfs`).
    #[inline]
    pub fn from_weight_gfs(w: Weight, gfs: GravitationalFieldStrength) -> Self {
        Self(w.0 / gfs.0)
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

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Acceleration, Distance, Force, Length, Mass, F};

    /// Checks the formulas behave as expected.
    #[test]
    fn mass_formulas() {
        // Force, Acceleration & Mass
        let mass = Mass::from_force_acceleration(Force(10.), Acceleration(2.));
        assert_float_eq!(5., mass.0, r2nd <= F::EPSILON);
        assert_float_eq!(2., mass.calc_acceleration(Force(10.)).0, r2nd <= F::EPSILON);
        assert_float_eq!(10., mass.calc_force(Acceleration(2.)).0, r2nd <= F::EPSILON);
    }
}
