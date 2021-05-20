//!
//!

use crate::{Length, F};

/// `Area`, in `m²` (squared [`Length`]).
///
/// The quantity that expresses the extent of a two-dimensional region, shape,
/// or planar lamina, in the plane.
///
/// External links:
/// - <https://en.wikipedia.org/wiki/Area>
/// - <https://en.wikipedia.org/wiki/Orders_of_magnitude_(area)>
#[derive(Clone, Copy, Debug)]
pub struct Area(pub F);

// TODO: prefixes
// impl_prefixes![Area, m2, metres, square];
