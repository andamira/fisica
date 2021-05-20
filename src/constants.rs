// fisika src/constants.rs
//!
//!

use crate::F;

// CONSTANTS
//
// https://en.wikipedia.org/wiki/Physical_constant

/// The speed of light, in `m/s`.
///
/// See also:
/// - [`SPEED_OF_LIGHT_SQUARED`]
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Speed_of_light>
pub const SPEED_OF_LIGHT: F = 299_792_458.;

/// The [`SPEED_OF_LIGHT`], squared.
///
/// See also:
/// - [`SPEED_OF_LIGHT`]
pub const SPEED_OF_LIGHT_SQUARED: F = 89_875_517_873_681_764.;

/// The Coulomb constant, the electric force constant, or the electrostatic constant
///
/// 8987551792.3(14) `N × m²/C²` (kg × m3 × s^−2 × C^−2)
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Coulomb_constant>
pub const COULOMB_CONSTANT: F = 8_987_551_792.3;
