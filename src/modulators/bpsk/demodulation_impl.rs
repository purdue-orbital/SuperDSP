use std::f32::consts::PI;
use num_complex::Complex;
use crate::common::constellation::{Constellation, ConstellationPoint};

use crate::modulators::bpsk::structs::demodulation::Demodulation;

impl Demodulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_signal: f32) -> Demodulation {

        let mut constellation = Constellation::new(message_signal,sample_rate,samples_per_symbol);

        let bin_zero = ConstellationPoint::new(0, 0.0, 0.0, 1.0,0.0);
        let bin_one = ConstellationPoint::new(1, PI, 0.0, 1.0,0.0);

        constellation.add_state(&bin_zero);
        constellation.add_state(&bin_one);

        Demodulation { samples_per_symbol, sample_rate, constellation}
    }

    /// Demodulate a radio signal using BPSK
    ///
    /// # Arguments
    /// * `arr` - Array of radio samples to
    pub fn run(&self, arr: Vec<Complex<f32>>) -> Vec<u8>
    {
        let mut to_return = vec![];

        let out = self.constellation.evaluate(arr.as_slice());

        let mut bin = 0_u8;
        let mut counter = 0;

        for x in out{
            // BPSK is always 1 bit
            bin <<= 1;

            // Add bin
            bin += x as u8;
            counter += 1;

            // at end of bin construction, add it to array
            if counter == 8 {
                to_return.push(bin);

                counter = 0;
                bin = 0;
            }
        }

        if counter != 0{
            to_return.push(bin);
        }

        to_return
    }
}
