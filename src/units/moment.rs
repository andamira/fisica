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
#[derive(Clone, Copy, Debug)]
pub struct Moment {
    pub d: Direction,
}

impl Moment {
    /// New Moment.
    #[inline]
    pub const fn new(d: Direction) -> Self {
        Self { d }
    }

    /// Returns the magnitude.
    #[inline]
    pub fn m(&self) -> Magnitude {
        self.d.magnitude()
    }
}

/// (== [`Moment`]) The informal name for Moment
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
        Self::new(f.d * d.m())
    }

    /// (Alias of [from_force_distance][Moment::from_force_distance]).
    #[inline]
    pub fn from_distance_force(d: Distance, f: Force) -> Self {
        Self::from_force_distance(f, d)
    }

    /// Calculates the [`Distance`] for a given [`Force`] (`d = M / F`).
    #[inline]
    pub fn calc_distance(&self, f: Force) -> Distance {
        Length::new(self.m() / f.m())
    }

    /// Calculates the [`Force`] for a given [`Distance`] (`F = M / d`).
    #[inline]
    pub fn calc_force(&self, d: Distance) -> Force {
        Force::new(self.d / d.m())
    }
}

impl_vector_methods_2units![Moment, q1a = N, q2a = m, Q1a = newtons, Q2a = metre, Ja = per];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn moment_formulas() {
        // Distance, Moment & Force
        let moment =
            Moment::from_force_distance(Force::new(Direction::new(30., 0., 0.)), Length::new(0.2));
        assert_float_eq!(6., moment.m(), r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            0.2,
            moment.calc_distance(Force::new(Direction::new(30., 0., 0.))).m(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(30., moment.calc_force(Length::new(0.2)).m(), r2nd <= Magnitude::EPSILON);
    }
}
