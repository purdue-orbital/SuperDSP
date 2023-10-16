use num_complex::Complex;

use crate::filters::iir::formula::Formula;

pub struct LowPass;

impl Formula for LowPass {
    fn filter_formula(current_value: Complex<f32>, previous_value: Complex<f32>) -> Complex<f32> {
        
    }
}