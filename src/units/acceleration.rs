//!
//!

#[allow(unused_imports)]
use crate::units::{Force, Length, Mass, Speed, Time, Velocity};
use crate::{Direction, Magnitude};

/// The rate at which the [`Velocity`] of a body changes in [`Time`],
/// in [`m`][Length]/[`s`][Time]².
#[derive(Clone, Copy, Debug)]
pub struct Acceleration {
    pub m: Magnitude,
    pub d: Direction,
}

/// # Constructors
impl Acceleration {
    /// new Acceleration
    #[inline]
    pub const fn new(m: Magnitude, d: Direction) -> Self {
        Self { m, d }
    }

    /// new Acceleration with undefined direction
    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m, Direction::ZERO)
    }
}

/// # Formulas
impl Acceleration {
    /// Derives `Acceleration` from the given change in [`Velocity`] and [`Time`] taken
    /// (`a = v / t`).
    #[inline]
    pub fn from_velocity_time(v: Velocity, t: Time) -> Self {
        Self::new(v.m / t.m, v.d)
    }

    /// (Alias of [from_velocity_time][Acceleration::from_velocity_time]).
    #[inline]
    pub fn from_time_velocity(t: Time, v: Velocity) -> Self {
        Self::from_velocity_time(v, t)
    }

    pub fn from_velocities_time(v_initial: Velocity, v_final: Velocity, t: Time) -> Self {
        Self::new((v_final.m - v_initial.m) / t.m, v_final.d + v_initial.d) // CHECK vector substraction? sum?
    }

    /// (Alias of [from_velocities_time][Acceleration::from_velocities_time]).
    pub fn from_time_velocities(t: Time, v_initial: Velocity, v_final: Velocity) -> Self {
        Self::from_velocities_time(v_initial, v_final, t)
    }

    /// Derives `Acceleration` from the given [`Mass`] and [`Force`] (`a = F / m`).
    #[inline]
    pub fn from_mass_force(m: Mass, f: Force) -> Self {
        Self::new(f.m / m.m, f.d)
    }

    /// (Alias of [from_mass_force][Acceleration::from_mass_force]).
    #[inline]
    pub fn from_force_mass(f: Force, m: Mass) -> Self {
        Self::from_mass_force(m, f)
    }

    /// Calculates the [`Mass`] given the [`Force`] (`m = F / a`).
    #[inline]
    pub fn calc_mass(&self, f: Force) -> Mass {
        Mass::new(f.m / self.m)
    }

    /// Calculates the [`Force`] given the [`Mass`] (`F = m × a`).
    #[inline]
    pub fn calc_force(&self, m: Mass) -> Force {
        Force::new(self.m * m.m, self.d)
    }
}

// TODO: prefixes
// impl_prefixes![Acceleration, km_s2, kilometers_second_squared];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn acceleration_formulas() {
        // Force, Acceleration & Mass
        let acceleration =
            Acceleration::from_mass_force(Mass::new(5.), Force::without_direction(10.));
        assert_float_eq!(2., acceleration.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            5.,
            acceleration.calc_mass(Force::without_direction(10.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            10.,
            acceleration.calc_force(Mass::new(5.)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}
