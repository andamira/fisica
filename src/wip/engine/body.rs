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

// WIP
// impl Body {
//     pub fn new() -> Self {
//         Self {
//             pos: Position::ZERO,
//             orien: Orientation::IDENTITY,
//             shape: Shape::Sphere {
//                 radius: Length::new(0.0),
//             },
//         }
//     }
// }
