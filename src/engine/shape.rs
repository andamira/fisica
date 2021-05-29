use crate::units::Length;

/// Defines the geometry of the [Body][crate::engine::Body].
#[derive(Debug)]
pub enum Shape {
    Sphere { radius: Length },
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Sphere {
            radius: Length::new(0.),
        }
    }
}
