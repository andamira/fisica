//!
//!

#[allow(unused_imports)]
use crate::units::Length;
use crate::Magnitude;

/// `Volume`, in `m³` (cubic [`Length`]).
#[derive(Clone, Copy, Debug)]
pub struct Volume {
    pub m: Magnitude,
}

/// # Constructors
impl Volume {
    /// new Volume
    #[inline]
    pub const fn new(m: Magnitude) -> Self {
        Self { m }
    }

    #[inline]
    pub const fn without_direction(m: Magnitude) -> Self {
        Self::new(m)
    }
}

/// # Formulas
impl Volume {
    fn_both_unicode![
        Volume,
        "l",
        "[`litres`][Volume::LITRE]",
        l,
        litres,
        Self::LITRE.m
    ];
}

/// # Constants
impl Volume {
    /// 1 cubic decimetre (dm3).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Litre
    pub const LITRE: Self = Volume::new(1e-3);
}

// FIXME:bad conversion prefixes calc for squares and cubes
// FIXME:nomenclature: cubic_metres, cubic_kilometres…
// impl_prefixes![Volume, m3, metres_cubic];

// #[cfg(test)]
// mod tests {
//     use {super::*, float_eq::assert_float_eq};
//
//     /// Checks the constants are defined as expected.
//     #[test]
//     fn volume_constants() {
//         assert_float_eq!(1., Volume::LITRE.as_dm3(), r2nd <= Magnitude::EPSILON);
//     }
// }
