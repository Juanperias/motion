use shape::Shape;

use crate::obj::obj_2d::Object2d;

pub mod shape;

pub struct Collision2d {
    pub obj1: Object2d,
    pub obj2: Object2d,
}

impl Collision2d {
    pub fn new(obj1: Object2d, obj2: Object2d) -> Self {
        Self { obj1, obj2 }
    }
    pub fn collider<F>(&self, on_collide: F)
    where
        F: Fn(&Object2d, &Object2d),
    {
        let check = match (&self.obj1.shape, &self.obj2.shape) {
            (Shape::Circle, Shape::Circle) => self.circle_collision(),
            (Shape::AABB(_, _), Shape::AABB(_, _)) => self.aabb_collision(),
            _ => false,
        };

        if check {
            on_collide(&self.obj1, &self.obj2);
        }
    }
    fn circle_collision(&self) -> bool {
        if self.obj2.shape != Shape::Circle && self.obj1.shape != Shape::Circle {
            return false;
        }
        let distance = self.obj1.vec.distance(self.obj2.vec);
        let vec1_magnitude = self.obj1.vec.magnitude();
        let vec2_magnitude = self.obj2.vec.magnitude();
        distance <= vec1_magnitude + vec2_magnitude
    }
    fn aabb_collision(&self) -> bool {
        let vec1 = self.obj1.shape.get_aabb();
        let vec2 = self.obj2.shape.get_aabb();

        vec1.0.x < vec2.1.x && vec1.1.x > vec2.0.x && vec1.0.y < vec2.1.y && vec1.1.y > vec2.0.y
    }
}
