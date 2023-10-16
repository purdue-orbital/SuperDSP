use num_complex::Complex;

pub trait Formula {
    fn filter_formula(current_value: Complex<f32>, previous_value: Complex<f32>) -> Complex<f32>; 
}