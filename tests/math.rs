use rustdsp::math;

#[test]
pub fn test_function_make_basis_vector() {
    let i = 1.0;
    let n = 4;
    let expected = ndarray::arr1(&[num::Complex::new(1.0, 0.0), num::Complex::new(0.0, -1.0), num::Complex::new(-1.0, 0.0), num::Complex::new(0.0, 1.0)]);
    let result = math::fourier::make_basis_vector(i, n);

    for (r, e) in result.iter().zip(expected.iter()) {
        let comp = r - e;
        assert!((comp.re < 1e-6) && (comp.im < 1e-6));
        assert!((comp.re > -1e-6) && (comp.im > -1e-6));
    }
}

#[test]
pub fn test_function_make_basis() {
    let n = 4;
    let expected = ndarray::arr2(&[[num::Complex::new(1.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(1.0, 0.0)],
        [num::Complex::new(1.0, 0.0), num::Complex::new(0.0, -1.0), num::Complex::new(-1.0, 0.0), num::Complex::new(0.0, 1.0)],
        [num::Complex::new(1.0, 0.0), num::Complex::new(-1.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(-1.0, 0.0)],
        [num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 1.0), num::Complex::new(-1.0, 0.0), num::Complex::new(0.0, -1.0)]]) / 2.0;
    let result = math::fourier::make_basis(n);

    for (r, e) in result.iter().zip(expected.iter()) {
        let comp = r - e;
        assert!((comp.re < 1e-6) && (comp.im < 1e-6));
        assert!((comp.re > -1e-6) && (comp.im > -1e-6));
    }
}

#[test]
pub fn test_function_make_inverse_basis() {
    let n = 4;
    let expected = ndarray::arr2(&[[num::Complex::new(1.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(1.0, 0.0)],
        [num::Complex::new(1.0, 0.0), num::Complex::new(0.0, 1.0), num::Complex::new(-1.0, 0.0), num::Complex::new(0.0, -1.0)],
        [num::Complex::new(1.0, 0.0), num::Complex::new(-1.0, 0.0), num::Complex::new(1.0, 0.0), num::Complex::new(-1.0, 0.0)],
        [num::Complex::new(1.0, 0.0), num::Complex::new(0.0, -1.0), num::Complex::new(-1.0, 0.0), num::Complex::new(0.0, 1.0)]]) / 2.0;
    let result = math::fourier::make_inverse_basis(n);

    for (r, e) in result.iter().zip(expected.iter()) {
        let comp = r - e;
        assert!((comp.re < 1e-6) && (comp.im < 1e-6));
        assert!((comp.re > -1e-6) && (comp.im > -1e-6));
    }
}

#[test]
pub fn test_function_fft_shift() {
    let n = 4;
    let expected = ndarray::arr2(&[
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0]]);
    let result = math::fourier::fft_shift(n);
    assert_eq!(result, expected);
}

#[test]
pub fn test_function_fft_shift_inverse() {
    let n = 4;
    let expected = ndarray::arr2(&[
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 0.0]]);
    let result = math::fourier::fft_shift_inverse(n);
    assert_eq!(result, expected);
}