use crate::obj::obj_2d::Object2d;

use super::force::Force;

pub const EARTH_GRAVITY: f32 = 9.807;

#[derive(Debug)]
pub struct Gravity {
    pub force: f32,
    pub time: f32,
}

impl Force for Gravity {
    fn apply_2d(&self, obj: &mut Object2d) {
        let y = obj.vec.y + obj.velocity.y * self.time - 0.5 * self.force * (self.time * self.time);
        obj.vec.y = y;
        obj.velocity.y -= self.force * self.time;
    }
}

#[inline]
pub fn gravity<F: Into<f32>, T: Into<f32>>(force: F, time: T) -> Gravity {
    Gravity {
        force: force.into(),
        time: time.into(),
    }
}
