//!

use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::{Magnitude, V3};

/// `Direction` is a vector quantity that represents a change in [`Position`].
///
/// `Position` and `Direction` are two sides of the same coin.
/// We can also think of any position as a change of position from the
/// origin (`Position::ZERO`) to the target location.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Direction {
    v: V3,
}

/// A vector quantity representing a unique location in space.
/// Semantic alias of [`Direction`].
pub type Position = Direction;

/// # Methods
impl Direction {
    /// New `Direction`.
    pub fn new(x: Magnitude, y: Magnitude, z: Magnitude) -> Self {
        Self {
            v: V3::new(x, y, z),
        }
    }

    pub fn to_array(&self) -> [Magnitude; 3] {
        self.v.to_array()
    }

    pub fn x(&self) -> Magnitude {
        self.v.to_array()[0]
    }

    pub fn y(&self) -> Magnitude {
        self.v.to_array()[1]
    }

    pub fn z(&self) -> Magnitude {
        self.v.to_array()[2]
    }

    /// Returns the `Magnitude` of the current `Direction`.
    ///
    /// $$
    /// d = |\bm{a}| = \sqrt{x^2 + y^2 + z^2}
    /// $$
    #[inline]
    pub fn magnitude(&self) -> Magnitude {
        self.v.length()
    }

    /// Returns the square of the `Magnitude` of the current `Direction`.
    ///
    /// This is faster than getting the `Magnitude` since we avoid having to
    /// calculate the square root of the sum of the vector components.
    /// This can be useful if we just need to compare magnitudes.
    ///
    /// $$
    /// d^2 = |\bm{a}|^2 = x^2 + y^2 + z^2
    /// $$
    pub fn magnitude_squared(&self) -> Magnitude {
        self.v.length_squared()
    }

    /// Returns the normalized vector, as a *unit vector*.
    ///
    /// That is the `Direction` with a [`Magnitude`] of 1.
    ///
    /// $$
    /// \bm{n} = \widehat{\bm{a}} = \frac{1}{d}\thinspace\bm{a} =
    /// \frac{\bm{a}}{|\bm{a}|}
    /// $$
    pub fn normalize(&self) -> Direction {
        Self {
            v: self.v.normalize(),
        }
    }

    /// Returns the cross product.
    ///
    /// Also known as the *vector product*. It allows to point in a `Direction`
    /// orthogonal to both vectors.
    ///
    /// $$
    /// \bm{a}\times\bm{b} =
    /// \begin{bmatrix} a_x \cr a_y \cr a_z \end{bmatrix} \times
    /// \begin{bmatrix} b_x \cr b_y \cr b_z \end{bmatrix} =
    /// \begin{bmatrix}
    ///     a_y b_z - a_z b_y \cr
    ///     a_z b_x - a_x b_z \cr
    ///     a_x b_y - a_y b_x
    /// \end{bmatrix}
    /// $$
    pub fn cross(&self, other: Self) -> Self {
        Self {
            v: self.v.cross(other.v),
        }
    }

    /// Returns the dot product.
    ///
    /// Also known as the *scalar product*. It allows us to calculate the
    /// `Magnitude` of one vector in the direction of another.
    ///
    /// $$
    /// \bm{a}\cdot\bm{b} =
    /// \begin{bmatrix} a_x \cr a_y \cr a_z \end{bmatrix} \cdot
    /// \begin{bmatrix} b_x \cr b_y \cr b_z \end{bmatrix} =
    /// a_x b_x + a_y b_y + a_z b_z
    /// $$
    pub fn dot(&self, other: Self) -> Magnitude {
        self.v.dot(other.v)
    }

    /// *Returns the underlying vector type.*
    pub fn vector(&self) -> V3 {
        self.v
    }
}

/// # Constants
impl Direction {
    /// All zeros.
    pub const ZERO: Self = Self { v: V3::ZERO };

    /// All ones.
    pub const ONE: Self = Self { v: V3::ONE };
}

// Vector addition
impl Add for Direction {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            v: self.v + other.v,
        }
    }
}
impl AddAssign for Direction {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            v: self.v + other.v,
        };
    }
}

// Vector substraction
impl Sub for Direction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            v: self.v - other.v,
        }
    }
}
impl SubAssign for Direction {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            v: self.v - other.v,
        };
    }
}

// Scalar multiplication
impl Mul<Magnitude> for Direction {
    type Output = Self;

    fn mul(self, other: Magnitude) -> Self {
        Self { v: self.v * other }
    }
}
impl MulAssign<Magnitude> for Direction {
    fn mul_assign(&mut self, other: Magnitude) {
        *self = Self { v: self.v * other };
    }
}

// Scalar division
impl Div<Magnitude> for Direction {
    type Output = Self;

    fn div(self, other: Magnitude) -> Self {
        Self { v: self.v / other }
    }
}
impl DivAssign<Magnitude> for Direction {
    fn div_assign(&mut self, other: Magnitude) {
        *self = Self { v: self.v / other };
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
