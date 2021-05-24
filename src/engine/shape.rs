use crate::units::Length;

/// Defines the geometry of the [Body][crate::engine::Body].
pub enum Shape {
    Sphere { radius: Length },
}
