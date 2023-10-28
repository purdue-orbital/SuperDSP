use std::f32::consts::PI;

use num_complex::Complex;

use crate::common::generate_wave::generate_wave;
use crate::encoders::nrz;
use crate::modulators::bpsk::structs::modulation::Modulation;

impl Modulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_signal: f32) -> Modulation {

        Modulation { samples_per_symbol, sample_rate, message_frequency:message_signal }
    }

    /// Modulate a radio signal using bpsk
    ///
    /// # Arguments
    /// * `bin` - String of binary bits (ONLY 1s & 0s) to modulate (AKA Symbols)
    pub fn run(&self, bin: &[u8]) -> Vec<Complex<f32>> {

        let signal = nrz(bin,self.samples_per_symbol);

        let mut carrier_signal =  generate_wave(
            self.message_frequency,
            self.sample_rate,
            signal.len() as i32,
            0,
            1.0,
            1.0,
            0.0,
            0.0,
        );

        // mix signals
        for index in 0..carrier_signal.len(){
            carrier_signal[index] *= signal[index];
        }

        carrier_signal
    }
}