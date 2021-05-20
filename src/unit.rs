//!
//!

use core::fmt;

use crate::{quantities::*, F};

///
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
/// - TODO: as_f64()
macro_rules! impl_unit {
    ($type:ty, $short:expr, $lsingular:expr, $lplural:expr, $method_name:ident) => {
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {}", self.0, Self::unit())
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
                // if self.0 == 1. {
                // https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
                if (self.0 - 1.).abs() < F::EPSILON {
                    $lsingular.into()
                } else {
                    $lplural.into()
                }
            }
            fn long(&self) -> String {
                format!["{} {}", self.0, self.unit_long()]
            }
        }

        // /// Constructors
        // FIXME: remove this
        // impl $type {
        //     pub const fn new(quantity: F) -> Self {
        //         Self(quantity)
        //     }
        //
        //     pub const fn $method_name(quantity: F) -> Self {
        //         Self::new(quantity)
        //     }
        // }
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
