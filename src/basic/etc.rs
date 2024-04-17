use alloc::vec::Vec;
use core::f32::consts::PI;

use crate::math::complex::Complex;
use crate::math::expj;
use crate::math::fourier::identity_shift;
use crate::math::matrix::Matrix;

/// Generates frequency shifting matrix that applies to the right-hand size after dft and fft shift
pub fn frequency_shift(num_samples: usize, sample_rate: f32, frequency_shift: f32) -> Matrix<Complex<f32>> {
    // calculate shift
    let shift = frequency_shift * (num_samples as f32 / sample_rate);
    let coeff2 = shift - (shift as i32) as f32;
    let coeff1 = 1.0 - coeff2;

    let matrix1 = Matrix::from_vec(identity_shift(num_samples, shift as isize, coeff1));
    let matrix2 = Matrix::from_vec(identity_shift(num_samples, (shift + 1.0) as isize, coeff2));

    matrix1 + matrix2
}

/// Generate wave given a frequency
pub fn generate_wave(frequency: f32, sample_rate: f32, offset: f32, num_samples: usize) -> Matrix<Complex<f32>> {
    let mut wave = Vec::new();
    let inverse_sample_rate = 1.0 / sample_rate;

    for x in 0..num_samples {
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate + offset))
    }

    Matrix::from_vec([wave].to_vec())
}