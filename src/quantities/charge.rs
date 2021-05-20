//!
//!

use crate::F;

/// `Charge`, in coulombs: `C`.
#[derive(Clone, Copy, Debug)]
pub struct Charge(pub F);

impl_prefixes![Charge, C, coulombs];
