use num_complex::Complex;

use rustdsp::common::rational_resampler::rational_resample;


fn generate_data() -> Vec<Complex<f32>>{
    let mut signal: Vec<Complex<f32>> = Vec::new();
    for i in 0..100 {
        signal.push(Complex::new(i as f32, i as f32));
    }
    signal
}
#[test]
fn test_rational_resample_0_0() {
    let mut signal = generate_data();
    let upsample = 0;
    let downsample = 0;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    assert!(original_length == signal.len());
}

#[test]
fn test_rational_resample_1_1() {
    let mut signal = generate_data();
    let upsample = 1;
    let downsample = 1;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    assert!(original_length == signal.len());
}

#[test]
fn test_rational_resample_1_2() {
    let mut signal = generate_data();
    let upsample = 1;
    let downsample = 2;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length == 2 * signal.len());
}

#[test]
fn test_rational_resample_1_3() {
    let mut signal = generate_data();
    let upsample = 1;
    let downsample = 3;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length / 3 == signal.len());
}

#[test]
fn test_rational_resample_1_4() {
    let mut signal = generate_data();
    let upsample = 1;
    let downsample = 4;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length / 4 == signal.len());
}

#[test]
fn test_rational_resample_2_2() {
    let mut signal = generate_data();
    let upsample = 2;
    let downsample = 2;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length == signal.len());
}

#[test]
fn test_rational_resample_3_3() {
    let mut signal = generate_data();
    let upsample = 3;
    let downsample = 3;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length == signal.len());
}

#[test]
fn test_rational_resample_2_1() {
    let mut signal = generate_data();
    let upsample = 2;
    let downsample = 1;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length * 2 == signal.len());
}

#[test]
fn test_rational_resample_3_1() {
    let mut signal = generate_data();
    let upsample = 3;
    let downsample = 1;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!(original_length * 3 == signal.len());
}

#[test]
fn test_rational_resample_2_3() {
    let mut signal = generate_data();
    let upsample = 2;
    let downsample = 3;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!((original_length as f32 * (2.0 / 3.0)) as usize == signal.len());
}

#[test]
fn test_rational_resample_7_12() {
    let mut signal = generate_data();
    let upsample = 7;
    let downsample = 12;
    let original_length = signal.len();

    rational_resample(upsample, downsample, &mut signal);

    println!("{} {}", original_length, signal.len());
    assert!((original_length as f32 * (7.0 / 12.0)) as usize == signal.len());
}


