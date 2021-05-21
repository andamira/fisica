//!
//!

use crate::F;

/// `Charge`, in coulombs: `C`.
#[derive(Clone, Copy, Debug)]
pub struct Charge(pub F);

/// `Charge` constants by order of magnitude
///
/// <https://en.wikipedia.org/wiki/Orders_of_magnitude_(charge)>
impl Charge {}

impl_prefixes![Charge, C, coulombs];
