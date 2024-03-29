// fisica::units::intensity
//
//!
//

use crate::Magnitude;

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
pub struct Intensity {
    pub m: Magnitude,
}

impl Intensity {
    /// New Intensity.
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    /// Returns the magnitude.
    #[inline]
    pub const fn m(&self) -> Magnitude {
        self.m
    }
}

impl_scalar_methods![Intensity, cd, candelas];
