use crate::elements::element::Element;
use crate::elements::prefabs::resampling::rational_resampler;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct RationalResampler {
    upsample_amount: usize,
    downsample_amount: usize,
    original_sample_rate: f32,
    roll_off: f32,
}

impl Element for RationalResampler {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        let copy = &mut samples.get_complex_f32();

        rational_resampler(builder, copy, self.original_sample_rate, self.upsample_amount, self.downsample_amount, self.roll_off);

        samples.set_complex_f32(copy.clone());
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }
    fn stop(&self, samples: &mut ElementParameter) -> bool { false }

    fn is_source(&self) -> bool {
        false
    }
}

impl RationalResampler {
    pub fn new(upsample_amount: usize, downsample_amount: usize, original_sample_rate: f32, roll_off: f32) -> RationalResampler {
        RationalResampler {
            upsample_amount,
            downsample_amount,
            original_sample_rate,
            roll_off,
        }
    }
}