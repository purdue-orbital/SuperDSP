use std::f32::consts::PI;

use num_complex::Complex;

use crate::elements::element::Element;


#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalGenerator {
    phi: f32,

    step: f32,

    sps: usize,
}

impl Element for SignalGenerator {

    #[cfg(feature = "ui")]
    fn init(&mut self, win_builder: &mut WindowBuilder) {}

    fn run(&mut self, samples: &mut [Complex<f32>]) {
        for x in samples {
            x.re = self.phi.cos();
            x.im = self.phi.sin();

            self.phi += self.step;
            self.phi %= 2.0 * PI;
        }
    }
}

impl SignalGenerator {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> SignalGenerator {
        let step = 2.0 * PI * frequency * (1.0 / sample_rate);

        SignalGenerator {
            phi: 0.0,
            step,
            sps,
        }
    }
}