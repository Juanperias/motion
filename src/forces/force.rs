use crate::obj::obj_2d::Object2d;

/// A trait for applying forces to 2D objects.
///
/// # Examples
///
/// Implementing the `Force` trait for a custom force:
///
/// ```
/// struct Gravity;
///
/// impl Force for Gravity {
///     fn apply_2d(&self, obj: &mut Object2d) {
///         // Apply gravitational force to obj
///     }
/// }
///
/// ```
///
/// # Methods
///
/// - `apply_2d(&self, obj: &mut Object2d)`: Applies a force to a 2D object.
pub trait Force {
    fn apply_2d(&self, obj: &mut Object2d);
}
