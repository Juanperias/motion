/// Calculates the square root of a given number.
///
/// # Examples
///
/// ```
/// let result = sqrt(4.0);
/// assert_eq!(result, 2.0);
///
/// let result = sqrt(9.0);
/// assert_eq!(result, 3.0);
///
/// let result = sqrt(-1.0);
/// assert!(result.is_nan());
/// ```
///
/// # Parameters
///
/// - `number`: The number to compute the square root of. If the number is negative, the function will return `NaN`.
///
/// # Returns
///
/// The square root of the given number, or `NaN` if the number is negative.
#[must_use]
pub fn sqrt(number: f32) -> f32 {
    if number < 0.0 {
        return f32::NAN;
    }
    f32::from_bits((number.to_bits() + 0x3f80_0000) >> 1)
}
