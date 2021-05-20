//!
//!

use crate::F;

/// Absolute temperature, in `K` (kelvin).
#[derive(Clone, Copy, Debug)]
pub struct Temperature(pub F);

impl_prefixes![Temperature, K, kelvins];
