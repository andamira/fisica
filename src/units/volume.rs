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
    scalar_methods![
        Volume,
        qa = l,
        Qa = litres,
        qu = "l",
        Qu = "[`litres`][Volume::LITRE]",
        f = Self::LITRE.m,
        fu = 1,
        bu = "dm³"
    ];
}

/// # Constants
impl Volume {
    /// 1 cubic decimetre (dm3).
    ///
    /// [0]:https://en.wikipedia.org/wiki/Litre
    pub const LITRE: Self = Volume::new(1e-3);
}

// FIXME:nomenclature: cubic_metres, cubic_kilometres…
impl_scalar_methods_cubed![Volume, m3, metres_cubed, qu = "m³", Qu = "metres cubed"];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the constants are defined as expected.
    #[test]
    fn volume_constants() {
        assert_float_eq!(1., Volume::LITRE.as_dm3(), r2nd <= Magnitude::EPSILON);
    }
}
