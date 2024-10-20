use crate::{collision::shape::Shape, forces::force::Force, vec::vec_2d::Vec2d};

/// Represents a 2D object with physical properties and shape.
#[derive(Debug, Clone, Copy)]
pub struct Object2d {
    /// The position vector of the object.
    pub vec: Vec2d,
    /// The density of the object.
    pub density: f32,
    /// The mass of the object.
    pub mass: f32,
    /// The velocity vector of the object.
    pub velocity: Vec2d,
    /// The acceleration vector of the object.
    pub acceleration: Vec2d,
    /// The radius of the object, if applicable.
    pub radius: f32,
    /// The shape of the object.
    pub shape: Shape,
}

impl Object2d {
    /// Creates a new `Object2d` with the specified properties.
    ///
    /// # Parameters
    ///
    /// - `vec`: The position vector of the object.
    /// - `density`: The density of the object.
    /// - `mass`: The mass of the object.
    /// - `velocity`: The velocity vector of the object.
    /// - `acceleration`: The acceleration vector of the object.
    /// - `radius`: The radius of the object.
    /// - `shape`: The shape of the object.
    ///
    /// # Returns
    ///
    /// A new `Object2d` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let obj = Object2d::new(vec2(1.0, 2.0), 1.0, 1.0, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.0, Shape::Circle);
    /// ```
    #[must_use]
    pub fn new(
        vec: Vec2d,
        density: f32,
        mass: f32,
        velocity: Vec2d,
        acceleration: Vec2d,
        radius: f32,
        shape: Shape,
    ) -> Self {
        Object2d {
            vec,
            density,
            mass,
            velocity,
            acceleration,
            radius,
            shape,
        }
    }

    /// Applies a force to the object.
    ///
    /// # Parameters
    ///
    /// - `force`: The force to apply.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut obj = Object2d::new(vec2(1.0, 2.0), 1.0, 1.0, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.0, Shape::Circle);
    /// let gravity = Gravity::new(EARTH_GRAVITY, 1.0, 0.016);
    /// obj.apply(gravity);
    /// ```
    pub fn apply<T: Force>(&mut self, force: &T) {
        force.apply_2d(self);
    }
}

/// Creates a new `Object2d` instance with the specified properties using a more convenient syntax.
///
/// # Parameters
///
/// - `vec`: The position vector of the object.
/// - `density`: The density of the object.
/// - `mass`: The mass of the object.
/// - `velocity`: The velocity vector of the object.
/// - `acceleration`: The acceleration vector of the object.
/// - `radius`: The radius of the object.
/// - `shape`: The shape of the object.
///
/// # Returns
///
/// A new `Object2d` instance.
///
/// # Examples
///
/// ```
/// let obj = obj2d(vec2(1.0, 2.0), 1.0, 1.0, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.0, Shape::Circle);
/// ```
#[inline]
pub fn obj2d<V: Into<Vec2d>>(
    vec: V,
    density: f32,
    mass: f32,
    velocity: V,
    acceleration: V,
    radius: f32,
    shape: Shape,
) -> Object2d {
    Object2d::new(
        vec.into(),
        density,
        mass,
        velocity.into(),
        acceleration.into(),
        radius,
        shape,
    )
}
