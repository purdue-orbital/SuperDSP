use core::f32::consts::PI;
use libm::sqrtf;

use nalgebra::SMatrix;
use num::Complex;

#[cfg(not(feature = "fixed-point"))]
use superdsp::math::fourier::{fft_shift, fft_shift_inverse, make_basis, make_inverse_basis};

const TEST_BASIS_LEN: usize = 4;

#[cfg(not(feature = "fixed-point"))]
#[test]
fn test_basis() {
    let basis: SMatrix<Complex<f32>, TEST_BASIS_LEN, TEST_BASIS_LEN> = make_basis();

    let expected = [
        num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, -0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, 0.5),
        num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, 0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, -0.5)
    ];

    let sliced = basis.as_slice();

    for (index, x) in sliced.iter().enumerate() {
        let diff = expected[index] - x;
        assert!(diff.re.abs() < 0.0001);
        assert!(diff.im.abs() < 0.0001);
    }
}


const TEST_DFT_LEN: usize = 8;

#[cfg(not(feature = "fixed-point"))]
#[test]
fn test_dft(){
    let basis = make_basis();
    let mut wave = nalgebra::SVector::<Complex<f32>, TEST_DFT_LEN>::zeros();
    
    let sample_rate = 4.0;
    let frequency = 2.0;
    

    for i in 0..8 {
        wave[i] = Complex::new(libm::cosf(2.0 * frequency * PI * i as f32 / sample_rate), libm::sinf(2.0 * frequency * PI * i as f32 / sample_rate));
    }

    let m = basis * wave;

    let expected = [
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(sqrtf(8.0),0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
    ];

    for i in 0..4 {
        let diff = m[i] - expected[i];
        assert!(diff.re.abs() < 0.0001);
        assert!(diff.im.abs() < 0.0001);
    }
}
#[cfg(not(feature = "fixed-point"))]
#[test]
fn test_dft_fft_shift(){
    let basis = make_basis();
    let fft_shift = fft_shift();
    let mut wave = nalgebra::SVector::<Complex<f32>, TEST_DFT_LEN>::zeros();

    let sample_rate = 4.0;
    let frequency = 2.0;
    
    for i in 0..8 {
        wave[i] = Complex::new(libm::cosf(2.0 * frequency * PI * i as f32 / sample_rate), libm::sinf(2.0 * frequency * PI * i as f32 / sample_rate));
    }

    let m = basis * fft_shift * wave;

    let expected = [
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(0.0,0.0),
        Complex::new(sqrtf(8.0),0.0),
    ];

    for i in 0..4 {
        let diff = m[i] - expected[i];
        assert!(diff.re.abs() < 0.0001);
        assert!(diff.im.abs() < 0.0001);
    }
}
#[cfg(not(feature = "fixed-point"))]
#[test]
fn test_idft() {
    let basis = make_basis();
    let mut wave = nalgebra::SVector::<Complex<f32>, TEST_DFT_LEN>::zeros();

    for i in 0..8 {
        wave[i] = Complex::new(libm::cosf(2.0 * PI * 4.0 * i as f32 / 8.0), libm::sinf(2.0 * PI * 4.0 * i as f32 / 8.0));
    }

    let m = basis * wave;
    let inverted = make_inverse_basis() * m;

    for i in 0..8 {
        let diff = wave[i] - inverted[i];
        assert!(diff.re.abs() < 0.0001);
        assert!(diff.im.abs() < 0.0001);
    }
}

const TEST_FFT_LEN_1: usize = 4;
const TEST_FFT_LEN_2: usize = 5;

#[cfg(not(feature = "fixed-point"))]
#[test]
fn test_fft_shift() {
    let shift = fft_shift();
    let mut m: nalgebra::SVector::<Complex<f32>, TEST_FFT_LEN_1> = nalgebra::SVector::zeros();

    m[0] = Complex::new(1.0, 0.0);
    m[1] = Complex::new(2.0, 0.0);
    m[2] = Complex::new(3.0, 0.0);
    m[3] = Complex::new(4.0, 0.0);

    let out = shift * &m;
    assert_eq!(out.as_slice(), &[Complex::new(4.0, 0.0), Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0)]);
    let out = fft_shift_inverse() * out;
    assert_eq!(out.as_slice(), &[Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)]);

    let shift = fft_shift();
    let mut m: nalgebra::SVector::<Complex<f32>, TEST_FFT_LEN_2> = nalgebra::SVector::zeros();

    m[0] = Complex::new(1.0, 0.0);
    m[1] = Complex::new(2.0, 0.0);
    m[2] = Complex::new(3.0, 0.0);
    m[3] = Complex::new(4.0, 0.0);
    m[4] = Complex::new(5.0, 0.0);

    let out = shift * m;
    assert_eq!(out.as_slice(), &[Complex::new(4.0, 0.0), Complex::new(5.0, 0.0), Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0)]);

    let out = fft_shift_inverse() * out;
    assert_eq!(out.as_slice(), &[Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0), Complex::new(4.0, 0.0), Complex::new(5.0, 0.0)]);
}
