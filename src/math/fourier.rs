use alloc::vec::Vec;
use core::f32::consts::PI;
use crate::math::complex::Complex;
use crate::math::matrix::Matrix;
use crate::math::{expj, sqrt};

fn generate_fourier_vector(j: f32, coeff: Complex<f32>, num_samples: usize) -> Vec<Complex<f32>>{
    let mut vector = Vec::new();
    for i in 0..num_samples{
        vector.push(expj((-2.0 * PI * i as f32 * j) / num_samples as f32) * coeff);
    }
    vector
}

pub fn generate_fourier_basis(num_samples: usize) -> Matrix<Complex<f32>>{
    let mut matrix = Vec::new();
    let coeff = Complex::new(1.0 / sqrt(num_samples as f32),0.0);
    
    for j in 0..num_samples{
        matrix.push(generate_fourier_vector(j as f32,coeff,num_samples));
    }
    
    Matrix::from_vec(matrix)
}

pub fn generate_inverse_fourier_basis(num_samples: usize) -> Matrix<Complex<f32>>{
    let mut matrix = Vec::new();
    let coeff = Complex::new(1.0 / sqrt(num_samples as f32),0.0);

    for j in 0..num_samples{
        matrix.push(generate_fourier_vector(-(j as f32),coeff,num_samples));
    }

    Matrix::from_vec(matrix)
}