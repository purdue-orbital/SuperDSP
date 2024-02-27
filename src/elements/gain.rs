use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct Gain {
    gain: f32,
}

impl Element for Gain {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // move gain to ram
        let gain = &ElementParameter::new_f32(self.gain);

        // multiply by gain
        builder.scalar_multiply_f32(&samples.get_complex_f32().get_real_array_wrapped(), gain);
        builder.scalar_multiply_f32(&samples.get_complex_f32().get_imag_array_wrapped(), gain)
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl Gain {
    pub fn new(gain: f32) -> Gain {
        Gain {
            gain
        }
    }
}