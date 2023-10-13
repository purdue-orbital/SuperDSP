use std::f32::consts::PI;

use num_complex::Complex;

use crate::filters::fir::shapes::shape::Shape;

pub struct FlatTop;

impl Shape for FlatTop {
    fn generate_shape(fft_size: usize, alpha: i16) -> Vec<Complex<f32>> {
        let mut to_return = Vec::with_capacity(fft_size);
        let coefficients: [f32; 5] = [0.21556895, 0.41663158, 0.277263158, 0.083578947, 0.006947368];
        // Generate window
        for x in 0..fft_size {
            let value: f32 =
                coefficients[0] -
                    coefficients[1] * ((2.0 * PI * x as f32) / fft_size as f32).cos() +
                    coefficients[2] * ((4.0 * PI * x as f32) / fft_size as f32).cos() -
                    coefficients[3] * ((6.0 * PI * x as f32) / fft_size as f32).cos() +
                    coefficients[4] * ((8.0 * PI * x as f32) / fft_size as f32).cos();
            to_return.push(Complex::new(value, value));
        }

        to_return
    }
}