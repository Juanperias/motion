use super::force::Force;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction2d {
    X,
    Y,
}

pub struct Move2d {
    pub direction: Direction2d,
    pub delta_time: f32,
    pub fps: f32,
}

impl Force for Move2d {
    fn apply_2d(&self, obj: &mut crate::obj::obj_2d::Object2d) {
        let time = self.fps * self.delta_time;

        match self.direction {
            Direction2d::Y => {
                obj.vec.y =
                    obj.vec.y + obj.velocity.y * time + 0.5 * obj.acceleration.y * (time * time);
            }
            Direction2d::X => {
                obj.vec.x =
                    obj.vec.x + obj.velocity.x * time + 0.5 * obj.acceleration.x * (time * time);
            }
        }
    }
}

#[inline]
pub fn move2d<DT: Into<f32>, F: Into<f32>>(
    direction: Direction2d,
    delta_time: DT,
    fps: F,
) -> Move2d {
    Move2d {
        direction,
        delta_time: delta_time.into(),
        fps: fps.into(),
    }
}
