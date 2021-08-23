//!
//!

use crate::{Direction, Magnitude};

/// in `N/kg`.
#[derive(Clone, Copy, Debug)]
pub struct GravitationalFieldStrength {
    pub d: Direction,
}

/// (== [`GravitationalFieldStrength`])
pub type Gfs = GravitationalFieldStrength;

impl GravitationalFieldStrength {
    /// New GravitationalFieldStrength
    #[inline]
    pub const fn new(d: Direction) -> Self {
        Self { d }
    }

    /// Returns the magnitude.
    #[inline]
    pub fn m(&self) -> Magnitude {
        self.d.magnitude()
    }
}

/// # `Length` constants by order of magnitude
impl GravitationalFieldStrength {
    pub fn in_mercury() -> Self {
        Self::new(Direction::new(0., 3.8, 0.))
    }
    pub fn in_venus() -> Self {
        Self::new(Direction::new(0., 8.8, 0.))
    }
    pub fn in_earth() -> Self {
        Self::new(Direction::new(0., 9.8, 0.))
    }
    pub fn in_mars() -> Self {
        Self::new(Direction::new(0., 3.8, 0.))
    }
    pub fn in_jupiter() -> Self {
        Self::new(Direction::new(0., 25., 0.))
    }
    pub fn in_saturn() -> Self {
        Self::new(Direction::new(0., 10.4, 0.))
    }
    pub fn in_uranus() -> Self {
        Self::new(Direction::new(0., 10.4, 0.))
    }
    pub fn in_neptune() -> Self {
        Self::new(Direction::new(0., 13.8, 0.))
    }
    pub fn in_moon() -> Self {
        Self::new(Direction::new(0., 1.6, 0.))
    }
    pub fn in_pluto() -> Self {
        Self::new(Direction::new(0., 0.49, 0.))
    }
    pub fn in_ceres() -> Self {
        Self::new(Direction::new(0., 0.27, 0.))
    }
    pub fn in_sun() -> Self {
        Self::new(Direction::new(0., 293., 0.))
    }
}

// TODO: impl_vector_methods_two_units![GravitationalFieldStrength, , ];
