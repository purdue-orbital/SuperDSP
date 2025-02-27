use num_complex::Complex;
use rustdsp::prelude::etc::wave_gen::wave_gen;

#[test]
pub fn test_wave_gen() {
    let frequency = 1.0;
    let taps = 16;
    let sample_rate = 16.0;

    let expected = [Complex::new(1.0, 0.0),
        Complex::new(0.9238795, 0.38268346),
        Complex::new(0.70710677, 0.70710677),
        Complex::new(0.38268346, 0.9238795),
        Complex::new(0.0, 1.0),
        Complex::new(-0.38268346, 0.9238795),
        Complex::new(-0.70710677, 0.70710677),
        Complex::new(-0.9238795, 0.38268346),
        Complex::new(-1.0, 0.0),
        Complex::new(-0.9238795, -0.38268346),
        Complex::new(-0.70710677, -0.70710677),
        Complex::new(-0.38268346, -0.9238795),
        Complex::new(0.0, -1.0),
        Complex::new(0.38268346, -0.9238795),
        Complex::new(0.70710677, -0.70710677),
        Complex::new(0.9238795, -0.38268346)];

    for (a, b) in wave_gen(frequency, taps, sample_rate).iter().zip(expected.iter()) {
        assert!((a.re - b.re).abs() < 0.0001);
        assert!((a.im - b.im).abs() < 0.0001);
    }
}