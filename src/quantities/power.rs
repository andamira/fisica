//!
//!

use crate::{Energy, Time, Work, F};

/// How quickly the [`Energy`] is transferred, or the [`Work`] is done, in `W` (watts).
#[derive(Clone, Copy, Debug)]
pub struct Power(pub F);

impl Power {
    /// From energy transferred (in joules) per [`Time`] (in seconds) (W = J / s).
    ///
    /// See also:
    /// - [Energy::from_power_time] `J = W × s`
    pub fn from_energy_time(e: Energy, t: Time) -> Self {
        Self(e.0 / t.0)
    }
}

impl_prefixes![Power, W, watts];
