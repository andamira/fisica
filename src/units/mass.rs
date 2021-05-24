//!
//!

use crate::units::{Acceleration, Energy, Force, GravitationalFieldStrength, Speed, Weight};
use crate::Magnitude;

/// The amount of matter in an object, in `kg` (kilograms).
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Mass>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass)>
#[derive(Clone, Copy, Debug)]
pub struct Mass {
    pub m: Magnitude,
}

/// # Constructors
impl Mass {
    /// new Mass
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

/// # Formulas
impl Mass {
    /// Returns the `Mass` [equivalent][0] to the given [`Energy`] (`m = E / c²`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Mass–energy_equivalence
    #[inline]
    pub fn from_energy(e: Energy) -> Self {
        Self::new(Speed::LIGHT_SQUARED.m / e.m)
    }

    /// Derive `Mass` from [`Force`] and [`Acceleration`] (`m = F / a`).
    #[inline]
    pub fn from_force_acceleration(f: Force, a: Acceleration) -> Self {
        Self::new(f.m / a.m)
    }

    /// (Alias of [from_mass_force][Acceleration::from_mass_force]).
    #[inline]
    pub fn from_acceleration_force(a: Acceleration, f: Force) -> Self {
        Self::from_force_acceleration(f, a)
    }

    /// Calculates the [`Force`] given the [`Acceleration`] (`F = m × a`).
    #[inline]
    pub fn calc_force(&self, a: Acceleration) -> Force {
        Force::new(self.m * a.m, a.d)
    }

    /// Calculates the `Acceleration` given the [`Force`] (`a = F / m`).
    pub fn calc_acceleration(&self, f: Force) -> Acceleration {
        Acceleration::new(f.m / self.m, f.d)
    }

    /// Derive `Mass` from [`Weight`] and [`GravitationalFieldStrength`] (`m = w / gfs`).
    #[inline]
    pub fn from_weight_gfs(w: Weight, gfs: GravitationalFieldStrength) -> Self {
        Self::new(w.m / gfs.m)
    }

    /// Calculates the [`Weight`] given the [`GravitationalFieldStrength`] (`W = m × g`).
    #[inline]
    pub fn calc_weight(&self, g: GravitationalFieldStrength) -> Weight {
        Force::new(self.m * g.m, g.d)
    }

    /// Calculates the [`GravitationalFieldStrength`] given the [`Weight`] (`g = w / m`).
    #[inline]
    pub fn calc_gfs(&self, w: Weight) -> GravitationalFieldStrength {
        GravitationalFieldStrength::new(w.m / self.m, w.d)
    }
}

impl_prefixes_base_kilo![Mass, g, grams];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn mass_formulas() {
        // Force, Acceleration & Mass
        let mass = Mass::from_force_acceleration(
            Force::without_direction(10.),
            Acceleration::without_direction(2.),
        );
        assert_float_eq!(5., mass.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            2.,
            mass.calc_acceleration(Force::without_direction(10.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            10.,
            mass.calc_force(Acceleration::without_direction(2.)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}