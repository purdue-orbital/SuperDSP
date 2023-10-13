use std::f32::consts::PI;

use num_complex::Complex;

use crate::common::constellation::{Constellation, ConstellationPoint};
use crate::modulators::qpsk::structs::demodulation::Demodulation;

impl Demodulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_frequency: f32) -> Demodulation {
        let mut constellation = Constellation::new(message_frequency, sample_rate, samples_per_symbol);

        // create points
        let bin_zero = ConstellationPoint::new(0, PI, PI, 1.0, 1.0);
        let bin_one = ConstellationPoint::new(1, PI, 0.0, 1.0, 1.0);
        let bin_two = ConstellationPoint::new(2, 0.0, PI, 1.0, 1.0);
        let bin_three = ConstellationPoint::new(3, 0.0, 0.0, 1.0, 1.0);

        // add points
        constellation.add_state(&bin_zero);
        constellation.add_state(&bin_one);
        constellation.add_state(&bin_two);
        constellation.add_state(&bin_three);

        Demodulation { samples_per_symbol, sample_rate, constellation }
    }

    /// Demodulate a radio signal using QPSK
    ///
    /// # Arguments
    /// * `arr` - Array of radio samples to
    pub fn run(&self, arr: &[Complex<f32>]) -> Vec<u8>
    {
        let mut to_return = vec![];

        let out = self.constellation.evaluate(arr);

        let mut bin = 0_u8;
        let mut counter = 0;

        for x in out {
            // QPSK is always 2 bits
            bin <<= 2;

            // Add bin
            bin ^= x as u8;
            counter += 2;

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
