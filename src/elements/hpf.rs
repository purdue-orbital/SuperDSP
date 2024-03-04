use crate::elements::element::Element;
use crate::elements::prefabs::fir::fir_hpf_dft;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct HPF {
    roll_off: f32,
    sample_rate: f32,
    cutoff_frequency: f32,
}

impl Element for HPF {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        fir_hpf_dft(builder, &samples.get_complex_f32(), self.sample_rate, self.cutoff_frequency, self.roll_off, 1.0)
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

impl HPF {
    pub fn new(cutoff_frequency: f32, sample_rate: f32, roll_off: f32) -> HPF {
        HPF {
            roll_off,
            sample_rate,
            cutoff_frequency,
        }
    }
}