// fisica::units::volume
//
//!
//

#[allow(unused_imports)]
use crate::units::Length;
use crate::Magnitude;

/// `Volume`, in `m³` (cubic [`Length`]).
#[derive(Clone, Copy, Debug)]
pub struct Volume {
    pub m: Magnitude,
}

impl Volume {
    /// New Volume.
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

impl_scalar_methods_cubic![
    Volume,
    qa = m3,
    QaL = cubic_,
    QaM = metres,
    qu = "m³",
    QuL = "cubic",
    QuM = "metres"
];

#[cfg(test)]
mod tests {
    use {super::*, float_eq::assert_float_eq};

    /// Checks the constants are defined as expected.
    #[test]
    fn volume_constants() {
        assert_float_eq!(1., Volume::LITRE.as_dm3(), r2nd <= Magnitude::EPSILON);
    }
}
