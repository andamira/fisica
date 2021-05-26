//!
//!

use crate::units::{Distance, Force, Mass, Power, Speed, Time};
use crate::Magnitude;

/// `Energy`, in joules: `J`.
#[derive(Clone, Copy, Debug)]
pub struct Energy {
    pub m: Magnitude,
}

/// (== [`Energy`]) Energy transferred when a [`Force`] moves an object over a
/// [`Distance`].
///
/// Also: The amount of force applied in the direction of motion.
pub type Work = Energy;

impl Work {
    /// Work (J) = [`Force`] (N) × [`Distance`] (m)
    #[inline]
    pub fn from_force_length(f: Force, d: Distance) -> Self {
        Self::new(f.m * d.m)
    }
}

/// # Constructors
impl Energy {
    /// new Energy
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

/// Formulas
impl Energy {
    /// Derives Energy from the given [`Power`] and [`Time`] (`J = P × t`).
    #[inline]
    pub fn from_power_time(p: Power, t: Time) -> Self {
        Self::new(p.m * t.m)
    }

    /// (Alias of [from_power_time][Energy::from_power_time]).
    #[inline]
    pub fn from_time_power(t: Time, p: Power) -> Self {
        Self::from_power_time(p, t)
    }

    /// Calculates the [`Power`] given the [`Time`] (`P = E / t`).
    #[inline]
    pub fn calc_power(&self, t: Time) -> Power {
        Power::new(self.m / t.m)
    }

    /// Calculates the [`Time`] given the [`Power`] (`t = E / P`).
    #[inline]
    pub fn calc_time(&self, p: Power) -> Time {
        Time::new(self.m / p.m)
    }

    /// Returns the `Energy` [equivalent][0] to the given [`Mass`] (`E = mc²`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Mass%E2%80%93energy_equivalence
    #[inline]
    pub fn from_mass(m: Mass) -> Self {
        Self::new(m.m * Speed::LIGHT_SQUARED.m)
    }
}

impl_scalar_methods![Energy, J, joules];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn energy_formulas() {
        // Energy, Power & Time
        let energy = Energy::from_power_time(Power::new(800.), Time::in_min(3.));
        assert_float_eq!(Energy::in_kJ(144.).m, energy.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            energy.m,
            Energy::from_time_power(Time::in_min(3.), Power::new(800.)).m,
            r2nd <= Magnitude::EPSILON,
        );
        assert_float_eq!(
            800.,
            energy.calc_power(Time::in_min(3.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            Time::in_min(3.).m,
            energy.calc_time(Power::new(800.)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}
