use crate::elements::element::Element;
use crate::elements::macros::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalAdder {
    sps: usize,
    sample_rate: f32,
    frequency: f32,
}

impl Element for SignalAdder {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // create wave generator
        let arr = wave_generator_complex_time_banked(builder, self.sample_rate, self.frequency, self.sps);

        let half = ElementParameter::new_f32(0.5);

        // add two signals together
        let src_i = arr.get_real_array_wrapped();
        let src_q = arr.get_imag_array_wrapped();

        builder.add_f32(&src_i,&samples.get_complex_f32().get_real_array_wrapped());
        builder.add_f32(&src_q,&samples.get_complex_f32().get_imag_array_wrapped());

        // divide by 2 so the highest point is 1
        // builder.scalar_multiply_f32(&samples.get_complex_f32().get_real_array_wrapped(), &half);
        // builder.scalar_multiply_f32(&samples.get_complex_f32().get_imag_array_wrapped(), &half);
    }

    fn run(&mut self, _samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        true
    }
}

impl SignalAdder {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> SignalAdder {

        SignalAdder {
            sps,
            sample_rate,
            frequency,
        }
    }
}