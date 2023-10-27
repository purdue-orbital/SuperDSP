use num_complex::Complex;

use crate::filters::fir::shapes::shape::Shape;

pub struct Rectangle;

impl Shape for Rectangle {
    fn generate_shape(fft_size: usize, _alpha: f32) -> Vec<Complex<f32>> {
        vec![Complex::new(1.0, 1.0); fft_size]
    }
}