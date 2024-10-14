use crate::vec::vec_2d::Vec2d;

use super::sqrt::sqrt;

pub fn dot(u: Vec2d, v: Vec2d) -> f32 {
    u.x * v.x + u.y * v.y
}

pub fn length2(u: Vec2d) -> f32 {
    dot(u, u)
}

pub fn length(u: Vec2d) -> f32 {
    sqrt(length2(u))
}
