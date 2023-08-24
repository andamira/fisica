//!
//!

use crate::units::{Distance, Energy, Length, Power, Speed};
use crate::Magnitude;

/// `Time`, in seconds: `s`.
///
///
/// It is defined precisely as the duration of 9.19263177e9 periods of the
/// radiation corresponding to the transition between two hyperfine levels
/// of the ground state of the caesium 133 atom.
///
/// More practically, it is the time of a single swing for a pendulum in Earth
/// 0.994m long.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Time_in_physics>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(time)>
#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub m: Magnitude,
}

impl Time {
    /// New Time.
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
        Self::new(d.m / s.m)
    }

    /// (Alias of [from_distance_speed][Time::from_distance_speed]).
    pub fn from_speed_distance(s: Speed, d: Distance) -> Self {
        Time::from_distance_speed(d, s)
    }

    /// Calculates the `Speed` from the given [`Distance`] (`s = d / t`).
    pub fn calc_speed(&self, d: Distance) -> Speed {
        Speed::new(d.m / self.m)
    }

    /// Calculates the [`Distance`] given the [`Speed`] (`d = s × t`).
    #[inline]
    pub fn calc_distance(&self, s: Speed) -> Distance {
        Length::new(self.m * s.m)
    }

    /// Derives Time from the given [`Energy`] and [`Power`] (`t = E / P`).
    #[inline]
    pub fn from_energy_power(e: Energy, p: Power) -> Self {
        Self::new(e.m / p.m)
    }

    /// (Alias of [from_energy_power][Time::from_energy_power]).
    #[inline]
    pub fn from_power_energy(p: Power, e: Energy) -> Self {
        Self::from_energy_power(e, p)
    }

    /// Calculates the [`Power`] given the [`Energy`] (`P = E / t`).
    #[inline]
    pub fn calc_power(&self, e: Energy) -> Power {
        Power::new(e.m / self.m)
    }

    /// Calculates the [`Energy`] given the [`Power`] (`E = P * t`).
    #[inline]
    pub fn calc_energy(&self, p: Power) -> Time {
        Time::new(self.m * p.m)
    }
}

/// # `Time` constants by order of magnitude]
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(time)>
impl Time {
    /// [*Julian Year*][0], used in astronomy, (`365.25 days`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Julian_year_(astronomy)
    pub const JULIAN_YEAR: Self = Time::new(31_557_600.);

    /// [*Full moon cycle*][0] (`411 days 18 hours 49 minutes 35 seconds`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Year#Full_moon_cycle
    pub const FULL_MOON_CYCLE: Self = Time::new(35578174.777056);

    /// [*Draconic Year*][0] (`346 days 14 hours 52 minutes 54 seconds`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Year#Draconic_year
    pub const DRACONIC_YEAR: Self = Time::new(29947974.5562912);

    /// [*Lunar Year*][0] (`354 days 8 hours 48 minutes 34 seconds`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Lunar_calendar
    pub const LUNAR_YEAR: Self = Time::new(30617314.848);
}

/// # Non SI units conversions
impl Time {
    scalar_methods![Time, qa = min, Qa = minutes, f = 60., fu = "60", bu = s];
    scalar_methods![Time, qa = h, Qa = hours, f = 3600., fu = "3600", bu = s];
    scalar_methods![Time, qa = d, Qa = days, f = 86_400., fu = "86400", bu = s];
    scalar_methods![Time, qa = w, Qa = weeks, f = 604_800., fu = "604800", bu = s];
    scalar_methods![Time, qa = y, Qa = years, f = 31_536e3, fu = "365", bu = "days"];
    scalar_methods![
        Time,
        qa = jy,
        Qa = julian_years,
        qu = "`jy`",
        Qu = "`julian years`",
        f = Time::JULIAN_YEAR.m,
        fu = "365.25",
        bu = "days"
    ];
}

impl_scalar_methods![Time, s, seconds];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the formulas behave as expected.
    #[test]
    fn time_formulas() {
        // Distance, Speed & Time
        let time = Time::from_distance_speed(Length::new(300.), Speed::new(12.));
        assert_float_eq!(25., time.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            time.m,
            Time::from_speed_distance(Speed::new(12.), Length::new(300.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(300., time.calc_distance(Speed::new(12.)).m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(12., time.calc_speed(Length::new(300.)).m, r2nd <= Magnitude::EPSILON);

        // Energy, Power & Time
        let time = Time::from_energy_power(Energy::in_kJ(144.), Power::new(800.));
        assert_float_eq!(Time::in_min(3.).m, time.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            time.m,
            Time::from_power_energy(Power::new(800.), Energy::in_kJ(144.)).m,
            r2nd <= Magnitude::EPSILON,
        );
        assert_float_eq!(
            Energy::in_kJ(144.).m,
            time.calc_energy(Power::new(800.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            Power::new(800.).m,
            time.calc_power(Energy::in_kJ(144.)).m,
            r2nd <= Magnitude::EPSILON
        );
    }

    /// Checks the constants are defined as expected.
    #[test]
    fn time_constants() {
        assert_float_eq!(365.25, Time::JULIAN_YEAR.as_days(), r2nd <= Magnitude::EPSILON);
        assert_float_eq!(411.78443029, Time::FULL_MOON_CYCLE.as_days(), r2nd <= Magnitude::EPSILON);
        assert_float_eq!(346.620075883, Time::DRACONIC_YEAR.as_days(), r2nd <= Magnitude::EPSILON);
        assert_float_eq!(354.36707, Time::LUNAR_YEAR.as_days(), r2nd <= Magnitude::EPSILON);
    }

    /// Checks the non SI prefixes behave as expected.
    // TODO: test as_ methods
    #[test]
    fn time_non_si() {
        assert_float_eq!(60., Time::in_min(1.).m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(3600., Time::in_h(1.).m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(86_400., Time::in_d(1.).m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(604_800., Time::in_w(1.).m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(31_536e3, Time::in_y(1.).m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(Time::JULIAN_YEAR.m, Time::in_jy(1.).m, r2nd <= Magnitude::EPSILON);
    }
}
