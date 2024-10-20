use crate::obj::obj_2d::Object2d;

use super::force::Force;

/// Constant for Earth's gravity in meters per second squared.
pub const EARTH_GRAVITY: f32 = -9.807;

/// Represents the gravitational force applied to an object.
///
/// # Fields
///
/// - `force`: The gravitational force.
/// - `time`: The time over which the force is applied.
/// - `delta_time`: The time step for the simulation.
#[derive(Debug)]
pub struct Gravity {
    pub force: f32,
    pub time: f32,
    pub delta_time: f32,
}

impl Force for Gravity {
    /// Applies the gravitational force to a 2D object.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut obj = Object2d::new(/* initial parameters */);
    /// let gravity = Gravity {
    ///     force: EARTH_GRAVITY,
    ///     time: 1.0,
    ///     delta_time: 0.016,
    /// };
    /// gravity.apply_2d(&mut obj);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `obj`: The 2D object to which the force is applied.
    fn apply_2d(&self, obj: &mut Object2d) {
        obj.vec.y += obj.velocity.y * self.delta_time
            - 0.5 * self.force * (self.delta_time * self.delta_time);
        obj.velocity.y -= self.force * self.delta_time;
    }
}

/// Creates a new `Gravity` instance.
///
/// # Parameters
///
/// - `force`: The gravitational force.
/// - `time`: The time over which the force is applied.
/// - `delta_time`: The time step for the simulation.
///
/// # Returns
///
/// A new `Gravity` instance.
///
/// # Examples
///
/// ```
/// let gravity = gravity(EARTH_GRAVITY, 1.0, 0.016);
/// ```
#[inline]
pub fn gravity<F: Into<f32>, T: Into<f32>, D: Into<f32>>(
    force: F,
    time: T,
    delta_time: D,
) -> Gravity {
    Gravity {
        force: force.into(),
        time: time.into(),
        delta_time: delta_time.into(),
    }
}
