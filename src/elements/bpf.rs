use crate::elements::element::Element;
use crate::elements::macros::fir::fir_bpf_dft;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct BPF {
    roll_off: f32,
    sample_rate: f32,
    lower_frequency_limit: f32,
    upper_frequency_limit: f32,
}

impl Element for BPF {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        fir_bpf_dft(builder, &samples.get_complex_f32(), self.sample_rate, self.roll_off, self.lower_frequency_limit, self.upper_frequency_limit);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl BPF {
    pub fn new(sample_rate: f32, roll_off: f32, lower_frequency_limit: f32, upper_frequency_limit: f32) -> BPF {
        BPF {
            roll_off,
            sample_rate,
            lower_frequency_limit,
            upper_frequency_limit,
        }
    }
}