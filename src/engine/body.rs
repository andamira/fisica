//!

use crate::{Position, Orientation};
use crate::engine::Shape;

/// A physical body
pub struct Body {
    pos: Position,
    ori: Orientation,
    shape: Shape,
}
