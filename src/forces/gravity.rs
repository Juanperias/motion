use crate::obj::obj_2d::Object2d;

use super::force::Force;

pub const EARTH_GRAVITY: f32 = -9.807;

#[derive(Debug)]
pub struct Gravity {
    pub force: f32,
    pub time: f32,
    pub delta_time: f32,
}

impl Force for Gravity {
    fn apply_2d(&self, obj: &mut Object2d) {
        obj.vec.y += obj.velocity.y * self.delta_time
            - 0.5 * self.force * (self.delta_time * self.delta_time);

        obj.velocity.y -= self.force * self.delta_time;
    }
}

#[inline]
pub fn gravity<F: Into<f32>, T: Into<f32>, D: Into<f32>>(
    force: F,
    time: T,
    delta_time: D,
) -> Gravity {
    Gravity {
        force: force.into(),
        time: time.into(),
        delta_time: delta_time.into(),
    }
}
