use super::sqrt::sqrt;

pub fn calculate_pt(a: f32, b: f32) -> f32 {
    let ab = a * a + b * b;

    sqrt(ab)
}
