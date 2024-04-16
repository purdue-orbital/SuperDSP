use alloc::vec::Vec;
use crate::math::complex::Complex;
use crate::math::matrix::Matrix;

/// Creates a rectangle bandpass filter
pub fn rectangle(start_frequency: f32, stop_frequency: f32, sample_rate: f32, num_samples: usize) -> Matrix<Complex<f32>>{
    let mut matrix_vector = Vec::new();
    
    let index_to_frequency = (sample_rate * 0.5) / num_samples as f32;
    
    for i in 0..num_samples {
        let mut row = Vec::new();
        
        for j in 0..num_samples{
        }
        
        matrix_vector.push(row);
    }
    
    Matrix::from_vec(matrix_vector)
}