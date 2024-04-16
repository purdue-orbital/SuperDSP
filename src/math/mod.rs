use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use core::ops::{Add, Mul, Sub};
use crate::math::complex::Complex;

pub mod matrix;
pub mod fourier;
pub mod complex;

fn cos_appox_2nd_order(val: f32) -> f32{
    1.0 -(0.484_803_33 * (val * val))
}

pub fn abs(x: f32) -> f32{
    f32::from_bits(x.to_bits() & (i32::MAX as u32))
}
/// This preforms the square root of f32s using newton's method
pub fn sqrt(val: f32) -> f32{
    let mut sqrt = val;
    let mut prev = 0.0;

    while abs(sqrt - prev) > 0.0000000001{
        prev = sqrt;
        sqrt = sqrt_rec(sqrt, val);
    }

    sqrt
}

pub fn sqrt_rec(val: f32, n: f32) -> f32{
    0.5 * (val + (n / val))
}

pub fn sincos<T>(val: T) -> (T, T)
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Into<f32> + From<f32> {
    let mut val_f32: f32 = ((abs(val.into()) + PI) % (2.0 * PI)) - PI;
    
    if val.into() < 0.0{
        val_f32 = -val_f32;
    }

    let mut cos = 0.0;
    let mut sin = 0.0;

    if val_f32 < - 3.0 * FRAC_PI_4 {
        let calculated = cos_appox_2nd_order(val_f32 + PI);
        let other = sqrt(1.0 - (calculated * calculated));

        cos = -calculated;
        sin = -other;
    } else if val_f32 < -FRAC_PI_4 {
        let calculated = cos_appox_2nd_order(val_f32 + FRAC_PI_2);
        let mut other = sqrt(1.0 - (calculated * calculated));

        if val_f32 < -FRAC_PI_2{
            other = -other;
        }

        cos = other;
        sin = -calculated;
    } else if val_f32 < FRAC_PI_4 {
        let calculated = cos_appox_2nd_order(val_f32);
        let mut other = sqrt(1.0 - (calculated * calculated));

        if val_f32 < 0.0 {
            other = -other;
        }

        cos = calculated;
        sin = other;
    } else if val_f32 < 3.0 * FRAC_PI_4 {
        let calculated = cos_appox_2nd_order(val_f32 - FRAC_PI_2);
        let mut other = sqrt(1.0 - (calculated * calculated));

        if val_f32 > FRAC_PI_2 {
            other = -other;
        }

        cos = other;
        sin = calculated;
    } else {
        let calculated = cos_appox_2nd_order(val_f32 - PI);
        let other = sqrt(1.0 - (calculated * calculated));

        cos = -calculated;
        sin = other;
    }

    (sin.into(), cos.into())
}

pub fn sin<T>(val: T) -> T
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Into<f32> + From<f32>{
    sincos(val).0
}

pub fn cos<T>(val: T) -> T
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Into<f32> + From<f32>{
    sincos(val).1
}


/// Preforms e^(i*self)
pub fn expj<T>(val: T) -> Complex<T>
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Into<f32> + From<f32>{
    let out = sincos(val);

    Complex{
        re: out.1,
        im: out.0,
    }
}
