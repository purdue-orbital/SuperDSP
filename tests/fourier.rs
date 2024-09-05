use core::f64::consts::PI;
use num::Complex;

use superdsp::math::fourier::{fft_shift, fft_shift_inverse, make_basis, make_inverse_basis};

#[test]
fn test_basis(){
    let basis = make_basis(4);
    
    let expected = [[
        num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, -0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, 0.5),
        num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, 0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, -0.5)
    ]];
    
    let sliced = basis.as_slice();
    let sliced = sliced.as_slice();
    
    for (index,x) in sliced[0].iter().enumerate() {
        let diff = expected[0][index] - x;
        assert!(diff.re.abs() < 0.0001);
        assert!(diff.im.abs() < 0.0001);
    }
}

#[test]
fn test_dft(){
    let basis = make_basis(8);
    let mut wave = ndarray::Array1::default(8);

    for i in 0..8{
        wave[i] = num::Complex::new(libm::cos(2.0 * PI * 4.0 * i as f64 / 8.0), libm::sin(2.0 * PI * 4.0 * i as f64 / 8.0));
    }

    let m = basis.dot(&wave);
    let inverted = make_inverse_basis(8).dot(&m);

    for i in 0..8{
        let diff = wave[i] - inverted[i];
        assert!(diff.re.abs() < 0.0001);
        assert!(diff.im.abs() < 0.0001);
    }
}

#[test]
fn test_fft_shift(){
    let shift = fft_shift(4);
    let mut m: ndarray::Array1<Complex<f64>> = ndarray::Array1::ones(4);
    
    m[1] = Complex::new(2.0,0.0);
    m[2] = Complex::new(3.0,0.0);
    m[3] = Complex::new(4.0,0.0); 

    let out = shift.dot(&m);
    assert_eq!(out.as_slice().unwrap(), &[Complex::new(4.0,0.0),Complex::new(1.0,0.0),Complex::new(2.0,0.0),Complex::new(3.0,0.0)]);
    let out = fft_shift_inverse(4).dot(&out);
    assert_eq!(out.as_slice().unwrap(), &[Complex::new(1.0,0.0),Complex::new(2.0,0.0),Complex::new(3.0,0.0),Complex::new(4.0,0.0)]);

    let shift = fft_shift(5);
    let mut m: ndarray::Array1<Complex<f64>> = ndarray::Array1::ones(5);
    
    m[1] = Complex::new(2.0,0.0);
    m[2] = Complex::new(3.0,0.0);
    m[3] = Complex::new(4.0,0.0);
    m[4] = Complex::new(5.0,0.0);

    let out = shift.dot(&m);
    assert_eq!(out.as_slice().unwrap(), &[Complex::new(4.0,0.0),Complex::new(5.0,0.0),Complex::new(1.0,0.0),Complex::new(2.0,0.0),Complex::new(3.0,0.0)]);

    let out = fft_shift_inverse(5).dot(&out);
    assert_eq!(out.as_slice().unwrap(), &[Complex::new(1.0,0.0),Complex::new(2.0,0.0),Complex::new(3.0,0.0),Complex::new(4.0,0.0),Complex::new(5.0,0.0)]);
}
