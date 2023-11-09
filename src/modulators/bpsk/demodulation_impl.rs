use num_complex::Complex;

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

        let mut peaks = Vec::with_capacity(convoluted.len()/self.samples_per_symbol);
        let mut to_return = Vec::with_capacity(peaks.len()/8);

        // find peaks
        for x in (0..convoluted.len()-self.samples_per_symbol).step_by(self.samples_per_symbol){
            let mut max = 0.0f32;

            for y in 0..self.samples_per_symbol{
                if convoluted[x+y].re.abs() > max.abs(){
                    max = convoluted[x+y].re;
                }
            }

            peaks.push(max);
        }

        let mut bin = 0_u8;
        let mut counter = 0;

        for x in peaks {

            let bit = if x.is_sign_positive(){
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
