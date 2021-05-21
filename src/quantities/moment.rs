//!
//!

use crate::{Distance, Force, Length, F};

/// The turning effect caused by a [`Force`] `F` applied at a [`Distance`] `d`
/// (`M = F × d`), in `Nm` (newton metre).
///
/// τ⃗ = r⃗ × F⃗
///
/// It makes an object rotate around a fixed point called a pivot.
///
/// <https://en.wikipedia.org/wiki/Moment_(physics)>
pub struct Moment(pub F);

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
        Self(f.0 * d.0)
    }

    /// (Alias of [from_force_distance][Moment::from_force_distance]).
    #[inline]
    pub fn from_distance_force(d: Distance, f: Force) -> Self {
        Self::from_force_distance(f, d)
    }

    /// Calculates the [`Distance`] for a given [`Force`] (`d = M / F`).
    #[inline]
    pub fn calc_distance(&self, f: Force) -> Distance {
        Length(self.0 / f.0)
    }

    /// Calculates the [`Force`] for a given [`Distance`] (`F = M / d`).
    #[inline]
    pub fn calc_force(&self, d: Distance) -> Force {
        Force(self.0 / d.0)
    }
}

// impl_prefixes![Moment, Nkg, ];

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Distance, Force, Length, Moment, F};

    /// Checks the formulas behave as expected.
    #[test]
    fn moment_formulas() {
        // Distance, Moment & Force
        let moment = Moment::from_force_distance(Force(30.), Length(0.2));
        assert_float_eq!(6., moment.0, r2nd <= F::EPSILON);
        assert_float_eq!(0.2, moment.calc_distance(Force(30.)).0, r2nd <= F::EPSILON);
        assert_float_eq!(30., moment.calc_force(Length(0.2)).0, r2nd <= F::EPSILON);
    }
}
