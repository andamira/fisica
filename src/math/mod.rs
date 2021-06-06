//! # The math
//!
//! ## Vectors
//!
//! Vectors are represented as an array of scalar values.
//! They represent quantities in 2D or 3D space.
//!
//! The vector $\bm{a}$ is the agroupation of the $z$, $y$, and $z$ coordinate
//! values along the X, Y and Z axes.
//!
//! $$
//! \bm{a} = \begin{bmatrix} x \cr y \cr z \end{bmatrix}
//! $$
//!
//! A vector can represent a unique [Position] in space. And any position can be
//! interpreted as a *change of position*.
//!
//! The change in position (from $\bm{a}_0$ to $\bm{a}_1$ where
//! $\Delta x = x_1 - x_0$ and similarly for $\Delta y$ and $\Delta z$)
//! would be represented as:
//!
//! $$
//! \bm{a} = \begin{bmatrix} \Delta x \cr \Delta y \cr \Delta z \end{bmatrix}
//! $$
//!
//! A change of position can be split into two elements: $\bm{a} = d\bm{n}$.
//! Where $d$ is the directionless [Magnitude] of the change (a scalar),
//! and $\bm{n}$ is the *unit vector* that represents the [Direction],
//! with a magnitude of 1.
//!
//!
//! To find $d$ (where $|\bm{a}|$ is the magnitude of the vector)
//! we can use the Pythagorean theorem in 3D:
//!
//! $$
//! d = |\bm{a}| = \sqrt{x^2 + y^2 + z^2}
//! $$
//!
//! And to find $\bm{n}$ (where $\widehat{\bm{a}}$ is the unit vector in the
//! direction of $\bm{a}$) we can use the formula $\bm{a} = d\bm{n}$ in the form:
//!
//! $$
//! \bm{n} = \widehat{\bm{a}} = \frac{1}{d}\thinspace\bm{a}
//! $$
//!
//! Finding $\bm{n}$ is called *normalizing*, and decomposing a vector into its
//! two components its called the *normal form* of the vector.
//!
//! ```
//! # use fisika::{Direction, Position};
//! assert_eq![
//!     Position::new(2., 3., 4.).normalize(),
//!     Direction::new(0.3713906763541037, 0.5570860145311556, 0.7427813527082074)
//! ];
//! ```
//!
//! Note: *To compare two magnitudes it is much faster to omit the square root
//! and just compare their magnitude squares: $(x^2 + y^2 + z^2)$*.
//!
//! ### Scalar and vector multiplication
//!
//! As previously shown in the normalization equations, it's possible to
//! multiply a scalar $k$ by a vector $\bm{a}$, like this:
//!
//! $$
//! k\bm{a} = k\begin{bmatrix} x \cr y \cr z \end{bmatrix} =
//! \begin{bmatrix} kx \cr ky \cr kz \end{bmatrix}
//! $$
//!
//! To divide a vector by a scalar:
//!
//! $$
//! a / b = a \times \frac{1}{b}
//! $$
//!

// f64 → Dvec3, Dquat, DMat3
// f32 → Vec3, Quat, Mat3

mod vectors;
pub use vectors::{Direction, Position};

/// The floating point type used for magnitudes
pub type Magnitude = f64;

// The vector type to use
pub(crate) type V3 = glam::DVec3;

/// Orientation
pub type Orientation = glam::DQuat;

/// Rotatation Matrix
pub type Matrix = glam::DMat3;
