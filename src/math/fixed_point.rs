/// Credit: javidx https://www.youtube.com/watch?v=ZMsrZvBmQnU
pub fn f32_to_i32_fixed(val:f32) -> i32 {
    ((val * 65536.0) + (if val >= 0.0 { 0.5 } else { -0.5 } )) as i32
}

pub fn i32_fixed_to_f32(val: i32) -> f32{
    (val as f32) / 65536.0
}

pub fn fixed_mul(a: i32, b: i32) -> i32 {
    (((a as i64) * (b as i64)) >> 16) as i32
}