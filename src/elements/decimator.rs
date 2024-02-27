use crate::elements::element::Element;
use crate::elements::macros::resampling::decimate;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct Decimator {
    downsample_amount: usize,
    original_sample_rate: f32,
    roll_off: f32,
}

impl Element for Decimator {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        let copy = &mut samples.get_complex_f32();

        decimate(builder, copy, self.downsample_amount, self.original_sample_rate, self.roll_off);

        samples.set_complex_f32(copy.clone());
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl Decimator {
    pub fn new(downsample_amount: usize, original_sample_rate: f32, roll_off: f32) -> Decimator {
        Decimator {
            downsample_amount,
            original_sample_rate,
            roll_off,
        }
    }
}