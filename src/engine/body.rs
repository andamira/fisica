//!

use crate::{
    engine::Shape,
    {Orientation, Position},
};

/// A physical body
#[derive(Debug, Default)]
pub struct Body {
    pub pos: Position,
    pub orien: Orientation,
    pub shape: Shape,
}
