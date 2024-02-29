use crate::elements::element::Element;
use crate::elements::prefabs::wave_generators::wave_generator_complex_time_banked;
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
        assert_eq!(samples.get_f32_array().len(),1);
        
        // create new samples output
        let output = wave_generator_complex_time_banked(builder, self.sample_rate, self.carrier_frequency, self.sps);
        
        let indexes = &ElementParameter::new_f32_array(vec![0.0; self.sps].as_slice());
        let array = &ElementParameter::new_f32_array(vec![0.0; self.sps].as_slice());
        
        builder.fetch_f32(samples,indexes,array);

        builder.pointwise_multiply_f32(array, &output.get_real_array_wrapped());
        builder.pointwise_multiply_f32(array, &output.get_imag_array_wrapped());

        samples.set_complex_f32(output);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {
    }

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