/// Calculates the power of a base raised to an exponent.
///
/// # Examples
///
/// ```
/// let result = powf(2.0, 3);
/// assert_eq!(result, 8.0);
///
/// let result = powf(5.0, -2);
/// assert_eq!(result, 0.04);
///
/// let result = powf(2.0, 0);
/// assert_eq!(result, 1.0);
/// ```
///
/// # Parameters
///
/// - `base`: The base number to be raised.
/// - `exponent`: The exponent to which the base is raised. Can be positive, negative, or zero.
///
/// # Returns
///
/// The result of raising the base to the given exponent.
#[must_use]
pub fn powf(base: f32, exponent: i32) -> f32 {
    if exponent == 0 {
        return 1.0;
    }
    let mut result = 1.0;
    let mut exp = exponent.abs();
    let mut base_current = base;
    while exp > 0 {
        if exp & 1 != 0 {
            result *= base_current;
        }
        base_current *= base_current;
        exp >>= 1;
    }
    if exponent < 0 {
        return 1.0 / result;
    }
    result
}
