//!
//!

use crate::F;

/// The luminous power emitted by a point light source in a particular direction.
///
/// A measure of the wavelength-weighted power emitted by a light source in a
/// particular direction per unit solid angle, based on the luminosity function,
/// a standardized model of the sensitivity of the human eye.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Luminous_intensity>
/// - <https://en.wikipedia.org/wiki/Candela>
#[derive(Clone, Copy, Debug)]
pub struct Intensity(pub F);

impl_prefixes![Intensity, cd, candelas];
