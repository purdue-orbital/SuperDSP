use nalgebra::SMatrix;
use num::Complex;
use superdsp::math;

#[test]
pub fn test_function_make_basis_vector() {
    let i = 1;
    let expected = nalgebra::Vector4::new(num::Complex::new(0.5, 0.0), num::Complex::new(0.0, -0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, 0.5));
    let result: SMatrix<Complex<f32>, 4, 1> = math::fourier::make_basis_vector(i as f32);

    for (r, e) in result.iter().zip(expected.iter()) {
        let comp = r - e;
        
        assert!((comp.re < 1e-6) && (comp.im < 1e-6));
        assert!((comp.re > -1e-6) && (comp.im > -1e-6));
    }
}

#[test]
pub fn test_function_make_basis() {
    let expected = nalgebra::Matrix4::new(
        num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, -0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, 0.5),
        num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, 0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, -0.5)
    );
    
    
    let result: SMatrix<Complex<f32>, 4, 4> = math::fourier::make_basis();

    for (r, e) in result.iter().zip(expected.iter()) {
        let comp = r - e;
        assert!((comp.re < 1e-6) && (comp.im < 1e-6));
        assert!((comp.re > -1e-6) && (comp.im > -1e-6));
    }
}

#[test]
pub fn test_function_make_inverse_basis() {
    let expected = nalgebra::Matrix4::new(
        num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, 0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, -0.5),
        num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0), num::Complex::new(0.5, 0.0), num::Complex::new(-0.5, 0.0),
        num::Complex::new(0.5, 0.0), num::Complex::new(0.0, -0.5), num::Complex::new(-0.5, 0.0), num::Complex::new(0.0, 0.5)
    );
    
    let result: SMatrix<Complex<f32>,4,4> = math::fourier::make_inverse_basis();

    for (r, e) in result.iter().zip(expected.iter()) {
        let comp = r - e;
        assert!((comp.re < 1e-6) && (comp.im < 1e-6));
        assert!((comp.re > -1e-6) && (comp.im > -1e-6));
    }
}

#[test]
pub fn test_function_fft_shift() {
    let expected = nalgebra::Matrix4::new(
        num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(1.0, 0.0),
            num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0),
            num::Complex::new(0.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0),
            num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 0.0)
    );
    let result: SMatrix<Complex<f32>, 4,4> = math::fourier::fft_shift();
    assert_eq!(result, expected);
}

#[test]
pub fn test_function_fft_shift_inverse() {
    let expected = nalgebra::Matrix4::new(
        num::Complex::new(0.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0),
        num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 0.0),
        num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(1.0, 0.0),
        num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0), num::Complex::new(0.0, 0.0)
    );
    let result: SMatrix<Complex<f32>, 4 ,4> = math::fourier::fft_shift_inverse();
    assert_eq!(result, expected);
}