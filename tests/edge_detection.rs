use num_complex::Complex;
use rustdsp::prelude::etc::edge_detection::{falling_edge_detector, rising_edge_detector};

#[test]
pub fn test_rising_edge_detection() {
    let test = vec![Complex::new(0, 0), Complex::new(0, 0), Complex::new(1, 0), Complex::new(0, 0), Complex::new(0, 0), Complex::new(1, 0), Complex::new(0, 0), Complex::new(0, 0)];
    let expected = vec![false, false, true, false, false, true, false, false];

    assert_eq!(rising_edge_detector(test.as_slice()), expected);
}

#[test]
pub fn test_falling_edge_detection() {
    let test = vec![Complex::new(0, 0), Complex::new(0, 0), Complex::new(1, 0), Complex::new(0, 0), Complex::new(0, 0), Complex::new(1, 0), Complex::new(0, 0), Complex::new(0, 0)];
    let expected = vec![false, false, false, true, false, false, true, false];

    assert_eq!(falling_edge_detector(test.as_slice()), expected);
}