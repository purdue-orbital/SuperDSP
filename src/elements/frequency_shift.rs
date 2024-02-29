use num_complex::Complex;

use crate::elements::element::Element;
use crate::elements::prefabs::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct FrequencyShift {
    sps: usize,
    sample_rate: f32,
    frequency: f32,
}

impl Element for FrequencyShift {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // create wave generator
        let arr = wave_generator_complex_time_banked(builder, self.sample_rate, self.frequency, self.sps);
        let scratch = ComplexF32::new(vec![Complex::new(0.0, 0.0); self.sps]);

        // get two signals seperated
        let src_i = arr.get_real_array_wrapped();
        let src_q = arr.get_imag_array_wrapped();

        let scratch_i = scratch.get_real_array_wrapped();
        let scratch_q = scratch.get_imag_array_wrapped();

        let samples_i = samples.get_complex_f32().get_real_array_wrapped();
        let samples_q = samples.get_complex_f32().get_imag_array_wrapped();

        // this is used to flip signs for multiplying to imaginary components
        let flip = ElementParameter::new_f32(-1.0);

        // copy to scratch
        builder.copy_f32(&samples_i, &scratch_i);
        builder.copy_f32(&samples_q, &scratch_q);

        // distribute and multiply with real part
        builder.pointwise_multiply_f32(&src_i, &samples_i);
        builder.pointwise_multiply_f32(&src_i, &samples_q); // samples_q is imaginary

        // distribute and multiply with imaginary part
        builder.pointwise_multiply_f32(&src_q, &scratch_i); // scratch_i is imaginary
        builder.pointwise_multiply_f32(&src_q, &scratch_q); // scratch_q is no longer imaginary

        // i * i = -1 so fix
        builder.scalar_multiply_f32(&scratch_q, &flip);

        // add seperated parts together (Euler's equation)
        builder.add_f32(&scratch_q, &samples_i);
        builder.add_f32(&scratch_i, &samples_q);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        true
    }
}

impl FrequencyShift {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> FrequencyShift {
        FrequencyShift {
            sps,
            sample_rate,
            frequency,
        }
    }
}