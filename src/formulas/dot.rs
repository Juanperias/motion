use crate::vec::vec_2d::Vec2d;

use super::sqrt::sqrt;

/// Calculates the dot product of two 2D vectors.
///
/// # Examples
///
/// ```
/// let u = Vec2d::new(3.0, 4.0);
/// let v = Vec2d::new(1.0, 2.0);
/// let result = dot(u, v);
/// assert_eq!(result, 11.0);
/// ```
///
/// # Parameters
///
/// - `u`: The first vector.
/// - `v`: The second vector.
///
/// # Returns
///
/// The dot product of the two vectors.
#[must_use]
pub fn dot(u: Vec2d, v: Vec2d) -> f32 {
    u.x * v.x + u.y * v.y
}

/// Calculates the squared length of a 2D vector.
///
/// # Examples
///
/// ```
/// let u = Vec2d::new(3.0, 4.0);
/// let result = length2(u);
/// assert_eq!(result, 25.0);
/// ```
///
/// # Parameters
///
/// - `u`: The vector to calculate the squared length for.
///
/// # Returns
///
/// The squared length of the vector.
#[must_use]
pub fn length2(u: Vec2d) -> f32 {
    dot(u, u)
}

/// Calculates the length (magnitude) of a 2D vector.
///
/// # Examples
///
/// ```
/// let u = Vec2d::new(3.0, 4.0);
/// let result = length(u);
/// assert_eq!(result, 5.0);
/// ```
///
/// # Parameters
///
/// - `u`: The vector to calculate the length for.
///
/// # Returns
///
/// The length of the vector.
#[must_use]
pub fn length(u: Vec2d) -> f32 {
    sqrt(length2(u))
}
