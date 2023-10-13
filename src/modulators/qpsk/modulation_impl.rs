use std::f32::consts::PI;

use num_complex::Complex;

use crate::common::constellation::{Constellation, ConstellationPoint};
use crate::modulators::qpsk::structs::modulation::Modulation;

impl Modulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_frequency: f32) -> Modulation {
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

        Modulation {
            samples_per_symbol,
            sample_rate,
            constellation,
        }
    }

    /// Modulate a radio signal using qpsk
    ///
    /// # Arguments
    /// * `bin` - String of binary bits (ONLY 1s & 0s) to modulate (AKA Symbols)
    pub fn run(&self, bin: &[u8]) -> Vec<Complex<f32>> {

        // explode bin into bits
        let mut corrected = vec![];

        for &x in bin {
            for y in (0..4).rev() {
                // QPSK is going to encode two bits
                corrected.push(((x >> (y * 2)) & 3) as u128);
            }
        }

        // run
        self.constellation.generate(corrected.as_slice())
    }
}