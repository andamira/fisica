//!
//!
//#![allow(non_snake_case)]

use crate::{Force, Moment, Speed, Time, F};

// pub const ASTRONOMICAL_UNIT: Length = Length::from_metres(149_597_870_700.);

/// The measure of one spatial dimension of an object in `m` (metres).
///
/// Base quantity.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Length>
//
// # Trivia
//
// The format for a sixty-four bit float includes one sign bit, followed by eleven
// bits to specify exponent, followed by fifty two bits for the mantissa.
//
// If we expand the number of exponent bits, then we can choose more orders of magnitude.
// A planck length is around 10-35 meters, while the observable universe is 93 billion
// light years across, or around 10e27 meters. That's a swing of 62 orders of magnitude.
// Eleven exponent bits can handle 2048 orders of magnitude.
//
// The limitation is that, for each order of magnitude, floating point can only handle
// 15 to 17 decimal significant figures. That means if we have an error of around 10e11
// meters when we use it to represent things on the scale of the observable universe.
// That's on the order of the distance from here to the Moon.
//
// If we want to have perfect accuracy at every scale, then we shouldn't even use a
// floating point value; we should use an integer number of Planck lengths. This is
// like just dumping the exponent and sign bit to use nothing but mantissa.
//
// As stated, we need to swing 62 decimal orders of magnitude. In binary, that's
// 62*ln(10)/ln(2) or around 206 bits.
//
// source: <https://www.reddit.com/r/theydidthemath/comments/2p2skt/request_how_many_bits_would_a_floating_point/>
//
#[derive(Clone, Copy, Debug)]
pub struct Length(pub F);

/// (== [`Length`]). How far apart objects are, in `m`.
pub type Distance = Length;

/// (== [`Length`]). Vertical length, in `m`.
pub type Height = Length;

/// # [`Distance`] formulas
impl Distance {
    /// Derives the [`Distance`] from the given [`Time`] and [`Speed`] (`d = s × t`).
    #[inline]
    pub fn from_time_speed(t: Time, s: Speed) -> Self {
        Length(t.0 * s.0)
    }

    /// (Alias of [from_time_speed][Length::from_time_speed]).
    #[inline]
    pub fn from_speed_time(s: Speed, t: Time) -> Self {
        Self::from_time_speed(t, s)
    }

    /// Calculates the `Speed` given the [`Time`] (`s = d / t`).
    pub fn calc_speed(&self, t: Time) -> Speed {
        Speed(self.0 / t.0)
    }

    /// Calculates the [`Time`] given the [`Speed`] (`t = d / s`).
    pub fn calc_time(&self, s: Speed) -> Time {
        Time(self.0 / s.0)
    }

    /// Derives the `Distance` from the given [`Moment`] and [`Force`] (`d = M / F`).
    pub fn from_moment_force(m: Moment, f: Force) -> Self {
        Self(m.0 / f.0)
    }

    /// (Alias of [from_moment_force][Length::from_moment_force]).
    #[inline]
    pub fn from_force_moment(f: Force, m: Moment) -> Self {
        Self::from_moment_force(m, f)
    }

    /// Calculates the [`Moment`] for a given [`Force`] (`M = F × d`).
    #[inline]
    pub fn calc_moment(&self, f: Force) -> Moment {
        Moment(self.0 * f.0)
    }

    /// Calculates the [`Force`] for a given [`Moment`] (`F = M / d`).
    #[inline]
    pub fn calc_force(&self, m: Moment) -> Force {
        Force(m.0 / self.0)
    }
}

/// # `Length` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(length)>
impl Length {
    // subatomic scale

    /// (10e-35) The [Planck length][0] constant.
    ///
    /// [0]:https://en.wikipedia.org/wiki/Planck_length
    pub const PLANCK: Self = Length(1.616255e-35);

    /// (10e-17) Range of the [weak force][0](`10 am`).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Weak_interaction
    pub const WEAK_FORCE_RANGE: Self = Length(1e-17);

    /// (10e-16) Approximate [proton][0] radius (`0.833 fm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Proton
    pub const PROTON_RADIUS: Self = Length(8.33e-16);

    // atomic to cellular scale

    /// (10e-15) [Classical electron radius][0] (`2.8179403227 fm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Classical_electron_radius
    pub const ELECTRON_RADIUS: Self = Length(2.8179403227e-15);

