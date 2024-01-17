use crate::elements::element::Element;
use crate::elements::parts::nco::add_nco;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalGenerator {
    sps: usize,
    sample_rate: f32,
    frequency: f32,
}

impl Element for SignalGenerator {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // signal generator doesn't need frequency adjustment (in terms of error)
        let error_0 = ElementParameter::new_f32(0.0);

        // signal generator is a lot like a NCO so we reuse those components
        let arr = add_nco(builder,self.sps,self.sample_rate,self.frequency, &error_0);

        samples.set_complex_f32(arr);
    }

    fn run(&mut self, samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        true
    }
}

impl SignalGenerator {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> SignalGenerator {
        SignalGenerator {
            sps,
            sample_rate,
            frequency
        }
    }
}