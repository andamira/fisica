//!
//!

use crate::F;

///
#[derive(Clone, Copy, Debug)]
pub struct GravitationalFieldStrength(pub F);
pub type Gfs = GravitationalFieldStrength;

/// in `N/kg`.
impl GravitationalFieldStrength {
    pub const fn in_mercury() -> Self {
        Self(3.8)
    }
    pub const fn in_venus() -> Self {
        Self(8.8)
    }
    pub const fn in_earth() -> Self {
        Self(9.8)
    }
    pub const fn in_mars() -> Self {
        Self(3.8)
    }
    pub const fn in_jupiter() -> Self {
        Self(25.)
    }
    pub const fn in_saturn() -> Self {
        Self(10.4)
    }
    pub const fn in_uranus() -> Self {
        Self(10.4)
    }
    pub const fn in_neptune() -> Self {
        Self(13.8)
    }
    pub const fn in_moon() -> Self {
        Self(1.6)
    }
    pub const fn in_pluto() -> Self {
        Self(0.49)
    }
    pub const fn in_ceres() -> Self {
        Self(0.27)
    }
    pub const fn in_sun() -> Self {
        Self(293.)
    }
}

// TODO: prefixes
// impl_prefixes![GravitationalFieldStrength, , ];
