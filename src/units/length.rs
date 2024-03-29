// fisica::units::length
//
//!
//

use crate::units::{Force, Moment, Speed, Time};
use crate::Magnitude;

/// [*Length*][0] is the measure of one spatial dimension of an object, in `m` (metres).
///
/// [0]: https://en.wikipedia.org/wiki/Length
#[derive(Clone, Copy, Debug)]
pub struct Length {
    pub m: Magnitude,
}

impl Length {
    /// New Length.
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

/// (== [`Length`]) How far apart objects are.
pub type Distance = Length;

/// (== [`Length`]) Vertical length.
pub type Height = Length;

/// # [`Distance`] formulas
impl Distance {
    /// Derives the [`Distance`] from the given [`Time`] and [`Speed`] (`d = s × t`).
    #[inline]
    pub fn from_time_speed(t: Time, s: Speed) -> Self {
        Length::new(t.m() * s.m())
    }

    /// (Alias of [from_time_speed][Length::from_time_speed]).
    #[inline]
    pub fn from_speed_time(s: Speed, t: Time) -> Self {
        Self::from_time_speed(t, s)
    }

    /// Calculates the `Speed` given the [`Time`] (`s = d / t`).
    pub fn calc_speed(&self, t: Time) -> Speed {
        Speed::new(self.m() / t.m())
    }

    /// Calculates the [`Time`] given the [`Speed`] (`t = d / s`).
    pub fn calc_time(&self, s: Speed) -> Time {
        Time::new(self.m() / s.m())
    }

    /// Derives the `Distance` from the given [`Moment`] and [`Force`] (`d = M / F`).
    pub fn from_moment_force(m: Moment, f: Force) -> Self {
        Self::new(m.m() / f.m())
    }

    /// (Alias of [from_moment_force][Length::from_moment_force]).
    #[inline]
    pub fn from_force_moment(f: Force, m: Moment) -> Self {
        Self::from_moment_force(m, f)
    }

    /// Calculates the [`Moment`] given the [`Force`] (`M = F × d`).
    #[inline]
    pub fn calc_moment(&self, f: Force) -> Moment {
        Moment::new(f.d * self.m())
    }

    /// Calculates the [`Force`] given the [`Moment`] (`F = M / d`).
    #[inline]
    pub fn calc_force(&self, m: Moment) -> Force {
        Force::new(m.d / self.m())
    }
}

/// # `Length` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(length)>
impl Length {
    // subatomic scale

    /// (10e-35) The [*Planck length*][0] constant.
    ///
    /// [0]:https://en.wikipedia.org/wiki/Planck_length
    pub const PLANCK: Self = Length::new(1.616255e-35);

    /// (10e-17) Range of the [*weak force*][0](`10 am`).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Weak_interaction
    pub const WEAK_FORCE_RANGE: Self = Length::new(1e-17);

    /// (10e-16) Approximate [*proton*][0] radius (`0.833 fm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Proton
    pub const PROTON_RADIUS: Self = Length::new(8.33e-16);

    // atomic to cellular scale

    /// (10e-15) [*Classical electron radius*][0] (`2.8179403227 fm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Classical_electron_radius
    pub const ELECTRON_RADIUS: Self = Length::new(2.8179403227e-15);

    /// (10e-15) Minimum diameter of the atomic nucleus (`3 fm`).
    pub const ATOMIC_NUCLEUS_DIAMETER_MIN: Self = Length::new(3e-15);

    /// (10e-14) Maximum diameter of the atomic nucleus (`15 fm`).
    pub const ATOMIC_NUCLEUS_DIAMETER_MAX: Self = Length::new(1.5e-14);

    /// (10e-12) Wavelength of shortest [*X-rays*][0] (`5 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/X-ray
    pub const XRAY_SHORTEST_WAVELENGTH: Self = Length::new(5e-12);

    /// (10e-11) Covalent radius of [*helium*][0] atom (`28 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Helium
    pub const HELIUM_RADIUS: Self = Length::new(2.8e-11);

    /// (10e-11) [*Bohr radius*][0] (`52.9177210903 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Bohr_radius
    pub const BOHR_RADIUS: Self = Length::new(5.29177210903e-11);

    /// (10e-10) 1 [*Ångström*][0] (`100 pm`)
    ///
    /// [0]:https://en.wikipedia.org/wiki/Angstrom
    pub const ANGSTROM: Self = Length::new(1e-10);

    /// (10e-10) Length of a carbon-carbon [*covalent bond*][0] in diamond (`154 pm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Bond_length
    pub const COVALENT_BOND_DIAMOND: Self = Length::new(1.54e-10);

    /// (10e11) Roughly the [*distance from the Earth to the Sun*][0] (`149.5978707 Tm`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Astronomical_unit
    pub const ASTRONOMICAL_UNIT: Self = Length::new(1.495978707e11);
}

/// # Non SI units conversions
impl Length {
    scalar_methods![
        Length,
        qa = au,
        Qa = astronomical_units,
        qu = "au",
        Qu = "[astronomical units][Length::ASTRONOMICAL_UNIT]",
        f = Self::ASTRONOMICAL_UNIT.m(),
        fu = "149.5978707",
        bu = "Tm"
    ];

    scalar_methods![
        Length,
        qa = A,
        Qa = angstroms,
        qu = "Å",
        Qu = "ångströms",
        f = 1.0e10,
        fu = "10⁻¹⁰",
        bu = "metres"
    ];
}

impl_scalar_methods![Length, m, metres];

#[cfg(test)]
mod tests {
    use crate::Direction;
    use {super::*, float_eq::assert_float_eq};

    /// Checks the constants are defined as expected.
    #[test]
    fn length_constants() {
        assert_float_eq!(
            0.00000000001616255,
            Length::PLANCK.as_ym(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            10.,
            Length::WEAK_FORCE_RANGE.as_am(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            0.833,
            Length::PROTON_RADIUS.as_fm(),
            r2nd <= Magnitude::EPSILON
        );

        assert_float_eq!(
            2.8179403227,
            Length::ELECTRON_RADIUS.as_fm(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            3.,
            Length::ATOMIC_NUCLEUS_DIAMETER_MIN.as_fm(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            15.,
            Length::ATOMIC_NUCLEUS_DIAMETER_MAX.as_fm(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            5.,
            Length::XRAY_SHORTEST_WAVELENGTH.as_pm(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            28.,
            Length::HELIUM_RADIUS.as_pm(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            52.9177210903,
            Length::BOHR_RADIUS.as_pm(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            154.,
            Length::COVALENT_BOND_DIAMOND.as_pm(),
            r2nd <= Magnitude::EPSILON
        );
    }

    /// Checks the formulas behave as expected.
    #[test]
    fn length_formulas() {
        // Distance, Speed & Time
        let distance = Distance::from_time_speed(Time::new(25.), Speed::new(12.));
        assert_float_eq!(300., distance.m(), r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            25.,
            distance.calc_time(Speed::new(12.)).m(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            12.,
            distance.calc_speed(Time::new(25.)).m(),
            r2nd <= Magnitude::EPSILON
        );

        // Distance, Moment & Force
        let distance = Distance::from_moment_force(
            Moment::new(Direction::new(6., 0., 0.)),
            Force::new(Direction::new(30., 0., 0.)),
        );
        assert_float_eq!(0.2, distance.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            6.,
            distance
                .calc_moment(Force::new(Direction::new(30., 0., 0.)))
                .m(),
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            30.,
            distance
                .calc_force(Moment::new(Direction::new(6., 0., 0.)))
                .m(),
            r2nd <= Magnitude::EPSILON
        );
    }
}
