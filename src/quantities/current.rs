//!
//!

use crate::F;

/// `Current`, in amperes (amps): `A`.
///
/// Derived quantity.
///
/// # External links
/// - <https://en.wikipedia.org/wiki/Electric_current>
#[derive(Clone, Copy, Debug)]
pub struct Current(pub F);

impl_prefixes![Current, A, amperes];
