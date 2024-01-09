use std::f32::consts::PI;

use num_complex::Complex;

use crate::components::loop_filter::LoopFilter;
use crate::components::lpf::LPF;
use crate::components::nco::NCO;
use crate::components::threshold;

pub struct Costas {
    nco: NCO,
    nco_offset: f32,
    i_lpf: LPF,
    q_lpf: LPF,
    loop_filter: LoopFilter,
}

impl Costas {
    pub fn run(&mut self, sample: &Complex<f32>) -> Complex<f32> {
        let next_sample = self.nco.run(self.nco_offset);

        // multiply sample with nco sample
        let i = sample.re * next_sample.re;
        let q = sample.im * next_sample.im;

        // apply lpf
        let i_lpf = self.i_lpf.run(i);
        let q_lpf = self.q_lpf.run(q);

        // apply threshold
        let i_threshold = threshold(i_lpf);
        let q_threshold = threshold(q_lpf);

        // cross values
        let i_cross = i_threshold * q_lpf;
        let q_cross = q_threshold * i_lpf;

        // phase detect
        let phase_diff = q_cross - i_cross;

        // run loop filter
        self.nco_offset = self.loop_filter.run(phase_diff);

        // return new sample
        Complex::new(i_threshold, q_threshold)
    }

    /// Creates a new costas loop instance
    pub fn new(sample_rate: f32, carrier_frequency: f32) -> Costas {
        Costas {
            nco: NCO::new(sample_rate, carrier_frequency),
            nco_offset: 0.0,
            i_lpf: LPF::new(sample_rate, carrier_frequency * 10.0),
            q_lpf: LPF::new(sample_rate, carrier_frequency * 10.0),
            loop_filter: LoopFilter::new(2.0 * PI / 100.0, sample_rate, 1.0),
        }
    }
}