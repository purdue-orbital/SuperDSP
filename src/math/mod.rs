use crate::math::complex::Complex;
use crate::math::fixed_point::{f32_to_i32_fixed, fixed_mul};

pub mod matrix;
pub mod fourier;
pub mod complex;
pub mod fixed_point;

pub static PI: i32 = 205887;
pub static FRAC_PI_2: i32 = PI >> 1;
pub static FRAC_PI_4: i32 = PI >> 2;

fn cos_appox_2nd_order_fixed_i32(val: i32) -> i32 {
    65536 - fixed_mul(31772, fixed_mul(val, val))
}

pub fn abs(x: i32) -> i32 {
    let mask = x >> 31;
    (mask + x)^mask
}


pub fn sqrt(x : i32) -> i32{
    let mut low = 0;
    let mut high = (x + (1 << 16)) >> 1;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        let mid_squared = fixed_mul(mid,mid);

        if mid_squared < x {
            low = mid + 1;
        } else if mid_squared > x {
            high = mid - 1;
        } else {
            return mid;
        }
    }

    high
}


pub fn sincos(val: i32) -> (i32, i32) {
    let val: i32 = ((abs(val + PI)) % (PI * 2)) - PI;
    
    if val < -3 * FRAC_PI_4 {
        let cos = -cos_appox_2nd_order_fixed_i32(val + PI);
        let sin = -sqrt(65536 - fixed_mul(cos,cos));
        
        (sin,cos)
    }else if val < -FRAC_PI_4 {
        let sin = -cos_appox_2nd_order_fixed_i32(val + FRAC_PI_2);
        let mut cos = sqrt(65536 - fixed_mul(sin,sin));
        
        if val < -FRAC_PI_2{
            cos = -cos;
        }

        (sin,cos)
    }else if  val < FRAC_PI_4 {
        let cos = cos_appox_2nd_order_fixed_i32(val);
        let mut sin = sqrt(65536 - fixed_mul(cos,cos));

        if val < 0{
            sin = -sin;
        }

        (sin,cos)
    }else if val < 3*FRAC_PI_4{
        let sin = cos_appox_2nd_order_fixed_i32(val - FRAC_PI_2);
        let mut cos = sqrt(65536 - fixed_mul(sin,sin));

        if val > FRAC_PI_2{
            cos = -cos;
        }

        (sin,cos)
    } else {
        let cos = -cos_appox_2nd_order_fixed_i32(val - PI);
        let sin = sqrt(65536 - fixed_mul(cos,cos));

        (sin,cos)
    }
}

pub fn sin(val: i32) -> i32 {
    sincos(val).0
}

pub fn cos(val: i32) -> i32 {
    sincos(val).1
}


/// Preforms e^(i*self)
pub fn expj(val: i32) -> Complex<i32> {
    let out = sincos(val);

    Complex {
        re: out.1,
        im: out.0,
    }
}
