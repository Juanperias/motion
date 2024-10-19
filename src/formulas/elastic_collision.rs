use crate::{obj::obj_2d::Object2d, vec::vec_2d::Vec2d};

pub fn calculate(obj1: Object2d, obj2: Object2d) -> Vec2d {
    let x = obj1.velocity.x * (obj1.mass - obj2.mass) + 2.0 * obj2.velocity.x * obj2.mass;
    let y = obj1.velocity.y * (obj1.mass - obj2.mass) + 2.0 * obj2.velocity.y * obj2.mass;
    let denominator = obj1.mass + obj2.mass;
    Vec2d::new(x / denominator, y / denominator)
}
