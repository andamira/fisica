//!
//!

use crate::{Acceleration, Distance, GravitationalFieldStrength, Length, Mass, Moment, F};

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
pub struct Force(pub F);

/// (== [`Force`]) of the pull of gravity on an object, in `N`.
pub type Weight = Force;

/// # Formulas: [`Force`]
impl Force {
    /// Returns the `Force` given the [`Mass`] and [`Acceleration`] (`F = m × a`).
    pub fn from_mass_acceleration(m: Mass, a: Acceleration) -> Self {
        Self(m.0 * a.0)
    }

    /// Returns the `Force` given the [`Moment`] and [`Distance`] (`F = M / d`).
    pub fn from_moment_distance(m: Moment, d: Distance) -> Self {
        Self(m.0 / d.0)
    }

    /// Calculates the [`Moment`] given the [`Distance`] (`M = F × d`).
    #[inline]
    pub fn calc_moment(&self, d: Distance) -> Moment {
        Moment(self.0 * d.0)
    }

    /// Calculates the [`Distance`] given the [`Moment`] (`d = M / F`).
    #[inline]
    pub fn calc_length(&self, m: Moment) -> Distance {
        Length(m.0 / self.0)
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
    /// # use fisika::{Mass, Weight, Gfs};
    /// let mass = Mass::in_kilograms(60.);
    /// let w_earth = Weight::from_mass_gfs(mass, Gfs::in_earth());
    /// let w_moon = Weight::from_mass_gfs(mass, Gfs::in_moon());
    /// let ratio = w_earth.0 / w_moon.0;
    /// print!("A mass of {} would weight {} in Earth and {} in the Moon ({} times less)",
    ///     mass, w_earth, w_moon, ratio);
    /// println!(", as heavy as a {} mass would feel in Earth",
    ///     Mass::in_kg(mass.0 / ratio));
    /// ```
    ///
    /// # Trivia
    ///
    /// A common home scale in reality measures the Weight (Force), calibrated
    /// to show the Mass in kg, assuming it's being used on Earth's surface.
    pub fn from_mass_gfs(m: Mass, g: GravitationalFieldStrength) -> Self {
        Self(m.0 * g.0)
    }

    /// Calculates the [`Mass`] given the [`GravitationalFieldStrength`] (`m = w / g`).
    #[inline]
    pub fn calc_mass(&self, g: GravitationalFieldStrength) -> Mass {
        Mass(self.0 / g.0)
    }

    /// Calculates the [`GravitationalFieldStrength`] given the [`Mass`] (`g = w / m`).
    #[inline]
    pub fn calc_gfs(&self, m: Mass) -> GravitationalFieldStrength {
        GravitationalFieldStrength(self.0 / m.0)
    }
}

impl_prefixes![Force, N, newtons];