    /// (10e-15) Minimum diameter of the atomic nucleus (`3 fm`).
    pub const ATOMIC_NUCLEUS_DIAMETER_MIN: Self = Length(3e-15);

    /// (10e-14) Maximum diameter of the atomic nucleus (`15 fm`).
    pub const ATOMIC_NUCLEUS_DIAMETER_MAX: Self = Length(1.5e-14);

    /// (10e-12) Wavelength of shortest [X-rays][0] (`5 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/X-ray
    pub const XRAY_SHORTEST_WAVELENGTH: Self = Length(5e-12);

    /// (10e-11) Covalent radius of [helium][0] atom (`28 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Helium
    pub const HELIUM_RADIUS: Self = Length(2.8e-11);

    /// (10e-11) [Bohr radius][0] (`52.9177210903 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Bohr_radius
    pub const BOHR_RADIUS: Self = Length(5.29177210903e-11);

    /// (10e-10) 1 [Ångström][0] (`100 pm`)
    ///
    /// [0]:https://en.wikipedia.org/wiki/Angstrom
    pub const ANGSTROM: Self = Length(1e-10);

    /// (10e-10) Length of a carbon-carbon [covalent bond][0] in diamond (`154 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Bond_length
    pub const COVALENT_BOND_DIAMOND: Self = Length(1.54e-10);

    /// (10e11) Roughly the [distance from the Earth to the Sun][0] (`149.5978707 Tm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Astronomical_unit
    pub const ASTRONOMICAL_UNIT: Self = Length(1.495978707e11);
}

/// # Non SI units conversions
impl Length {
    fn_both_unicode![
        Length,
        "au",
        "[`astronomical units`][Length::ASTRONOMICAL_UNIT]",
        au,
        astronomical_units,
        Self::ASTRONOMICAL_UNIT.0
    ];

    fn_both_unicode![
        Length,
        "`Å`",
        "`ångströms`",
        A,
        angstroms,
        metres,
        1.0e10,
        "10⁻¹⁰"
    ];
}

impl_prefixes![Length, m, metres];

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Distance, Force, Length, Moment, Speed, Time, F};

    /// Checks the constants are defined as expected.
    #[test]
    fn length_constants() {
        assert_float_eq!(0.0000000000162, Length::PLANCK.as_ym(), r2nd <= F::EPSILON);
        assert_float_eq!(10., Length::WEAK_FORCE_RANGE.as_am(), r2nd <= F::EPSILON);
        assert_float_eq!(0.833, Length::PROTON_RADIUS.as_fm(), r2nd <= F::EPSILON);

        assert_float_eq!(
            2.8179403227,
            Length::ELECTRON_RADIUS.as_fm(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(
            3.,
            Length::ATOMIC_NUCLEUS_DIAMETER_MIN.as_fm(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(
            15.,
            Length::ATOMIC_NUCLEUS_DIAMETER_MAX.as_fm(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(
            5.,
            Length::XRAY_SHORTEST_WAVELENGTH.as_pm(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(28., Length::HELIUM_RADIUS.as_pm(), r2nd <= F::EPSILON);
        assert_float_eq!(
            52.9177210903,
            Length::BOHR_RADIUS.as_pm(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(
            154.,
            Length::COVALENT_BOND_DIAMOND.as_pm(),
            r2nd <= F::EPSILON
        );
    }

    /// Checks the formulas behave as expected.
    #[test]
    fn length_formulas() {
        // Distance, Speed & Time
        let distance = Distance::from_time_speed(Time(25.), Speed(12.));
        assert_float_eq!(300., distance.0, r2nd <= F::EPSILON);
        assert_float_eq!(25., distance.calc_time(Speed(12.)).0, r2nd <= F::EPSILON);
        assert_float_eq!(12., distance.calc_speed(Time(25.)).0, r2nd <= F::EPSILON);

        // Distance, Moment & Force
        let distance = Distance::from_moment_force(Moment(6.), Force(30.));
        assert_float_eq!(0.2, distance.0, r2nd <= F::EPSILON);
        assert_float_eq!(6., distance.calc_moment(Force(30.)).0, r2nd <= F::EPSILON);
        assert_float_eq!(30., distance.calc_force(Moment(6.)).0, r2nd <= F::EPSILON);
    }
}
