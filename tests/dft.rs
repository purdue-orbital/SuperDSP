use rustdsp::basic::etc::generate_wave;
use rustdsp::math::fourier::{generate_fft_shift, generate_fourier_basis, generate_ifft_shift};

#[test]
pub fn dft() {
    let frequency = 1.0;
    let sample_rate = 4.0;
    let num_samples = 16;

    let matrix = generate_fourier_basis(num_samples);
    let input = generate_wave(frequency, sample_rate, 0.0, num_samples);

    assert_eq!((input * matrix).cpu_matrix[0][4].re, 4.0);
}

#[test]
pub fn idft() {
/*    let frequency = 1.0;
    let sample_rate = 4.0;
    let num_samples = 16;


    let matrix = generate_fourier_basis(num_samples);
    let inverse_matrix = generate_inverse_fourier_basis(num_samples);
    let input = generate_wave(frequency, sample_rate, 0.0, num_samples);*/
}

#[test]
pub fn fft_shift_test() {
    let frequency = 1.0;
    let sample_rate = 4.0;
    let num_samples = 16;

    let matrix = generate_fourier_basis(num_samples);
    let fft_shift = generate_fft_shift(num_samples);
    let iff_shift = generate_ifft_shift(num_samples);
    let input = generate_wave(frequency, sample_rate, 0.0, num_samples);

    assert_eq!(input.clone() * matrix.clone() * fft_shift * iff_shift, input.clone() * matrix.clone());
}