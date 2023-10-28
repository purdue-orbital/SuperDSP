use std::f32::consts::PI;

use num_complex::Complex;

use crate::common::constellation::{Constellation, ConstellationPoint};
use crate::common::convolution;
use crate::common::generate_wave::generate_wave;
use crate::modulators::bpsk::structs::demodulation::Demodulation;

impl Demodulation {

    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_signal: f32) -> Demodulation {

        Demodulation { samples_per_symbol, sample_rate, message_frequency:message_signal }
    }

    /// Demodulate a radio signal using BPSK
    ///
    /// # Arguments
    /// * `arr` - Array of radio samples to
    pub fn run(&self, arr: &[Complex<f32>]) -> Vec<u8>
    {
        let mut to_return = vec![];

        let mut carrier_signal =  generate_wave(
            self.message_frequency,
            self.sample_rate,
            arr.len() as i32,
            0,
            1.0,
            1.0,
            0.0,
            0.0,
        );

        for index in 0..arr.len(){
            carrier_signal[index].re *= arr[index].re;
            carrier_signal[index].im *= arr[index].im;
        }

        let convoluted = convolution(carrier_signal.as_slice(), vec![Complex::new(1.0,1.0);self.samples_per_symbol].as_slice());

        let mut bin = 0_u8;
        let mut counter = 0;

        for x in convoluted.iter().skip(self.samples_per_symbol-1).step_by(self.samples_per_symbol) {

            let bit = if x.re.is_sign_positive(){
                1
            }else {
                0
            };

            // BPSK is always 1 bit
            bin <<= 1;

            // Add bin
            bin ^= bit;
            counter += 1;

            // at end of bin construction, add it to array
            if counter == 8 {
                to_return.push(bin);

                counter = 0;
                bin = 0;
            }
        }

        if counter != 0 {
            to_return.push(bin);
        }

        to_return
    }
}
