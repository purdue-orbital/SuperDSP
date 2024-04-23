use crate::math::complex::Complex;
//use crate::math::fourier::{generate_fft_shift, generate_fourier_basis, generate_ifft_shift, generate_inverse_fourier_basis};
use crate::math::matrix::Matrix;
// 
// /// Returns dft matrix
// pub fn dft(num_samples: usize) -> Matrix<Complex<f32>> {
//     generate_fourier_basis(num_samples)
// }
// 
// /// Returns idft matrix
// pub fn idft(num_samples: usize) -> Matrix<Complex<f32>> {
//     generate_inverse_fourier_basis(num_samples)
// }
// 
// /// Returns fft shift matrix
// pub fn fft_shift(num_samples: usize) -> Matrix<Complex<f32>> {
//     generate_fft_shift(num_samples)
// }
// 
// /// Returns inverse fft shift matrix
// pub fn inverse_fft_shift(num_samples: usize) -> Matrix<Complex<f32>> {
//     generate_ifft_shift(num_samples)
// }
// 
// /// Returns dft with fft shift matrix
// pub fn dft_with_shift(num_samples: usize) -> Matrix<Complex<f32>> {
//     dft(num_samples) * fft_shift(num_samples)
// }
// 
// /// Returns idft with inverse fft shift matrix
// pub fn idft_with_inverse_shift(num_samples: usize) -> Matrix<Complex<f32>> {
//     idft(num_samples) * inverse_fft_shift(num_samples)
// }