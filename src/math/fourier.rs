use core::f32::consts::PI;
use core::ops::Mul;
use std::ops::{Add, Div, Neg, Rem, Sub};
use crate::math::complex::Complex;
use crate::math::matrix::Matrix;
use crate::math::sincos::{expj, sqrt};

fn generate_fourier_vector<T>(j: T, coeff: Complex<T>, num_samples: usize) -> Vec<Complex<T>>
    where T: Copy + Sub<T, Output=T> + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> + PartialOrd + Neg<Output=T> + From<f32> + Into<f32> {
    let mut vector = Vec::new();

    for k in 0..num_samples {
        let w_x = expj((T::from(-2.0) * T::from(PI) * j * T::from(k as f32)) / T::from(num_samples as f32)) * coeff;
        vector.push(w_x);
    }
    vector
}

pub fn generate_fourier_basis<T>(num_samples: usize) -> Matrix<Complex<T>>
    where T: Copy + Sub<T, Output=T> + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> + PartialOrd + Neg<Output=T> + From<f32> + Into<f32> {

    let mut matrix = Vec::new();
    let coeff = Complex::new(T::from(1.0) / sqrt(T::from(num_samples as f32)), T::from(0.0));

    for j in 0..num_samples {
        matrix.push(generate_fourier_vector(T::from(j as f32), coeff, num_samples));
    }

    Matrix::from_vec(matrix)
}

pub fn generate_inverse_fourier_basis<T>(num_samples: usize) -> Matrix<Complex<T>>
    where T: Copy + Sub<T, Output=T> + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> + PartialOrd + Neg<Output=T> + From<f32> + Into<f32> {
    let mut matrix = Vec::new();
    let coeff = Complex::new(T::from(1.0) / sqrt(T::from(num_samples as f32)), T::from(0.0));

    for j in 0..num_samples {
        matrix.push(generate_fourier_vector(T::from(-(j as f32)), coeff, num_samples));
    }

    Matrix::from_vec(matrix)
}

pub fn identity_shift<T>(num_samples: usize, shift: isize, coeff: i32) -> Vec<Vec<Complex<T>>>
    where T: Copy + Sub<T, Output=T> + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> + PartialOrd + Neg<Output=T> + From<f32> + Into<f32> {
    let mut matrix = Vec::new();

    for i in 0..num_samples {
        let mut row = Vec::new();
        for j in 0..num_samples {
            if (i as isize + shift) == j as isize {
                row.push(Complex::new(T::from(coeff as f32), T::from(0.0)))
            } else {
                row.push(Complex::new(T::from(0.0), T::from(0.0)))
            }
        }
        matrix.push(row);
    }

    matrix
}

/// Return fft shift matrix (right-hand side only)
pub fn generate_fft_shift<T>(num_samples: usize) -> Matrix<Complex<T>>
    where T: Copy + Sub<T, Output=T> + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> + PartialOrd + Neg<Output=T> + From<f32> + Into<f32> {
    // rotate elements
    let shift = num_samples >> 1;
    Matrix::from_vec(identity_shift(num_samples, shift as isize, 1)) + Matrix::from_vec(identity_shift(num_samples, -(shift as isize), 1))
}

/// Return inverse fft shift matrix (right-hand side only)
pub fn generate_ifft_shift<T>(num_samples: usize) -> Matrix<Complex<T>>
    where T: Copy + Sub<T, Output=T> + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> + PartialOrd + Neg<Output=T> + From<f32> + Into<f32> {
    generate_fft_shift(num_samples)
}