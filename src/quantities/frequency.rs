//!
//!

use crate::F;

/// `Frequency`, in hertzs: `Hz` (1 per second).
#[derive(Clone, Copy, Debug)]
pub struct Frequency(pub F);

impl_prefixes![Frequency, Hz, hertzs];
