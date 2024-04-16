use crate::math::complex::Complex;
use crate::math::fourier::{generate_fourier_basis, generate_inverse_fourier_basis};
use crate::math::matrix::Matrix;

/// Returns dft matrix
pub fn dft(num_samples: usize) -> Matrix<Complex<f32>>{
    generate_fourier_basis(num_samples)
}

/// Returns idft matrix
pub fn idft(num_samples: usize) -> Matrix<Complex<f32>>{
    generate_inverse_fourier_basis(num_samples)
}