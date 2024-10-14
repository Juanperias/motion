pub fn sqrt(number: f32) -> f32 {
    if number < 0.0 {
        return f32::NAN;
    }

    f32::from_bits((number.to_bits() + 0x3f80_0000) >> 1)
}
