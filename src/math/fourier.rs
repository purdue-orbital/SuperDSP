use alloc::vec::Vec;
use crate::math::complex::Complex;
use crate::math::matrix::Matrix;

// fn generate_fourier_vector(j: f32, coeff: Complex<f32>, num_samples: usize) -> Vec<Complex<f32>> {
//     let mut vector = Vec::new();
//     for i in 0..num_samples {
//         vector.push(expj((-2.0 * PI * i as f32 * j) / num_samples as f32) * coeff);
//     }
//     vector
// }
// 
// pub fn generate_fourier_basis(num_samples: usize) -> Matrix<Complex<f32>> {
//     let mut matrix = Vec::new();
//     let coeff = Complex::new(1.0 / sqrt(num_samples as f32), 0.0);
// 
//     for j in 0..num_samples {
//         matrix.push(generate_fourier_vector(j as f32, coeff, num_samples));
//     }
// 
//     Matrix::from_vec(matrix)
// }
// 
// pub fn generate_inverse_fourier_basis(num_samples: usize) -> Matrix<Complex<f32>> {
//     let mut matrix = Vec::new();
//     let coeff = Complex::new(1.0 / sqrt(num_samples as f32), 0.0);
// 
//     for j in 0..num_samples {
//         matrix.push(generate_fourier_vector(-(j as f32), coeff, num_samples));
//     }
// 
//     Matrix::from_vec(matrix)
// }

pub fn identity_shift(num_samples: usize, shift: isize, coeff: f32) -> Vec<Vec<Complex<f32>>> {
    let mut matrix = Vec::new();

    for i in 0..num_samples {
        let mut row = Vec::new();
        for j in 0..num_samples {
            if (i as isize + shift) == j as isize {
                row.push(Complex::new(coeff, 0.0))
            } else {
                row.push(Complex::new(0.0, 0.0))
            }
        }
        matrix.push(row);
    }

    matrix
}

/// Return fft shift matrix (right-hand side only)
pub fn generate_fft_shift(num_samples: usize) -> Matrix<Complex<f32>> {
    // rotate elements
    let shift = num_samples >> 1;
    Matrix::from_vec(identity_shift(num_samples, shift as isize, 1.0)) + Matrix::from_vec(identity_shift(num_samples, -(shift as isize), 1.0))
}

/// Return inverse fft shift matrix (right-hand side only)
pub fn generate_ifft_shift(num_samples: usize) -> Matrix<Complex<f32>> {
    generate_fft_shift(num_samples)
}