use num_complex::Complex;
use crate::filters::fir::shapes::shape::Shape;
use std::f32::consts::PI;

pub struct Hann;

impl Shape for Hann{ 
    // alpha should be set to 0.5 for hann function
    fn generate_shape(fft_size: usize, alpha:i16) -> Vec<Complex<f32>> {
        let mut to_return = Vec::with_capacity(fft_size);

        // Generate window
        for x in 0..fft_size{
            let value: f32 = alpha as f32 * (1.0 - ((2.0 * x as f32 * PI) / fft_size as f32).cos());
            to_return.push(Complex::new(value,value));
        }

        to_return
    }
}