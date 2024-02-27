use std::thread::sleep;
use std::time::Duration;

use crate::elements::element::Element;
use crate::elements::macros::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalGenerator {
    sps: usize,
    sample_rate: f32,
    frequency: f32,
    time_delay: f32,
    delay: bool,
}

impl Element for SignalGenerator {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // create wave generator
        let arr = wave_generator_complex_time_banked(builder, self.sample_rate, self.frequency, self.sps);

        // set output as the out of the wave generator
        samples.set_complex_f32(arr)
    }

    fn run(&mut self, _samples: &mut ElementParameter) {
        sleep(Duration::from_secs_f32(self.time_delay))
    }

    fn halt(&self) -> bool {
        self.delay
    }

    fn is_source(&self) -> bool {
        true
    }
}

impl SignalGenerator {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize, delay: bool) -> SignalGenerator {
        SignalGenerator {
            sps,
            sample_rate,
            frequency,
            time_delay: sps as f32 / sample_rate,
            delay,
        }
    }
}