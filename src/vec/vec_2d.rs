use crate::formulas::dot::length;

/// `Vec2d` is a simple 2D vector struct used for various vector operations.
///
/// # Examples
///
/// ```rust
///
/// let v1 = Vec2d::new(3.0, 4.0);
/// let v2 = Vec2d::new(6.0, 8.0);
///
/// assert_eq!(v1.magnitude(), 5.0);
/// assert_eq!(v1.distance(v2), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    /// The x component of the vector
    pub x: f32,
    /// The y component of the vector
    pub y: f32,
}

impl Vec2d {
    /// Creates a new `Vec2d` with the given x and y components.
    ///
    /// # Examples
    ///
    /// ```
    /// let vec = Vec2d::new(1.0, 2.0);
    /// assert_eq!(vec.x, 1.0);
    /// assert_eq!(vec.y, 2.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Vec2d { x, y }
    }

    /// Computes the component vector from this vector to the target vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = Vec2d::new(1.0, 2.0);
    /// let v2 = Vec2d::new(4.0, 6.0);
    /// let component = v1.component(v2);
    /// assert_eq!(component.x, 3.0);
    /// assert_eq!(component.y, 4.0);
    /// ```
    pub fn component(&self, target: Vec2d) -> Vec2d {
        Vec2d {
            x: (target.x - self.x),
            y: (target.y - self.y),
        }
    }

    /// Computes the magnitude (length) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = Vec2d::new(3.0, 4.0);
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    pub fn magnitude(&self) -> f32 {
        length(*self)
    }

    /// Computes the distance between this vector and the target vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = Vec2d::new(1.0, 2.0);
    /// let v2 = Vec2d::new(4.0, 6.0);
    /// assert_eq!(v1.distance(v2), 5.0);
    /// ```
    pub fn distance(&self, target: Vec2d) -> f32 {
        let ab = self.component(target);
        length(ab)
    }
}

/// A convenience function to create a new `Vec2d`.
///
/// # Examples
///
/// ```
/// let vec = vec2(1.0, 2.0);
/// assert_eq!(vec.x, 1.0);
/// assert_eq!(vec.y, 2.0);
/// ```
#[inline]
pub fn vec2<A: Into<f32>, B: Into<f32>>(a: A, b: B) -> Vec2d {
    Vec2d::new(a.into(), b.into())
}
