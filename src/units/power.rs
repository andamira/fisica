//!
//!

#[allow(unused_imports)]
use crate::units::{Energy, Time, Work};
use crate::Magnitude;

/// How quickly the [`Energy`] is transferred, or the [`Work`] is done,
/// in `W` (watts).
#[derive(Clone, Copy, Debug)]
pub struct Power {
    pub m: Magnitude,
}

/// # Constructors
impl Power {
    /// new Power
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
impl Power {
    /// Derives the Power from the given [`Energy`] and [`Time`] (`P = E / t`).
    pub fn from_energy_time(e: Energy, t: Time) -> Self {
        Self::new(e.m / t.m)
    }

    /// (Alias of [from_energy_time][Power::from_energy_time]).
    pub fn from_time_energy(t: Time, e: Energy) -> Self {
        Self::from_energy_time(e, t)
    }

    /// Calculates the [`Energy`] given the [`Time`] (`E = P × t`).
    #[inline]
    pub fn calc_energy(&self, t: Time) -> Energy {
        Energy::new(self.m * t.m)
    }

    /// Calculates the [`Time`] given the [`Energy`] (`t = E / P`).
    #[inline]
    pub fn calc_time(&self, e: Energy) -> Time {
        Time::new(e.m / self.m)
    }
}

impl_scalar_methods![Power, W, watts];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn power_formulas() {
        // Energy, Power & Time
        let power = Power::from_energy_time(Energy::in_kJ(144.), Time::in_min(3.));
        assert_float_eq!(800., power.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            power.m,
            Power::from_time_energy(Time::in_min(3.), Energy::in_kJ(144.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            Energy::in_kJ(144.).m,
            power.calc_energy(Time::in_min(3.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            Time::in_min(3.).m,
            power.calc_time(Energy::in_kJ(144.)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}
