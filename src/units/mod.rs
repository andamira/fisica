//! All the physical units
//!
//! External links:
//! - <https://en.wikipedia.org/wiki/International_System_of_Units>
//! - <https://en.wikipedia.org/wiki/SI_unit>
//! - <https://en.wikipedia.org/wiki/Order_of_magnitude>

use crate::Magnitude;
use core::fmt;

// Base quantities
// - https://en.wikipedia.org/wiki/International_System_of_Quantities#Base_quantities
// - https://en.wikipedia.org/wiki/Physical_quantity#Base_quantities

// The 7 base units
// - https://en.wikipedia.org/wiki/2019_redefinition_of_the_SI_base_units

mod amount;
mod current;
mod intensity;
mod length;
mod mass;
mod temperature;
mod time;

pub use amount::Amount;
pub use current::Current;
pub use intensity::Intensity;
pub use length::{Distance, Height, Length};
pub use mass::Mass;
pub use temperature::Temperature;
pub use time::Time;

// Derived quantities
// - https://en.wikipedia.org/wiki/International_System_of_Quantities#Derived_quantities
// - https://en.wikipedia.org/wiki/Physical_quantity#General_derived_quantities

mod acceleration;
mod area;
mod charge;
mod density;
mod energy;
mod force;
mod frequency;
mod gfs;
mod moment;
mod momentum;
mod power;
mod pressure;
mod speed;
mod velocity;
mod volume;

pub use acceleration::Acceleration;
pub use area::Area;
pub use charge::Charge;
pub use density::Density;
pub use energy::{Energy, Work};
pub use force::{Force, Weight};
pub use frequency::Frequency;
pub use gfs::{Gfs, GravitationalFieldStrength};
pub use moment::{Moment, Torque};
pub use momentum::Momentum;
pub use power::Power;
pub use pressure::Pressure;
pub use speed::Speed;
pub use velocity::Velocity;
pub use volume::Volume;

/// Units for kinematics.
///
/// Kinematics is a subfield of physics, developed in [classical mechanics][0],
/// that describes the [motion][1] of points, bodies, and systems of bodies
/// without considering the forces that cause them to move.
///
/// [0]:https://en.wikipedia.org/wiki/Classical_mechanics
/// [1]:https://en.wikipedia.org/wiki/Motion_(physics)
pub mod kinematics {
    pub use crate::units::{Distance, Length, Speed, Time};
}

/// Units for dynamics.
///
/// Dynamics is the branch of physics developed in [classical mechanics][0]
/// concerned with the study of [forces][1] and their effects on [motion][2].
///
/// [0]:https://en.wikipedia.org/wiki/Classical_mechanics
/// [1]:https://en.wikipedia.org/wiki/Force_(physics)
/// [2]:https://en.wikipedia.org/wiki/Motion_(physics)
pub mod dynamics {
    pub use crate::units::{Force, Velocity};
}

/// A trait common to all units.
pub trait Unit {
    /// Returns the unit in short format.
    fn unit() -> String;

    /// Returns the unit in long format, singular.
    fn unit_long_p(&self) -> String;

    /// Returns the unit in long format, plural.
    fn unit_long_s(&self) -> String;

    /// Returns the unit in long format, singular if magnitude == 1, plural otherwise.
    fn unit_long(&self) -> String;

    /// Returns the unit and magnitude in long format.
    fn long(&self) -> String;
}

/// Auto implements:
/// - fmt::Display _(using short format)_
/// - [`Unit`]
/// - constructors: `new()` & `$method_name()` _(`$lplural` by default)_.
///
macro_rules! impl_unit {
    ($type:ty, $short:expr, $lsingular:expr, $lplural:expr, $method_name:ident) => {
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {}", self.m(), Self::unit())
            }
        }

        impl Unit for $type {
            fn unit() -> String {
                $short.into()
            }

            fn unit_long_s(&self) -> String {
                $lsingular.into()
            }

            fn unit_long_p(&self) -> String {
                $lplural.into()
            }

            fn unit_long(&self) -> String {
                // if self.m == 1. {
                // https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
                if (self.m() - 1.).abs() < Magnitude::EPSILON {
                    $lsingular.into()
                } else {
                    $lplural.into()
                }
            }

            fn long(&self) -> String {
                format!["{} {}", self.m(), self.unit_long()]
            }
        }
    };
}

// base units

impl_unit!(Time, "s", "second", "seconds", seconds);
impl_unit!(Length, "m", "metre", "metres", metres);
impl_unit!(Mass, "kg", "kilogram", "kilograms", kilograms);
impl_unit!(Current, "A", "ampere", "amperes", amperes);
impl_unit!(Temperature, "K", "kelvin", "kelvins", kelvins);
impl_unit!(Intensity, "cd", "candela", "candelas", candelas);
impl_unit!(Amount, "mol", "mole", "moles", moles);

// derived units

impl_unit!(Area, "m²", "square metre", "square metres", square_metres);
impl_unit!(Volume, "m³", "cubic metre", "cubic metres", cubic_metres);
impl_unit!(Force, "N", "newton", "newtons", newtons);
impl_unit!(
    Speed,
    "m/s",
    "meter per second",
    "metres per second",
    metres_per_second
);
impl_unit!(
    Acceleration,
    "m/s²",
    "metre per second squared",
    "metres per second squared",
    metres_per_second_squared
);
impl_unit!(Frequency, "Hz", "hertz", "hertzs", hertzs);
impl_unit!(
    Momentum,
    "kg m/s",
    "kilogram metre per second",
    "kilograms metres per second",
    kilograms_metres_per_second
);
impl_unit!(Pressure, "Pa", "pascal", "pascals", pascals);
impl_unit!(Energy, "J", "joule", "joules", joules);
impl_unit!(Power, "W", "watt", "watts", watts);
impl_unit!(Charge, "C", "coulomb", "coulombs", coulombs);
impl_unit!(
    GravitationalFieldStrength,
    "N/kg",
    "newton per kilogram",
    "newtons per kilogram",
    newtons_per_kilogram
);
impl_unit!(
    Moment,
    "Nm",
    "newton per metre",
    "newtons per metre",
    newtons_per_metre
);
