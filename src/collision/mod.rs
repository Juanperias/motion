use crate::obj::obj_2d::Object2d;
pub mod shape;
use shape::Shape;

/// Represents a 2D collision detection between two objects.
pub struct Collision2d {
    /// The first object involved in the collision.
    pub obj1: Object2d,
    /// The second object involved in the collision.
    pub obj2: Object2d,
}

impl Collision2d {
    /// Creates a new `Collision2d` instance with the specified objects.
    ///
    /// # Parameters
    ///
    /// - `obj1`: The first object involved in the collision.
    /// - `obj2`: The second object involved in the collision.
    ///
    /// # Returns
    ///
    /// A new `Collision2d` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let obj1 = Object2d::new(/* parameters */);
    /// let obj2 = Object2d::new(/* parameters */);
    /// let collision = Collision2d::new(obj1, obj2);
    /// ```
    pub fn new(obj1: Object2d, obj2: Object2d) -> Self {
        Self { obj1, obj2 }
    }

    /// Detects if a collision occurs between the two objects.
    ///
    /// # Returns
    ///
    /// `true` if a collision is detected, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let obj1 = Object2d::new(/* parameters */);
    /// let obj2 = Object2d::new(/* parameters */);
    /// let collision = Collision2d::new(obj1, obj2);
    /// if collision.collider() {
    ///     println!("Collision detected!");
    /// }
    /// ```
    pub fn collider(&self) -> bool {
        match (&self.obj1.shape, &self.obj2.shape) {
            (Shape::Circle, Shape::Circle) => self.circle_collision(),
            (Shape::AABB(_, _), Shape::AABB(_, _)) => self.aabb_collision(),
            _ => false,
        }
    }

    /// Detects if a collision occurs between two circular shapes.
    ///
    /// # Returns
    ///
    /// `true` if a collision is detected, `false` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if either shape is not a circle.
    fn circle_collision(&self) -> bool {
        if self.obj2.shape != Shape::Circle || self.obj1.shape != Shape::Circle {
            return false;
        }
        let distance = self.obj1.vec.distance(self.obj2.vec);
        let radius1 = self.obj1.radius;
        let radius2 = self.obj2.radius;
        distance <= (radius1 + radius2)
    }

    /// Detects if a collision occurs between two axis-aligned bounding boxes (AABBs).
    ///
    /// # Returns
    ///
    /// `true` if a collision is detected, `false` otherwise.
    fn aabb_collision(&self) -> bool {
        let vec1 = self.obj1.shape.get_aabb();
        let vec2 = self.obj2.shape.get_aabb();
        vec1.0.x < vec2.1.x && vec1.1.x > vec2.0.x && vec1.0.y < vec2.1.y && vec1.1.y > vec2.0.y
    }
}
