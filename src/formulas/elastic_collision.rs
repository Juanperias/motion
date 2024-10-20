use crate::vec::vec_2d::Vec2d;

/// Calculates the new velocity of an object after an elastic collision.
///
/// # Examples
///
/// ```
/// let v1 = Vec2d::new(3.0, 4.0);
/// let v2 = Vec2d::new(1.0, 2.0);
/// let m1 = 2.0;
/// let m2 = 3.0;
///
/// let new_velocity = calculate(v1, m1, m2, v2);
/// ```
///
/// # Parameters
///
/// - `v1`: The velocity of the first object before collision.
/// - `m1`: The mass of the first object.
/// - `m2`: The mass of the second object.
/// - `v2`: The velocity of the second object before collision.
///
/// # Returns
///
/// The new velocity of the first object after an elastic collision.
#[must_use]
pub fn calculate(v1: Vec2d, m1: f32, m2: f32, v2: Vec2d) -> Vec2d {
    let numerator_x = v1.x * (m1 - m2) + 2.0 * v2.x * m2;
    let numerator_y = v1.y * (m1 - m2) + 2.0 * v2.y * m2;
    let denominator = m1 + m2;
    Vec2d::new(numerator_x / denominator, numerator_y / denominator)
}
