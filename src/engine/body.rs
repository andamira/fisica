//!

use crate::engine::Shape;
use crate::{Orientation, Position};

/// A physical body
pub struct Body {
    pub pos: Position,
    pub orien: Orientation,
    pub shape: Shape,
}
