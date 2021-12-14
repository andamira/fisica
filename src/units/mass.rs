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

impl Mass {
    /// New Mass.
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

/// # Formulas
impl Mass {
    /// Returns the `Mass` [*equivalent*][0] to the given [`Energy`] (`m = E / c²`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Mass–energy_equivalence
    #[inline]
    pub fn from_energy(e: Energy) -> Self {
        Self::new(Speed::LIGHT_SQUARED.m() / e.m())
    }

    /// Derive `Mass` from [`Force`] and [`Acceleration`] (`m = F / a`).
    #[inline]
    pub fn from_force_acceleration(f: Force, a: Acceleration) -> Self {
        Self::new(f.m() / a.m())
    }

    /// (Alias of [from_mass_force][Acceleration::from_mass_force]).
    #[inline]
    pub fn from_acceleration_force(a: Acceleration, f: Force) -> Self {
        Self::from_force_acceleration(f, a)
    }

    /// Calculates the [`Force`] given the [`Acceleration`] (`F = m × a`).
    #[inline]
    pub fn calc_force(&self, a: Acceleration) -> Force {
        Force::new(a.d * self.m())
    }

    /// Calculates the `Acceleration` given the [`Force`] (`a = F / m`).
    pub fn calc_acceleration(&self, f: Force) -> Acceleration {
        Acceleration::new(f.d / self.m())
    }

    /// Derive `Mass` from [`Weight`] and [`GravitationalFieldStrength`] (`m = w / gfs`).
    #[inline]
    pub fn from_weight_gfs(w: Weight, gfs: GravitationalFieldStrength) -> Self {
        Self::new(w.m() / gfs.m())
    }

    /// Calculates the [`Weight`] given the [`GravitationalFieldStrength`] (`W = m × g`).
    #[inline]
    pub fn calc_weight(&self, g: GravitationalFieldStrength) -> Weight {
        Force::new(g.d * self.m())
    }

    /// Calculates the [`GravitationalFieldStrength`] given the [`Weight`] (`g = w / m`).
    #[inline]
    pub fn calc_gfs(&self, w: Weight) -> GravitationalFieldStrength {
        GravitationalFieldStrength::new(w.d / self.m())
    }
}

/// # `Mass` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass)>
impl Mass {
    /// (10e-31) The mass of the [*electron*][0] (`0.0009 yg`).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Electron
    pub const ELECTRON: Self = Mass::new(9.109_383_70e-31);

    /// (10e-27) The mass of the [*proton*][0] (`1.6 yg`).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Proton
    pub const PROTON: Self = Mass::new(1.672_621_923_69e-27);

    /// (10e-27) The mass of the [*neutron*][0] (`1.6 yg`).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Neutron
    pub const NEUTRON: Self = Mass::new(1.674_927_498e-27);
}

impl_scalar_methods_base_kilo![Mass, g, grams];

#[cfg(test)]
mod tests {
    use crate::Direction;
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn mass_formulas() {
        // Force, Acceleration & Mass
        let mass = Mass::from_force_acceleration(
            Force::new(Direction::new(10., 0., 0.)),
            Acceleration::new(Direction::new(2., 0., 0.)),
        );
        assert_float_eq!(5., mass.m(), r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            2.,
            mass.calc_acceleration(Force::new(Direction::new(10., 0., 0.))).m(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            10.,
            mass.calc_force(Acceleration::new(Direction::new(2., 0., 0.))).m(),
            r2nd <= Magnitude::EPSILON
        );
    }
}
