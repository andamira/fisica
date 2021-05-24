//!
//!

use crate::{Direction, Magnitude};

/// in `N/kg`.
#[derive(Clone, Copy, Debug)]
pub struct GravitationalFieldStrength {
    pub m: Magnitude,
    pub d: Direction,
}

/// (== [`GravitationalFieldStrength`])
pub type Gfs = GravitationalFieldStrength;

/// # Constructors
impl GravitationalFieldStrength {
    /// new GravitationalFieldStrength
    #[inline]
    pub const fn new(m: Magnitude, d: Direction) -> Self {
        Self { m, d }
    }

    /// new GravitationalFieldStrength with undefined direction
    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m, Direction::ZERO)
    }
}

/// # `Length` constants by order of magnitude
impl GravitationalFieldStrength {
    pub const fn in_mercury() -> Self {
        Self::without_direction(3.8)
    }
    pub const fn in_venus() -> Self {
        Self::without_direction(8.8)
    }
    pub const fn in_earth() -> Self {
        Self::without_direction(9.8)
    }
    pub const fn in_mars() -> Self {
        Self::without_direction(3.8)
    }
    pub const fn in_jupiter() -> Self {
        Self::without_direction(25.)
    }
    pub const fn in_saturn() -> Self {
        Self::without_direction(10.4)
    }
    pub const fn in_uranus() -> Self {
        Self::without_direction(10.4)
    }
    pub const fn in_neptune() -> Self {
        Self::without_direction(13.8)
    }
    pub const fn in_moon() -> Self {
        Self::without_direction(1.6)
    }
    pub const fn in_pluto() -> Self {
        Self::without_direction(0.49)
    }
    pub const fn in_ceres() -> Self {
        Self::without_direction(0.27)
    }
    pub const fn in_sun() -> Self {
        Self::without_direction(293.)
    }
}

// TODO: prefixes
// impl_prefixes![GravitationalFieldStrength, , ];
