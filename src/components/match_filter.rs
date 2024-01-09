use num_complex::Complex;

use crate::common::convolution;

pub struct MatchFilter {
    pulse_shape: Vec<Complex<f32>>,
    samples_per_a_symbol: usize,
}

impl MatchFilter {
    pub fn new(samples_per_a_symbol: usize, pulse_shape: &[Complex<f32>]) -> MatchFilter {
        assert_eq!(samples_per_a_symbol, pulse_shape.len());

        MatchFilter {
            pulse_shape: pulse_shape.to_vec(),
            samples_per_a_symbol,
        }
    }

    pub fn run(&self, samples: &[Complex<f32>]) -> Vec<Complex<f32>> {
        debug_assert_eq!(self.samples_per_a_symbol, samples.len());

        convolution(samples, self.pulse_shape.as_slice())
    }
}