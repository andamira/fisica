//!
//!

use crate::{Length, Time, F};

/// How fast something is moving, in `m/s` ([`Length`] per [`Time`]).
#[derive(Clone, Copy, Debug)]
pub struct Speed(pub F);

/// (== [`Speed`]). How fast something is moving in a particular direction, in `m/s`.
pub type Velocity = Speed;

// TODO: prefixes
// impl_prefixes![Speed, km_s, kilometers_second];
