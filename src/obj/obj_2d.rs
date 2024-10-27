use crate::{
    collision::shape::Shape,
    forces::force::Force,
    vec::vec_2d::{vec2, Vec2d},
};

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
#[deprecated(since = "0.1.3", note = "Use the Object2dBuilder instead")]
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

#[derive(Debug)]
pub struct Object2dBuilder {
    position: Vec2d,
    density: f32,
    mass: f32,
    velocity: Vec2d,
    acceleration: Vec2d,
    radius: f32,
    shape: Shape,
}

impl Object2dBuilder {
    #[must_use]
    pub fn new() -> Self {
        Object2dBuilder {
            position: vec2(0.0, 0.0),
            density: 0.0,
            mass: 0.0,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            radius: 0.0,
            shape: Shape::None,
        }
    }
    #[must_use]
    pub fn position(mut self, vec: Vec2d) -> Self {
        self.position = vec;
        self
    }

    #[must_use]
    pub fn density(mut self, density: f32) -> Self {
        self.density = density;
        self
    }

    #[must_use]
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    #[must_use]
    pub fn velocity(mut self, vec: Vec2d) -> Self {
        self.velocity = vec;
        self
    }

    #[must_use]
    pub fn acceleration(mut self, vec: Vec2d) -> Self {
        self.acceleration = vec;
        self
    }

    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    #[must_use]
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
    #[must_use]
    pub fn build(self) -> Object2d {
        Object2d {
            vec: self.position,
            density: self.density,
            mass: self.mass,
            velocity: self.velocity,
            acceleration: self.acceleration,
            radius: self.radius,
            shape: self.shape,
        }
    }
}

impl Default for Object2dBuilder {
    fn default() -> Self {
        Self::new()
    }
}
