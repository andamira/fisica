//!
//!

use crate::F;

/// `Pressure`, in pascals: `Pa`.
#[derive(Clone, Copy, Debug)]
pub struct Pressure(pub F);

impl_prefixes![Pressure, Pa, pascals];
