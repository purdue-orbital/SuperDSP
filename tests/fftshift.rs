use num_complex::Complex;

use rustdsp::common::fftshift::split_reverse;

#[test]
fn fftshifttest() {
    let mut test_data: Vec<Complex<f32>> = Vec::new();

    for i in 0..100 {
        test_data.push(Complex::new(i as f32, i as f32));
    }

    let shifted_data: Vec<Complex<f32>> = split_reverse(&mut test_data);

    assert!(test_data[test_data.len() - 1] == shifted_data[test_data.len() / 2 - 1]);
}