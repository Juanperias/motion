use crate::formulas::dot::length;
use core::ops::{Add, Div, Mul, Sub};

/// `Vec2d` is a simple 2D vector struct used for various vector operations.
///
/// # Examples
///
/// ```rust
///
/// let v1 = Vec2d::new(3.0, 4.0);
/// let v2 = Vec2d::new(6.0, 8.0);
///
/// assert_eq!(v1.magnitude(), 5.0);
/// assert_eq!(v1.distance(v2), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    /// The x component of the vector
    pub x: f32,
    /// The y component of the vector
    pub y: f32,
}

impl Vec2d {
    /// Creates a new `Vec2d` with the given x and y components.
    ///
    /// # Examples
    ///
    /// ```
    /// let vec = Vec2d::new(1.0, 2.0);
    /// assert_eq!(vec.x, 1.0);
    /// assert_eq!(vec.y, 2.0);
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Vec2d { x, y }
    }

    /// Computes the component vector from this vector to the target vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = Vec2d::new(1.0, 2.0);
    /// let v2 = Vec2d::new(4.0, 6.0);
    /// let component = v1.component(v2);
    /// assert_eq!(component.x, 3.0);
    /// assert_eq!(component.y, 4.0);
    /// ```
    #[must_use]
    pub fn component(&self, target: Vec2d) -> Vec2d {
        Vec2d {
            x: (target.x - self.x),
            y: (target.y - self.y),
        }
    }

    /// Computes the magnitude (length) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = Vec2d::new(3.0, 4.0);
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        length(*self)
    }

    /// Computes the distance between this vector and the target vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = Vec2d::new(1.0, 2.0);
    /// let v2 = Vec2d::new(4.0, 6.0);
    /// assert_eq!(v1.distance(v2), 5.0);
    /// ```
    #[must_use]
    pub fn distance(&self, target: Vec2d) -> f32 {
        let ab = self.component(target);
        length(ab)
    }
}

/// Implements the addition of two 2D vectors.
///
/// This implementation allows using the `+` operator to add two `Vec2d` vectors component-wise.
/// For example, `Vec2d { x: 1.0, y: 2.0 } + Vec2d { x: 3.0, y: 4.0 }` results in `Vec2d { x: 4.0, y: 6.0 }`.
impl Add for Vec2d {
    type Output = Vec2d;

    /// Adds two `Vec2d` vectors.
    ///
    /// # Parameters
    /// - `self`: The left-hand side vector in the addition.
    /// - `rhs`: The right-hand side vector in the addition.
    ///
    /// # Returns
    /// A new `Vec2d` vector where each component is the sum of the corresponding components of the input vectors.
    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implements the subtraction of two 2D vectors.
///
/// This implementation allows using the `-` operator to subtract one `Vec2d` vector from another component-wise.
/// For example, `Vec2d { x: 5.0, y: 7.0 } - Vec2d { x: 3.0, y: 2.0 }` results in `Vec2d { x: 2.0, y: 5.0 }`.
impl Sub for Vec2d {
    type Output = Vec2d;

    /// Subtracts one `Vec2d` vector from another.
    ///
    /// # Parameters
    /// - `self`: The left-hand side vector in the subtraction.
    /// - `rhs`: The right-hand side vector in the subtraction.
    ///
    /// # Returns
    /// A new `Vec2d` vector where each component is the difference of the corresponding components of the input vectors.
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Implements the dot product of two 2D vectors.
///
/// This implementation allows using the `*` operator to compute the dot product of two `Vec2d` vectors.
/// The result is a scalar value representing the dot product.
/// For example, `Vec2d { x: 1.0, y: 2.0 } * Vec2d { x: 3.0, y: 4.0 }` results in `11.0`.
impl Mul for Vec2d {
    type Output = f32;

    /// Computes the dot product of two `Vec2d` vectors.
    ///
    /// # Parameters
    /// - `self`: The left-hand side vector in the multiplication.
    /// - `rhs`: The right-hand side vector in the multiplication.
    ///
    /// # Returns
    /// A `f32` scalar value representing the dot product of the two input vectors.
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

/// Implements scalar division for 2D vectors.
///
/// This implementation allows using the `/` operator to divide each component of a `Vec2d` vector by a scalar value of type `f32`.
/// For example, `Vec2d { x: 10.0, y: 20.0 } / 2.0` results in `Vec2d { x: 5.0, y: 10.0 }`.
impl Div<f32> for Vec2d {
    type Output = Vec2d;

    /// Divides each component of a `Vec2d` vector by a scalar value.
    ///
    /// # Parameters
    /// - `self`: The vector to be divided.
    /// - `rhs`: The scalar value to divide each component by.
    ///
    /// # Returns
    /// A new `Vec2d` vector where each component is the result of the division of the corresponding component of the input vector by the scalar.
    fn div(self, rhs: f32) -> Self::Output {
        Vec2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// A convenience function to create a new `Vec2d`.
///
/// # Examples
///
/// ```
/// let vec = vec2(1.0, 2.0);
/// assert_eq!(vec.x, 1.0);
/// assert_eq!(vec.y, 2.0);
/// ```
#[inline]
pub fn vec2<A: Into<f32>, B: Into<f32>>(a: A, b: B) -> Vec2d {
    Vec2d::new(a.into(), b.into())
}
