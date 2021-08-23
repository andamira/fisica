//!
//!

use crate::Magnitude;

/// `Charge`, in coulombs: `C`.
#[derive(Clone, Copy, Debug)]
pub struct Charge {
    pub m: Magnitude,
}

impl Charge {
    /// new Charge
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

/// `Charge` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(charge)>
impl Charge {
    /// (10e-19) The [elementary charge][1] of the [proton][0].
    ///
    /// [0]: https://en.wikipedia.org/wiki/Proton
    /// [1]: https://en.wikipedia.org/wiki/Elementary_charge
    pub const PROTON: Self = Self::new(1.602176634e-19);

    /// (10e-19) The [elementary charge][1] of the [electron][0].
    ///
    /// [0]: https://en.wikipedia.org/wiki/Electron
    /// [1]: https://en.wikipedia.org/wiki/Elementary_charge
    pub const ELECTRON: Self = Self::new(-1.602176634e-19);

    /// (10e-6) [Static electricity][0] from rubbing materials together (1 µC).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Static_electricity
    pub const STATIC_RUBBING: Self = Self::new(1e-6);

    /// (10e1) Charge in a typical [thundercloud][0] (15-350 C).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Cumulonimbus_cloud
    pub const THUNDERCLOUD: Self = Self::new(2.6e1);

    /// (10e3) Typical alkaline [AA battery][0] is about 5000 C ≈ 1.4 Ah.
    ///
    /// [0]: https://en.wikipedia.org/wiki/AA_battery
    pub const AA_BATTERY: Self = Self::new(5e3);

    /// (10e4) The [Faraday constant] represents the charge per mole of electrons.
    ///
    /// [0]: https://en.wikipedia.org/wiki/Faraday_constant
    pub const FARADAY: Self = Self::new(9.648_533_212_33e4);
}

impl_scalar_methods![Charge, C, coulombs];
