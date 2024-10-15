use crate::formulas::dot::length;
#[derive(Debug, Clone, Copy)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2d { x, y }
    }
    pub fn component(&self, target: Vec2d) -> Vec2d {
        Vec2d {
            x: (target.x - self.x),
            y: (target.y - self.y),
        }
    }
    pub fn magnitude(&self) -> f32 {
        length(*self)
    }
}

#[inline]
pub fn vec2<A: Into<f32>, B: Into<f32>>(a: A, b: B) -> Vec2d {
    Vec2d::new(a.into(), b.into())
}
