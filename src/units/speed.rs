//!
//!

use crate::units::{Distance, Length, Time};
use crate::Magnitude;

/// How fast something is moving, in `m/s` ([`Distance`] per [`Time`]).
#[derive(Clone, Copy, Debug)]
pub struct Speed {
    pub m: Magnitude,
}

impl Speed {
    /// New speed.
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

/// # `Speed` formulas
impl Speed {
    /// Derives the `Speed` from the given [`Distance`] and [`Time`] (`s = d / t`).
    #[inline]
    pub fn from_distance_time(d: Distance, t: Time) -> Self {
        Self::new(d.m / t.m)
    }

    /// (Alias of [from_distance_time][Speed::from_distance_time]).
    #[inline]
    pub fn from_time_distance(t: Time, d: Distance) -> Self {
        Self::from_distance_time(d, t)
    }

    /// Calculates the [`Distance`] given the [`Time`] (`d = s × t`).
    #[inline]
    pub fn calc_distance(&self, t: Time) -> Distance {
        Length::new(self.m * t.m)
    }

    /// Calculates the [`Time`] given the [`Distance`] (`t = d / s`).
    pub fn calc_time(&self, d: Distance) -> Time {
        Time::new(d.m / self.m)
    }
}

/// # `Speed` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(speed)>
impl Speed {
    /// (10e-12) Average growth rate of a limestone [stalactite][0].
    ///
    /// [0]:https://en.wikipedia.org/wiki/Stalactite
    pub const STALACTITE: Self = Speed::new(4.12e-12);

    /// (10e-9) Average [human hair growth][0] speed.
    /// (Note that there is a great range of variation)
    ///
    /// [0]:https://en.wikipedia.org/wiki/Human_hair_growth
    pub const HUMAN_HAIR_GROWTH: Self = Speed::new(4.8e-9);

    /// (10e-1) 1 Km/h ([kilometre per hour][0]).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Kilometres_per_hour
    pub const KM_H: Self = Speed::new(0.2778);

    /// (10e-1) 1 mph ([mile per hour][0]).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Miles_per_hour
    pub const MPH: Self = Speed::new(0.44704);

    /// (10e-1) 1 knot ([nautical mile][0] per hour).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Nautical_mile
    pub const KNOT: Self = Speed::new(0.5144);

    /// (10e0) Average [running][0] speed
    /// (`4.98 m/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Running
    pub const RUNNING: Self = Speed::new(4.98);

    /// (10e1) Maximum speed a human can attain during a face-down [free-fall][0]
    /// (`54 m/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Free_fall
    pub const HUMAN_FREE_FALL_MAX: Self = Speed::new(54.);

    /// (10e2) [Speed of sound][0] in [standard atmosphere][1] (15 °C and 1 atm)
    /// (`340.3 m/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Speed_of_sound
    /// [1]:https://en.wikipedia.org/wiki/International_Standard_Atmosphere
    pub const SOUND: Self = Speed::new(340.3);

    /// (10e3) [Speed of sound][0] in [water][1] or in [soft tissue][2]
    /// (`1.5 lm/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Speed_of_sound
    /// [1]:https://en.wikipedia.org/wiki/Water
    /// [2]:https://en.wikipedia.org/wiki/Soft_tissue
    pub const SOUND_WATER: Self = Speed::new(1_500.);

    /// (10e3) [Escape velocity][0] from Moon
    /// (`2.375 km/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Escape_velocity
    pub const ESCAPE_VELOCITY_MOON: Self = Speed::new(2_375.);

    /// (10e4) [Escape velocity][0] from Earth
    /// (`11.2 km/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Escape_velocity
    pub const ESCAPE_VELOCITY_EARTH: Self = Speed::new(11_200.);

    /// (10e4) Speed of the [Earth][0] in orbit around the [Sun][1]
    /// (`29.8 km/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Earth
    /// [1]:https://en.wikipedia.org/wiki/Sun
    pub const EARTH_ORBIT: Self = Speed::new(29_800.);

    /// (10e5) Orbital speed of the [solar system][0] in the [Milky Way][1] galaxy
    /// (`200 km/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Solar_System
    /// [1]:https://en.wikipedia.org/wiki/Milky_Way
    pub const SOLAR_SYSTEM_ORBIT: Self = Speed::new(2e5);

    /// (10e8) Speed of a [signal][0] in an [optical fiber][1]
    /// (`200 Mm/s`).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Signalling_(telecommunications)
    /// [1]:https://en.wikipedia.org/wiki/Optical_fiber
    pub const FIBER_OPTIC_SIGNAL: Self = Speed::new(2e8);

    /// (10e8) The [speed of light][0].
    ///
    /// [0]:https://en.wikipedia.org/wiki/Speed_of_light
    pub const LIGHT: Self = Speed::new(299_792_458.);

    /// (10e16) The [speed of light][0], squared.
    ///
    /// [0]:https://en.wikipedia.org/wiki/Speed_of_light
    pub const LIGHT_SQUARED: Self = Speed::new(89_875_517_873_681_764.);
}

impl_scalar_methods_2units![
    Speed,
    q1a = m,
    q2a = s,
    Q1a = metres,
    Q2a = second,
    Ja = per
];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the constants are defined as expected.
    #[test]
    fn speed_constants() {
        //assert_float_eq!(0.0000000000162, Length::PLANCK.as_ym(), r2nd <= Magnitude::EPSILON);
    }

    /// Checks the formulas behave as expected.
    #[test]
    fn speed_formulas() {
        // Distance, Speed & Time
        let speed = Speed::from_distance_time(Distance::in_m(300.), Time::new(25.));
        assert_float_eq!(12., speed.m, r2nd <= Magnitude::EPSILON);
        assert_float_eq!(
            25.,
            speed.calc_time(Length::new(300.)).m,
            r2nd <= Magnitude::EPSILON
        );
        assert_float_eq!(
            300.,
            speed.calc_distance(Time::new(25.)).m,
            r2nd <= Magnitude::EPSILON
        );
    }
}
