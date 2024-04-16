use std::f32::consts::PI;
use rustdsp::math::complex::Complex;
use rustdsp::math::expj;
use rustdsp::math::fourier::{generate_fft_shift, generate_fourier_basis, generate_inverse_fourier_basis};
use rustdsp::math::matrix::Matrix;

#[test]
pub fn dft() {
    let frequency = 1.0;
    let sample_rate = 2.0;
    let num_samples = 16;
    
    dbg!("test 1");
    
    let inverse_sample_rate =  1.0 / sample_rate;
    
    let matrix = generate_fourier_basis(num_samples);
    
    let mut wave = Vec::new();

    dbg!("test 2");
    
    
    for x in 0..num_samples{
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate))
    }

    dbg!("test 3");
    
    
    let input = Matrix::from_vec(vec![wave]);

    dbg!("test 4");
    
    dbg!(input * matrix);
}

#[test]
pub fn idft() {
    let frequency = 1.0;
    let sample_rate = 2.0;
    let num_samples = 16;

    let inverse_sample_rate =  1.0 / sample_rate;

    let matrix = generate_fourier_basis(num_samples);
    let inverse_matrix = generate_inverse_fourier_basis(num_samples);

    let mut wave = Vec::new();

    for x in 0..num_samples{
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate))
    }

    let input = Matrix::from_vec(vec![wave]);

    dbg!(input * matrix * inverse_matrix);
}

#[test]
pub fn fft_shift_test(){
    let frequency = -1.0;
    let sample_rate = 2.0;
    let num_samples = 16;

    let inverse_sample_rate =  1.0 / sample_rate;

    let matrix = generate_fourier_basis(num_samples);
    let fft_shift = generate_fft_shift(num_samples);

    let mut wave = Vec::new();

    for x in 0..num_samples{
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate))
    }

    let input = Matrix::from_vec(vec![wave]);

    dbg!(input * matrix * fft_shift);
}

#[test]
pub fn filter_test() {
    let frequency = 1.0;
    let sample_rate = 2.0;
    let num_samples = 16;

    let inverse_sample_rate =  1.0 / sample_rate;

    let matrix = generate_fourier_basis(num_samples);
    let inverse_matrix = generate_inverse_fourier_basis(num_samples);
    let filter = Matrix::from_vec(
        vec![
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(1.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),],
            vec![Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),Complex::new(0.0_f32,0.0),],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples],
            vec![Complex::new(0.0_f32,0.0); num_samples]
        ]
    );

    let mut wave = Vec::new();

    for x in 0..num_samples{
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate))
    }

    let input = Matrix::from_vec(vec![wave]);
    
    let main_matrix = matrix * filter * inverse_matrix;

    dbg!(input * main_matrix);
}