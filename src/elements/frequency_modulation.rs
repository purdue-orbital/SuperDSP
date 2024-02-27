use crate::elements::element::Element;
use crate::elements::macros::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct FrequencyModulation {
    sps: usize,
    carrier_frequency: f32,
    sample_rate: f32,
}

impl Element for FrequencyModulation {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // create new samples output
        let output = wave_generator_complex_time_banked(builder, self.sample_rate, self.carrier_frequency, self.sps);

        builder.scalar_multiply_f32(&output.get_real_array_wrapped(), samples);
        builder.scalar_multiply_f32(&output.get_imag_array_wrapped(), samples);

        samples.set_complex_f32(output);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl FrequencyModulation {
    pub fn new(sps: usize, carrier_frequency: f32, sample_rate: f32) -> FrequencyModulation {
        FrequencyModulation {
            sps,
            carrier_frequency,
            sample_rate,
        }
    }
}