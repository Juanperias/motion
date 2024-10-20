use super::sqrt::sqrt;

/// Calculates the hypotenuse of a right-angled triangle using the Pythagorean theorem.
///
/// # Examples
///
/// ```
/// let result = calculate_pt(3.0, 4.0);
/// assert_eq!(result, 5.0);
///
/// let result = calculate_pt(6.0, 8.0);
/// assert_eq!(result, 10.0);
/// ```
///
/// # Parameters
///
/// - `a`: The length of one side of the right-angled triangle.
/// - `b`: The length of the other side of the right-angled triangle.
///
/// # Returns
///
/// The length of the hypotenuse calculated using the Pythagorean theorem.
pub fn calculate_pt(a: f32, b: f32) -> f32 {
    let ab = a * a + b * b;
    sqrt(ab)
}
