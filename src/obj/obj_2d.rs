use crate::{forces::force::Force, vec::vec_2d::Vec2d};

#[derive(Debug, Clone)]
pub struct Object2d {
    pub vec: Vec2d,
    pub density: f32,
    pub mass: f32,
    pub velocity: Vec2d,
    pub acceleration: Vec2d,
    pub radius: f32,
}

impl Object2d {
    pub fn new(
        vec: Vec2d,
        density: f32,
        mass: f32,
        velocity: Vec2d,
        acceleration: Vec2d,
        radius: f32,
    ) -> Self {
        Object2d {
            vec,
            density,
            mass,
            velocity,
            acceleration,
            radius,
        }
    }
    pub fn apply<T: Force>(&mut self, force: T) {
        force.apply_2d(self);
    }
}

#[inline]
pub fn obj2d<V: Into<Vec2d>>(
    vec: V,
    density: f32,
    mass: f32,
    velocity: V,
    acceleration: V,
    radius: f32,
) -> Object2d {
    Object2d::new(
        vec.into(),
        density,
        mass,
        velocity.into(),
        acceleration.into(),
        radius,
    )
}