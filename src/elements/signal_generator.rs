use std::f32::consts::PI;

use num_complex::Complex;

use crate::elements::element::Element;
use crate::math::builder::WorkflowBuilder;
use crate::math::ElementParameter;


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
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder) {

    }

    fn run(&mut self, samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
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