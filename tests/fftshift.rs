use num_complex::Complex;

use rustdsp::common::fftshift::split_reverse;

#[test]
fn fftshifttest() {
    let mut test_data: Vec<Complex<f32>> = Vec::new();

    for i in 0..100 {
        test_data.push(Complex::new(i as f32, i as f32));
    }

    let test_data_clone = test_data.clone();

    split_reverse(test_data.as_mut_slice());

    dbg!(&test_data_clone);
    dbg!(&test_data);

    assert_eq!(test_data[test_data.len() - 1], test_data_clone[test_data.len() / 2 - 1]);
}