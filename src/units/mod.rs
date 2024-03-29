// fisica::units
//
//! All the physical units
//!
//! External links:
//! - <https://en.wikipedia.org/wiki/International_System_of_Units>
//! - <https://en.wikipedia.org/wiki/SI_unit>
//! - <https://en.wikipedia.org/wiki/Order_of_magnitude>
//

use crate::Magnitude;
use core::fmt;

pub use all::*;
pub(crate) mod all {
    // base quantities
    #[doc(inline)]
    pub use super::{
        amount::Amount,
        current::Current,
        intensity::Intensity,
        length::{Distance, Height, Length},
        mass::Mass,
        temperature::Temperature,
        time::Time,
    };

    // derived quantities
    #[doc(inline)]
    pub use super::{
        acceleration::Acceleration,
        area::Area,
        charge::Charge,
        density::Density,
        energy::{Energy, Work},
        force::{Force, Weight},
        frequency::Frequency,
        gfs::{Gfs, GravitationalFieldStrength},
        moment::{Moment, Torque},
        momentum::Momentum,
        power::Power,
        pressure::Pressure,
        speed::Speed,
        velocity::Velocity,
        volume::Volume,
    };
}

// Base quantities
//
// - https://en.wikipedia.org/wiki/International_System_of_Quantities#Base_quantities
// - https://en.wikipedia.org/wiki/Physical_quantity#Base_quantities
//
// The 7 base units
// - https://en.wikipedia.org/wiki/2019_redefinition_of_the_SI_base_units

mod amount;
mod current;
mod intensity;
mod length;
mod mass;
mod temperature;
mod time;

// Derived quantities
//
// - https://en.wikipedia.org/wiki/International_System_of_Quantities#Derived_quantities
// - https://en.wikipedia.org/wiki/Physical_quantity#General_derived_quantities

// scalar:
mod area; // Length²
mod charge;
mod density;
mod energy;
mod frequency;
mod power;
mod pressure;
mod speed; // Length / Time
mod volume; // Length³

// vector:
mod acceleration; // Length / Time²
mod force;
mod gfs; // Force × Mass
mod moment; // Force × Length
mod momentum; // Mass × Length / Time
mod velocity; // Length / Time

// thematic sub-modules

/// Units for kinematics.
///
/// Kinematics is a subfield of physics, developed in [*classical mechanics*][0],
/// that describes the [*motion*][1] of points, bodies, and systems of bodies
/// without considering the forces that cause them to move.
///
/// [0]:https://en.wikipedia.org/wiki/Classical_mechanics
/// [1]:https://en.wikipedia.org/wiki/Motion_(physics)
pub mod kinematics {
    #[doc(inline)]
    pub use crate::units::{Distance, Length, Speed, Time};
}

/// Units for dynamics.
///
/// Dynamics is the branch of physics developed in [*classical mechanics*][0]
/// concerned with the study of [*forces*][1] and their effects on [motion][2].
///
/// [0]:https://en.wikipedia.org/wiki/Classical_mechanics
/// [1]:https://en.wikipedia.org/wiki/Force_(physics)
/// [2]:https://en.wikipedia.org/wiki/Motion_(physics)
pub mod dynamics {
    #[doc(inline)]
    pub use crate::units::{Force, Velocity};
}

/// A trait common to all units.
pub trait Unit {
    /// Returns the unit in short format.
    fn unit() -> String;

    /// Returns the unit in long format, singular.
    fn unit_long_s(&self) -> String;

    /// Returns the unit in long format, plural.
    fn unit_long_p(&self) -> String;

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

// scalar

impl_unit!(Area, "m²", "square metre", "square metres", square_metres);
impl_unit!(Charge, "C", "coulomb", "coulombs", coulombs);
impl_unit!(Energy, "J", "joule", "joules", joules);
impl_unit!(Frequency, "Hz", "hertz", "hertzs", hertzs);
impl_unit!(Power, "W", "watt", "watts", watts);
impl_unit!(Pressure, "Pa", "pascal", "pascals", pascals);
impl_unit!(
    Speed,
    "m/s",
    "metre per second",
    "metres per second",
    metres_per_second
);
impl_unit!(Volume, "m³", "cubic metre", "cubic metres", cubic_metres);

// vector

impl_unit!(
    Acceleration,
    "m/s²",
    "metre per second squared",
    "metres per second squared",
    metres_per_second_squared
);
impl_unit!(Force, "N", "newton", "newtons", newtons);
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
impl_unit!(
    Momentum,
    "kg m/s",
    "kilogram metre per second",
    "kilograms metre per second",
    kilograms_metres_per_second
);
impl_unit!(
    Velocity,
    "m/s",
    "metre per second",
    "metres per second",
    metres_per_second
);
