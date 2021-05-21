//!
//!

use crate::{Force, Length, Mass, F};

/// The rate of change of [`Velocity`] of a body, in `m/s²` ([`Length`] per [`Time`] squared).
#[derive(Clone, Copy, Debug)]
pub struct Acceleration(pub F);

impl Acceleration {
    /// Derive `Acceleration` from [`Mass`] and [`Force`] (`a = F / m`).
    pub fn from_mass_force(m: Mass, f: Force) -> Self {
        Self(f.0 / m.0)
    }

    /// (Alias of [from_mass_force][Acceleration::from_mass_force]).
    #[inline]
    pub fn from_force_mass(f: Force, m: Mass) -> Self {
        Self::from_mass_force(m, f)
    }

    /// Calculates the [`Mass`] given the [`Force`] (`m = F / a`).
    #[inline]
    pub fn calc_mass(&self, f: Force) -> Mass {
        Mass(f.0 / self.0)
    }

    /// Calculates the [`Force`] given the [`Mass`] (`F = m × a`).
    #[inline]
    pub fn calc_force(&self, m: Mass) -> Force {
        Force(self.0 * m.0)
    }
}

// TODO: prefixes
// impl_prefixes![Acceleration, km_s2, kilometers_second_squared];

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Acceleration, Distance, Force, Length, Mass, F};

    /// Checks the formulas behave as expected.
    #[test]
    fn acceleration_formulas() {
        // Force, Acceleration & Mass
        let acceleration = Acceleration::from_mass_force(Mass(5.), Force(10.));
        assert_float_eq!(2., acceleration.0, r2nd <= F::EPSILON);
        assert_float_eq!(5., acceleration.calc_mass(Force(10.)).0, r2nd <= F::EPSILON);
        assert_float_eq!(10., acceleration.calc_force(Mass(5.)).0, r2nd <= F::EPSILON);
    }
}
