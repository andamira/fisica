//!

use crate::{engine::Shape, {Orientation, Position}, units::Length};

/// A physical body
pub struct Body {
    pub pos: Position,
    pub orien: Orientation,
    pub shape: Shape,
}

impl Body {
    pub fn new() -> Self {
        Self {
            pos: Position::ZERO,
            orien: Orientation::IDENTITY,
            shape: Shape::Sphere { radius: Length::new(0.0)},
        }
    }
}
