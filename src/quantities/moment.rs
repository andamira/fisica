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

    // TODO: https://www.mathsisfun.com/physics/moment-torque.html

    /// Calculates the [`Distance`] for a given [`Force`] (`d = M / F`).
    #[inline]
    pub fn calc_length(&self, f: Force) -> Distance {
        Length(self.0 / f.0)
    }

    /// Calculates the [`Force`] for a given [`Distance`] (`F = M / d`).
    #[inline]
    pub fn calc_force(&self, d: Distance) -> Force {
        Force(self.0 / d.0)
    }
}

// TODO: prefixes
// # is it meganewtons per kilogram? newtons per milligram?
// impl_prefixes![Moment, Nkg, ];
