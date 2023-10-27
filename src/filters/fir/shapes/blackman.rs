use std::f32::consts::PI;

use num_complex::Complex;

use crate::filters::fir::shapes::shape::Shape;

pub struct Blackman;

impl Shape for Blackman {
    fn generate_shape(fft_size: usize, alpha: f32) -> Vec<Complex<f32>> {
        let mut to_return = Vec::with_capacity(fft_size);

        let alpha_half = alpha / 2.0;

        // Generate window
        for x in 0..fft_size {
            let value: f32 = alpha - 0.5 * ((2.0 * PI * x as f32) / fft_size as f32).cos() + alpha_half * ((4.0 * PI * x as f32) / fft_size as f32).cos();
            to_return.push(Complex::new(value, value));
        }

        to_return
    }
}