use crate::vec::vec_2d::Vec2d;

/// Represents different shapes in a 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    /// No shape defined.
    None,
    /// A circular shape.
    Circle,
    /// An axis-aligned bounding box (AABB) defined by two vectors.
    AABB(Vec2d, Vec2d),
}

impl Shape {
    /// Retrieves the AABB (axis-aligned bounding box) vectors if the shape is an AABB.
    ///
    /// # Examples
    ///
    /// ```
    /// let shape = Shape::AABB(Vec2d::new(0.0, 0.0), Vec2d::new(1.0, 1.0));
    /// let (v1, v2) = shape.get_aabb();
    /// assert_eq!(v1, Vec2d::new(0.0, 0.0));
    /// assert_eq!(v2, Vec2d::new(1.0, 1.0));
    /// ```
    ///
    /// # Returns
    ///
    /// A tuple containing two `Vec2d` vectors representing the corners of the AABB.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not an AABB.
    #[must_use]
    pub fn get_aabb(&self) -> (Vec2d, Vec2d) {
        if let Shape::AABB(v1, v2) = self {
            (*v1, *v2)
        } else {
            panic!("Shape is not an AABB");
        }
    }
}
