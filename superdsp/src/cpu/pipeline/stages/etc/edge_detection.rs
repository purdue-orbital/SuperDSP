use std::ops::{Add, Sub};
use num::Zero;
use num_complex::Complex;
pub fn rising_edge_detector<T: Sub<Output=T> + Add<Output=T> + Zero + Copy + PartialOrd>(input: &[Complex<T>]) -> Vec<bool> {
    let mut output = Vec::new();
    let mut last = *input.first().unwrap_or(&Complex::new(T::zero(), T::zero()));
    for &sample in input {
        if (sample.re + sample.im) - (last.re + last.im) > T::zero() {
            output.push(true);
        } else {
            output.push(false);
        }
        last = sample;
    }
    output
}

pub fn falling_edge_detector<T: Sub<Output=T> + Add<Output=T> + Zero + Copy + PartialOrd>(input: &[Complex<T>]) -> Vec<bool> {
    let mut output = Vec::new();
    let mut last = *input.first().unwrap_or(&Complex::new(T::zero(), T::zero()));
    for &sample in input {
        if  (last.re + last.im) - (sample.re + sample.im) > T::zero() {
            output.push(true);
        } else {
            output.push(false);
        }
        last = sample;
    }
    output
}


