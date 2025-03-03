use num::Complex;
use superdsp_core::{Res, ResMut};

/// This assumes the arrays are the same length
pub fn i12_to_f32(i12_arr: Res<Vec<Complex<i16>>>, mut f32_arr: ResMut<Vec<Complex<f32>>>) {
    for (index, x) in i12_arr.iter().enumerate() {
        f32_arr[index] = Complex::new(x.re as f32, x.re as f32) / 2048.0;
    }
}

/// This assumes the arrays are the same length
pub fn f32_to_i12(mut i12_arr: ResMut<Vec<Complex<i16>>>, f32_arr: Res<Vec<Complex<f32>>>) {
    for (index, x) in i12_arr.iter_mut().enumerate() {
        *x = Complex::new((f32_arr[index].re * 2048.0) as i16, (f32_arr[index].im * 2048.0) as i16)
    }
}