//!
//!

use crate::{Distance, Length, Speed, F};

/// `Time`, in seconds: `s`.
///
///
/// It is defined as the duration of 9.19263177e9 periods of the radiation
/// corresponding to the transition between two hyperfine levels of the
/// ground state of the caesium 133 atom.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Time_in_physics>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(time)>
#[derive(Clone, Copy, Debug)]
pub struct Time(pub F);

// impl Time {
//     /// `second`
//     pub const NAME: &'static str = "second";
//
//     /// `seconds`
//     pub const UNITS: &'static str = "seconds";
//
//     /// `s`
//     pub const SYMBOL: &'static str = "s";
// }

/// # `Time` formulas
impl Time {
    /// Derives the `Time` from the given [`Distance`] and [`Speed`] (`t = d / s`).
    pub fn from_distance_speed(d: Distance, s: Speed) -> Self {
        Self(d.0 / s.0)
    }

    /// (Alias of [from_distance_speed][Length::from_distance_speed]).
    pub fn from_speed_distance(s: Speed, d: Distance) -> Self {
        Time::from_distance_speed(d, s)
    }

    /// Calculates the `Speed` from the given [`Distance`] (`s = d / t`).
    pub fn calc_speed(&self, d: Distance) -> Speed {
        Speed(d.0 / self.0)
    }

    /// Calculates the [`Distance`] given the [`Speed`] (`d = s × t`).
    #[inline]
    pub fn calc_distance(&self, s: Speed) -> Distance {
        Length(self.0 * s.0)
    }
}

/// # `Time` constants by order of magnitude]
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(time)>
impl Time {
    /// [Julian Year][0], used in astronomy, (`365.25 days`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Julian_year_(astronomy)
    pub const JULIAN_YEAR: Self = Time(31_557_600.);

    /// [Full moon cycle][0] (`411 days 18 hours 49 minutes 35 seconds`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Year#Full_moon_cycle
    pub const FULL_MOON_CYCLE: Self = Time(35578174.777056);

    /// [Draconic Year][0] (`346 days 14 hours 52 minuets 54 seconds`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Year#Draconic_year
    pub const DRACONIC_YEAR: Self = Time(29947974.5562912);

    /// [Lunar Year][0] (`354 days 8 hours 48 minutes 34 seconds`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Lunar_calendar
    pub const LUNAR_YEAR: Self = Time(30617314.848);
}

/// # Non SI units conversions
impl Time {
    fn_both_unicode![Time, "`min`", "`minutes`", min, minutes, 60.];
    fn_both_unicode![Time, "`h`", "`hours`", h, hours, 3600.];
    fn_both_unicode![Time, "`d`", "`days`", d, days, 86_400.];
    fn_both_unicode![Time, "`w`", "`weeks`", w, weeks, 604_800.];
    fn_both_unicode![Time, "`y`", "`years`", y, years, days, 31_536e3, "365"];
    fn_both_unicode![
        Time,
        "`jy`",
        "`julian years`",
        jy,
        julian_years,
        days,
        Time::JULIAN_YEAR.0,
        "365.25"
    ];
}

impl_prefixes![Time, s, seconds];

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Distance, Length, Speed, Time, F};

    /// Checks the formulas behave as expected.
    #[test]
    fn time_formulas() {
        // Distance, Speed & Time
        let time = Time::from_distance_speed(Length(300.), Speed(12.));
        assert_float_eq!(25., time.0, r2nd <= F::EPSILON);
        assert_float_eq!(300., time.calc_distance(Speed(12.)).0, r2nd <= F::EPSILON);
        assert_float_eq!(12., time.calc_speed(Length(300.)).0, r2nd <= F::EPSILON);
    }

    /// Checks the constants are defined as expected.
    #[test]
    fn time_constants() {
        assert_float_eq!(365.25, Time::JULIAN_YEAR.as_days(), r2nd <= F::EPSILON);
        assert_float_eq!(
            411.78443029,
            Time::FULL_MOON_CYCLE.as_days(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(
            346.620075883,
            Time::DRACONIC_YEAR.as_days(),
            r2nd <= F::EPSILON
        );
        assert_float_eq!(354.36707, Time::LUNAR_YEAR.as_days(), r2nd <= F::EPSILON);
    }

    /// Checks the non SI prefixes behave as expected.
    // TODO: test as_ methods
    #[test]
    fn time_non_si() {
        assert_float_eq!(60., Time::in_min(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(3600., Time::in_h(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(86_400., Time::in_d(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(604_800., Time::in_w(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(31_536e3, Time::in_y(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(Time::JULIAN_YEAR.0, Time::in_jy(1.).0, r2nd <= F::EPSILON);
    }
}
