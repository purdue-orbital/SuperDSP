use std::f32::consts::PI;

use rustdsp::basic::dft::dft_with_shift;
use rustdsp::basic::etc::frequency_shift;
use rustdsp::math::expj;
use rustdsp::math::matrix::Matrix;

#[test]
pub fn shift() {
    let frequency = 1.0;
    let sample_rate = 4.0;
    let num_samples = 16;
    let inverse_sample_rate = 1.0 / sample_rate;

    let mut wave = Vec::new();
    for x in 0..num_samples {
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate))
    }
    let input = Matrix::from_vec(vec![wave]);

    let shift = frequency_shift(num_samples, sample_rate, 0.5);

    let main_matrix = dft_with_shift(num_samples) * shift;

    assert_eq!((input * main_matrix).cpu_matrix[0][14].re, 4.0)
}