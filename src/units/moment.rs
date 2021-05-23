//!
//!

use crate::units::{Distance, Force, Length};
use crate::{Direction, Magnitude};

/// The turning effect caused by a [`Force`] `F` applied at a [`Distance`] `d`
/// (`M = F × d`), in `Nm` (newton metre).
///
/// τ⃗ = r⃗ × F⃗
///
/// It makes an object rotate around a fixed point called a pivot.
///
/// <https://en.wikipedia.org/wiki/Moment_(physics)>
pub struct Moment {
    pub m: Magnitude,
    pub d: Direction,
}

/// # Constructors
impl Moment {
    /// new Moment
    #[inline]
    pub const fn new(m: Magnitude, d: Direction) -> Self {
        Self { m, d }
    }

    /// new Moment with undefined direction
    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self {
            m,
            d: Direction::ZERO,
        }
    }
}

/// The informal name for Moment
///
/// # External links
/// - <https://physics.stackexchange.com/questions/16389/torque-vs-moment>
/// - <https://www.physicsforums.com/threads/torque-vs-moment.279526/>
pub type Torque = Moment;

/// # Formulas
impl Moment {
    /// Returns the `Moment` of applying a [`Force`] over some [`Distance`]
    /// (`M = F × d`).
    pub fn from_force_distance(f: Force, d: Distance) -> Self {
        Self {
            m: f.m * d.m,
            d: f.d,
        }
    }

    /// (Alias of [from_force_distance][Moment::from_force_distance]).
    #[inline]
    pub fn from_distance_force(d: Distance, f: Force) -> Self {
        Self::from_force_distance(f, d)
    }

    /// Calculates the [`Distance`] for a given [`Force`] (`d = M / F`).
    #[inline]
    pub fn calc_distance(&self, f: Force) -> Distance {
        Length { m: self.m / f.m }
    }

    /// Calculates the [`Force`] for a given [`Distance`] (`F = M / d`).
    #[inline]
    pub fn calc_force(&self, d: Distance) -> Force {
        Force {
            m: self.m / d.m,
            d: self.d,
        }
    }
}

// impl_prefixes![Moment, Nkg, ];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn moment_formulas() {
        // Distance, Moment & Force
        let moment = Moment::from_force_distance(Force::without_direction(30.), Length::new(0.2));
        assert_float_eq!(6., moment.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            0.2,
            moment.calc_distance(Force::without_direction(30.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            30.,
            moment.calc_force(Length::new(0.2)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}
