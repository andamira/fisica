//!
//!

use crate::units::{Acceleration, Distance, GravitationalFieldStrength, Length, Mass, Moment};
use crate::{Direction, Magnitude};

/// Any interaction that, when unopposed, will change the motion of an object,
/// measured in `N` (newtons).
///
/// See also:
/// - [Weight]
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Force>
/// - <https://en.wikipedia.org/wiki/Newton_(unit)>
#[derive(Clone, Copy, Debug)]
pub struct Force {
    pub m: Magnitude,
    pub d: Direction,
}

/// # Constructors
impl Force {
    /// new Force
    #[inline]
    pub const fn new(m: Magnitude, d: Direction) -> Self {
        Self { m, d }
    }

    /// new Force with undefined direction
    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m, Direction::ZERO)
    }
}

/// (== [`Force`]) The pull of gravity on an object.
pub type Weight = Force;

/// # Formulas: [`Force`]
impl Force {
    /// Derives the `Force` from the given [`Mass`] and [`Acceleration`] (`F = m × a`).
    pub fn from_mass_acceleration(m: Mass, a: Acceleration) -> Self {
        Self::new(m.m * a.m, a.d)
    }

    /// (Alias of [from_mass_acceleration][Force::from_mass_acceleration]).
    #[inline]
    pub fn from_acceleration_mass(a: Acceleration, m: Mass) -> Self {
        Self::from_mass_acceleration(m, a)
    }

    /// Calculates the [`Mass`] given the [`Acceleration`] (`m = F / a`).
    #[inline]
    pub fn calc_mass(&self, a: Acceleration) -> Mass {
        Mass::new(self.m / a.m)
    }

    /// Calculates the [`Acceleration`] given the [`Mass`] (`a = F / m`).
    #[inline]
    pub fn calc_acceleration(&self, m: Mass) -> Acceleration {
        Acceleration::new(self.m / m.m, self.d)
    }

    /// Derives the `Force` from the given [`Moment`] and [`Distance`] (`F = M / d`).
    pub fn from_moment_distance(m: Moment, d: Distance) -> Self {
        Self::new(m.m / d.m, m.d)
    }

    /// (Alias of [from_moment_distance][Force::from_moment_distance]).
    #[inline]
    pub fn from_distance_moment(d: Distance, m: Moment) -> Self {
        Self::from_moment_distance(m, d)
    }

    /// Calculates the [`Moment`] given the [`Distance`] (`M = F × d`).
    #[inline]
    pub fn calc_moment(&self, d: Distance) -> Moment {
        Moment::new(self.m * d.m, self.d)
    }

    /// Calculates the [`Distance`] given the [`Moment`] (`d = M / F`).
    #[inline]
    pub fn calc_distance(&self, m: Moment) -> Distance {
        Length::new(m.m / self.m)
    }
}

/// # Formulas: [`Weight`]
impl Weight {
    /// Returns the `Weight` given the [`Mass`] and [`GravitationalFieldStrength`]
    /// (`w = m × g`).
    ///
    /// # Example
    ///
    /// A mass of 60 kg would weight 588 N in Earth and 96 N in the Moon (6.125
    /// times less), as heavy as a 9.79 kg mass would feel in Earth.
    ///
    /// ```
    /// # use fisika::units::{Mass, Weight, Gfs};
    /// let mass = Mass::in_kilograms(60.);
    /// let w_earth = Weight::from_mass_gfs(mass, Gfs::in_earth());
    /// let w_moon = Weight::from_mass_gfs(mass, Gfs::in_moon());
    /// let ratio = w_earth.m / w_moon.m;
    /// print!("A mass of {} would weight {} in Earth and {} in the Moon ({} times less)",
    ///     mass, w_earth, w_moon, ratio);
    /// println!(", as heavy as a {} mass would feel in Earth",
    ///     Mass::in_kg(mass.m / ratio));
    /// ```
    ///
    /// # Trivia
    ///
    /// A common home scale in reality measures the Weight (Force), calibrated
    /// to show the Mass in kg, assuming it's being used on Earth's surface.
    pub fn from_mass_gfs(m: Mass, g: GravitationalFieldStrength) -> Self {
        Self::new(m.m * g.m, g.d)
    }

    /// Calculates the [`Mass`] given the [`GravitationalFieldStrength`] (`m = w / g`).
    #[inline]
    pub fn calc_mass_from_gfs(&self, g: GravitationalFieldStrength) -> Mass {
        Mass::new(self.m / g.m)
    }

    /// Calculates the [`GravitationalFieldStrength`] given the [`Mass`] (`g = w / m`).
    #[inline]
    pub fn calc_gfs(&self, m: Mass) -> GravitationalFieldStrength {
        GravitationalFieldStrength::new(self.m / m.m, self.d)
    }
}

impl_vector_methods![Force, N, newtons];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn force_formulas() {
        // Force, Acceleration & Mass
        let force =
            Force::from_mass_acceleration(Mass::new(5.), Acceleration::without_direction(2.));
        assert_float_eq!(10., force.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            5.,
            force.calc_mass(Acceleration::without_direction(2.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            2.,
            force.calc_acceleration(Mass::new(5.)).m,
            r2nd <= Magnitude::EPSILON
        );

        // Distance, Moment & Force
        let force = Force::from_moment_distance(Moment::without_direction(6.), Length::new(0.2));
        assert_float_eq!(30., force.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            0.2,
            force.calc_distance(Moment::without_direction(6.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            6.,
            force.calc_moment(Length::new(0.2)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}
