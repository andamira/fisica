//! Physical constants
//!
//! Many [unit magnitudes][crate::units] have their own associated constants.
//! E.g. [`Speed::SOUND`][crate::units::Speed::SOUND].
//!
//! The ones defined in this module, doesn't pertain to any magnitude.
//!

use crate::Magnitude;

// CONSTANTS
//
// https://en.wikipedia.org/wiki/Physical_constant
// https://en.wikipedia.org/wiki/List_of_physical_constants

/// The [Coulomb constant][0], the electric force constant,
/// or the electrostatic constant.
///
/// `8.987551792.3×10⁹ N×m²/C²`
///
/// [0]:https://en.wikipedia.org/wiki/Coulomb_constant
//
// https://www.johndcook.com/blog/2021/03/31/coulombs-constant/
// https://physics.stackexchange.com/questions/93588/why-does-coulombs-constant-have-units
pub const COULOMB_CONSTANT: Magnitude = 8_987_551_792.3;
