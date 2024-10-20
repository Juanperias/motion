pub fn powf(base: f32, exponent: i32) -> f32 {
    if exponent == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut exp = exponent.abs();
    let mut base_current = base;

    while exp > 0 {
        if exp & 1 != 0 {
            result *= base_current;
        }
        base_current *= base_current;
        exp >>= 1;
    }

    if exponent < 0 {
        return 1.0 / result;
    }

    result
}
