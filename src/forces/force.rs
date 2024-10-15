use crate::obj::obj_2d::Object2d;

pub trait Force {
    fn apply_2d(&self, obj: &mut Object2d);
}
