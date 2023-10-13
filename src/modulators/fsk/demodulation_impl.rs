use num_complex::Complex;
use rand::distributions::uniform::SampleBorrow;

use crate::common::bi_signal_demodulation::bi_signal_demodulation;
use crate::common::goertzel_algorithm::GoertzelAlgorithm;
use crate::modulators::fsk::structs::demodulation::Demodulation;

impl Demodulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_signal_2: f32) -> Demodulation {
        Demodulation { samples_per_symbol, sample_rate, goertzel_algorithm_fsk: GoertzelAlgorithm::new(samples_per_symbol as f32, sample_rate, message_signal_2) }
    }


    /// Demodulate a radio signal using FSK
    ///
    /// # Arguments
    /// * `arr` - Array of radio samples to
    pub fn run(&self, arr: &[Complex<f32>]) -> Vec<u8>
    {
        bi_signal_demodulation(arr, &self.goertzel_algorithm_fsk, &(self.samples_per_symbol as f32 / 2.0), self.samples_per_symbol.borrow())
    }
}
