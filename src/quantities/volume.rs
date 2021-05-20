//!
//!

use crate::{Length, F};

/// `Volume`, in `m³` (cubic [`Length`]).
#[derive(Clone, Copy, Debug)]
pub struct Volume(pub F);

// TODO: prefixes
// impl_prefixes![Volume, m3, metres, cubic];
