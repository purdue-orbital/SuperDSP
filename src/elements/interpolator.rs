use crate::elements::element::Element;
use crate::elements::macros::resampling::interpolate;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct Interpolator {
    upsample_amount:usize, 
    original_sample_rate:f32, 
    lpf_frequency:f32, 
    roll_off:f32
}

impl Element for Interpolator {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        let mut copy = &mut samples.get_complex_f32();

        interpolate(builder, &mut copy, self.upsample_amount, self.original_sample_rate, self.lpf_frequency, self.roll_off);

        samples.set_complex_f32(copy.clone());
        
        dbg!(copy.get_real_array_wrapped().get_f32_array().len());
    }

    fn run(&mut self, _samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl Interpolator {
    pub fn new(upsample_amount:usize, original_sample_rate:f32, lpf_frequency:f32, roll_off:f32) -> Interpolator {
        Interpolator {
            upsample_amount,
            original_sample_rate,
            lpf_frequency,
            roll_off
        }
    }
}