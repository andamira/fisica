//!
//!

use crate::{Length, Mass, Time, F};

/// `Momentum`, in [`Mass`] times [`Length`] per [`Time`]: `kg m/s`.
#[derive(Clone, Copy, Debug)]
pub struct Momentum(pub F);

// TODO: prefixes
// impl_prefixes![Momentum, , ];
