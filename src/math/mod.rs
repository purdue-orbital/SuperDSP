use core::ops::{Add, Mul, Sub};

use crate::math::complex::Complex;

pub mod matrix;
pub mod fourier;
pub mod complex;

pub static PI: i16 = i16::MAX >> 1;
pub static FRAC_PI_2: i16 = i16::MAX >> 2;
pub static FRAC_PI_4: i16 = i16::MAX >> 3;

fn cos_appox_2nd_order_i16(val: i16) -> i16 {
    i16::MAX - (15886 * (val * val))
}

pub fn abs(x: i16) -> i16 { x & 0b0111111111111111 }

/// This preforms the square root of i16s using newton's method
pub fn sqrt(val: i32) -> i32 {
    let mut sqrt = val;
    let mut prev = 0;

    while ((sqrt - prev) & i32::MAX) > 1 {
        prev = sqrt;
        sqrt = sqrt_rec(sqrt, val);
    }

    sqrt
}

pub fn sqrt_rec(val: i32, n: i32) -> i32 {
    (val + (n / val)) >> 1
}

pub fn sincos(val: i16) -> (i16, i16) {
    let mut val: i16 = ((abs(val) + PI) % (PI * 2)) - PI;

    if val < 0 {
        val = -val;
    }

    let mut cos = 0;
    let mut sin = 0;

    if val < -3 * FRAC_PI_4 {
        let calculated = cos_appox_2nd_order_i16(val + PI);
        let other = sqrt(32767 - (calculated as i32  * calculated as i32)) as i16;

        cos = -calculated;
        sin = -other;
    } else if val < -FRAC_PI_4 {
        let calculated = cos_appox_2nd_order_i16(val + FRAC_PI_2);
        let mut other = sqrt(i32::MAX - (calculated as i32  * calculated as i32)) as i16;

        if val < -FRAC_PI_2 {
            other = -other;
        }

        cos = other;
        sin = -calculated;
    } else if val < FRAC_PI_4 {
        let calculated = cos_appox_2nd_order_i16(val);
        let mut other = sqrt(i32::MAX - (calculated as i32  * calculated as i32)) as i16;

        if val < 0 {
            other = -other;
        }

        cos = calculated;
        sin = other;
    } else if val < 3 * FRAC_PI_4 {
        let calculated = cos_appox_2nd_order_i16(val - FRAC_PI_2);
        let mut other = sqrt(i32::MAX - (calculated as i32  * calculated as i32)) as i16;

        if val > FRAC_PI_2 {
            other = -other;
        }

        cos = other;
        sin = calculated;
    } else {
        let calculated = cos_appox_2nd_order_i16(val - PI);
        let other = (sqrt(i32::MAX - (calculated as i32  * calculated as i32))) as i16;

        cos = -calculated;
        sin = other;
    }

    (sin, cos)
}

pub fn sin(val: i16) -> i16 {
    sincos(val).0
}

pub fn cos(val: i16) -> i16 {
    sincos(val).1
}


/// Preforms e^(i*self)
pub fn expj(val: i16) -> Complex<i16> {
    let out = sincos(val);

    Complex {
        re: out.1,
        im: out.0,
    }
}
