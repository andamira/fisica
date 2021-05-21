//!
//!

use crate::F;

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

/// # Constants by order of magnitude
impl Time {
    /// [Julian Year](https://en.wikipedia.org/wiki/Julian_year_(astronomy)),
    /// used in astronomy, (`365.25 days`).
    pub const JULIAN_YEAR: Self = Time(31_557_600.);

    /// [Full moon cycle](https://en.wikipedia.org/wiki/Year#Full_moon_cycle)
    /// (`411 days 18 hours 49 minutes 35 seconds`)
    pub const FULL_MOON_CYCLE: Self = Time(35578174.777056);

    /// [Draconic Year](https://en.wikipedia.org/wiki/Year#Draconic_year),
    /// (`346 days 14 hours 52 minuets 54 seconds`)
    pub const DRACONIC_YEAR: Self = Time(29947974.5562912);

    /// [Lunar Year](https://en.wikipedia.org/wiki/Lunar_calendar)
    /// (`354 days 8 hours 48 minutes 34 seconds`)
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

    use crate::{Time, F};

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

    #[test]
    fn time_non_si() {
        assert_float_eq!(60., Time::in_m(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(3600., Time::in_h(1.).0, r2nd <= F::EPSILON);
        assert_float_eq!(86_400., Time::in_d(1.).0, r2nd <= F::EPSILON);
    }
}
