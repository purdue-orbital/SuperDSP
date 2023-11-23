//! low pass filters (LPFs) are a critical component in filtering and used in the construction of
//! larger, more powerful components. Implementation based off https://www.dsp-weimich.com/digital-signal-processing/iir-first-order-digital-filter/

use std::f32::consts::PI;

pub struct LPF{
    x_n: f32,
    prev_y_n: f32,
    a: f32
}


impl LPF{

    pub fn run(&mut self, sample: f32) -> f32{
        let y_n = self.prev_y_n + self.a * (sample - self.prev_y_n);

        self.prev_y_n = y_n;

        y_n
    }

    pub fn new(sample_rate:f32, passband:f32) -> LPF{
        // calculate coefficient
        let rc = 1.0 / (2.0 * PI * passband);
        let ts = 1.0 / sample_rate;
        let a = ts / (ts + rc);

        LPF{
            x_n: 0.0,
            prev_y_n: 0.0,
            a
        }
    }
}