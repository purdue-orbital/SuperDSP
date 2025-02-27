use rustdsp::prelude::etc::duplicator::{duplicator, duplicator_complex, duplicator_i8, duplicator_u8, unduplicator, unduplicator_complex, unduplicator_i8, unduplicator_u8};

#[test]
pub fn test_duplicator() {
    let test = vec![1.0, 2.0, 3.0];
    let expected = vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0];

    assert_eq!(duplicator(test.as_slice(), 2), expected);
}

#[test]
pub fn test_unduplicator(){
    let test = vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0];
    let expected = vec![1.0, 2.0, 3.0];

    assert_eq!(unduplicator(test.as_slice(), 2, 1), expected);
}

#[test]
pub fn test_duplicator_full_cycle() {
    let test = vec![1.0, 2.0, 3.0];

    let out = duplicator(test.as_slice(), 2);

    let out = unduplicator(out.as_slice(), 2, 0);

    assert_eq!(test, out);
}

#[test]
pub fn test_duplicator_i8() {
    let test: Vec<i8> = vec![1, 2, 3];
    let expected: Vec<i8> = vec![1, 1, 2, 2, 3, 3];

    assert_eq!(duplicator_i8(test.as_slice(), 2), expected);
}

#[test]
pub fn test_unduplicator_i8(){
    let test: Vec<i8> = vec![1, 1, 2, 2, 3, 3];
    let expected: Vec<i8> = vec![1, 2, 3];

    assert_eq!(unduplicator_i8(test.as_slice(), 2, 1), expected);
}

#[test]
pub fn test_duplicator_full_cycle_i8() {
    let test: Vec<i8> = vec![1, 2, 3];

    let out = duplicator_i8(test.as_slice(), 2);

    let out = unduplicator_i8(out.as_slice(), 2, 1);

    assert_eq!(test, out);
}

#[test]
pub fn test_duplicator_u8() {
    let test: Vec<u8> = vec![1, 2, 3];
    let expected: Vec<u8> = vec![1, 1, 2, 2, 3, 3];

    assert_eq!(duplicator_u8(test.as_slice(), 2), expected);
}

#[test]
pub fn test_unduplicator_u8(){
    let test: Vec<u8> = vec![1, 1, 2, 2, 3, 3];
    let expected: Vec<u8> = vec![1, 2, 3];

    assert_eq!(unduplicator_u8(test.as_slice(), 2, 1), expected);
}

#[test]
pub fn test_duplicator_full_cycle_u8() {
    let test: Vec<u8> = vec![1, 2, 3];

    let out = duplicator_u8(test.as_slice(), 2);

    let out = unduplicator_u8(out.as_slice(), 2, 0);

    assert_eq!(test, out);
}

#[test]
pub fn test_duplicator_complex() {
    use num_complex::Complex;

    let test = vec![Complex::new(1.0, 1.0), Complex::new(2.0, 2.0), Complex::new(3.0, 3.0)];
    let expected = vec![Complex::new(1.0, 1.0), Complex::new(1.0, 1.0), Complex::new(2.0, 2.0), Complex::new(2.0, 2.0), Complex::new(3.0, 3.0), Complex::new(3.0, 3.0)];

    assert_eq!(duplicator_complex(test.as_slice(), 2), expected);
}

#[test]
pub fn test_unduplicator_complex(){
    use num_complex::Complex;

    let test = vec![Complex::new(1.0, 1.0), Complex::new(1.0, 1.0), Complex::new(2.0, 2.0), Complex::new(2.0, 2.0), Complex::new(3.0, 3.0), Complex::new(3.0, 3.0)];
    let expected = vec![Complex::new(1.0, 1.0), Complex::new(2.0, 2.0), Complex::new(3.0, 3.0)];

    assert_eq!(unduplicator_complex(test.as_slice(), 2, 0), expected);
}

#[test]
pub fn test_duplicator_full_cycle_complex() {
    use num_complex::Complex;

    let test = vec![Complex::new(1.0, 1.0), Complex::new(2.0, 2.0), Complex::new(3.0, 3.0)];

    let out = duplicator_complex(test.as_slice(), 2);

    let out = unduplicator_complex(out.as_slice(), 2, 1);

    assert_eq!(test, out);
}
