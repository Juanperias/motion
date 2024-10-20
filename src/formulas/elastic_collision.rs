use crate::vec::vec_2d::Vec2d;

pub fn calculate(v1: Vec2d, m1: f32, m2: f32, v2: Vec2d) -> Vec2d {
    let numerator_x = v1.x * (m1 - m2) + 2.0 * v2.x * m2;
    let numerator_y = v1.y * (m1 - m2) + 2.0 * v2.y * m2;

    let denominator = m1 + m2;

    Vec2d::new(numerator_x / denominator, numerator_y / denominator)
}
