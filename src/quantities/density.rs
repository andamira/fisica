//!
//!

use crate::{Mass, Volume, F};

/// [`Mass`] per unit [`Volume`], in `kg/m³`.
///
/// ρ = m / V
#[derive(Clone, Copy, Debug)]
pub struct Density(pub F);
