use super::force::Force;

/// Represents the direction in a 2D plane.
#[derive(Debug, PartialEq, Eq)]
pub enum Direction2d {
    /// Movement along the x-axis.
    X,
    /// Movement along the y-axis.
    Y,
}

/// Represents a 2D movement with specified direction, delta time, and frames per second.
///
/// # Fields
///
/// - `direction`: The direction of the movement (X or Y).
/// - `delta_time`: The time step for the simulation.
/// - `fps`: The frames per second for the simulation.
pub struct Move2d {
    pub direction: Direction2d,
    pub delta_time: f32,
    pub fps: f32,
}

impl Force for Move2d {
    /// Applies the movement force to a 2D object based on the direction.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut obj = Object2d::new(/* initial parameters */);
    /// let move_force = Move2d {
    ///     direction: Direction2d::X,
    ///     delta_time: 0.016,
    ///     fps: 60.0,
    /// };
    /// move_force.apply_2d(&mut obj);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `obj`: The 2D object to which the force is applied.
    fn apply_2d(&self, obj: &mut crate::obj::obj_2d::Object2d) {
        let time = self.fps * self.delta_time;
        match self.direction {
            Direction2d::Y => {
                obj.vec.y =
                    obj.vec.y + obj.velocity.y * time + 0.5 * obj.acceleration.y * (time * time);
            }
            Direction2d::X => {
                obj.vec.x =
                    obj.vec.x + obj.velocity.x * time + 0.5 * obj.acceleration.x * (time * time);
            }
        }
    }
}

/// Creates a new `Move2d` instance.
///
/// # Parameters
///
/// - `direction`: The direction of the movement (X or Y).
/// - `delta_time`: The time step for the simulation.
/// - `fps`: The frames per second for the simulation.
///
/// # Returns
///
/// A new `Move2d` instance.
///
/// # Examples
///
/// ```
/// let move_force = move2d(Direction2d::X, 0.016, 60.0);
/// ```
#[inline]
pub fn move2d<DT: Into<f32>, F: Into<f32>>(
    direction: Direction2d,
    delta_time: DT,
    fps: F,
) -> Move2d {
    Move2d {
        direction,
        delta_time: delta_time.into(),
        fps: fps.into(),
    }
}
