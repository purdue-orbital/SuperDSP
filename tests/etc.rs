use std::f32::consts::PI;

//use rustdsp::basic::etc::frequency_shift;
//use rustdsp::math::fixed_point::FixedI32;
use rustdsp::math::matrix::Matrix;

// #[test]
// pub fn shift() {
//     let frequency = 1.0;
//     let sample_rate = 4.0;
//     let num_samples = 16;
//     let inverse_sample_rate = 1.0 / sample_rate;
// 
//     let mut wave = Vec::new();
//     for x in 0..num_samples {
//         wave.push(FixedI32::from_f32(x as f32 * 2.0 * PI * frequency * inverse_sample_rate).expj())
//     }
//     let input = Matrix::from_vec(vec![wave]);
// 
//     let shift = frequency_shift(num_samples, sample_rate, 0.5);
// 
//     let main_matrix = dft_with_shift(num_samples) * shift;
// 
//     assert_eq!((input * main_matrix).cpu_matrix[0][14].re, 4.0)
// }