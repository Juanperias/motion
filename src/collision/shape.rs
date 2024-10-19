use crate::vec::vec_2d::Vec2d;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    None,
    Circle,
    AABB(Vec2d, Vec2d),
}

impl Shape {
    pub fn get_aabb(&self) -> (Vec2d, Vec2d) {
        if let Shape::AABB(v1, v2) = self {
            (*v1, *v2)
        } else {
            panic!("Shape is not an AABB");
        }
    }
}
