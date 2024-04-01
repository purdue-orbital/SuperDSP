use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct AGC {
    rate: f32,
    gain: f32,
    reference: f32,
    max_gain: f32,
}

impl Element for AGC {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        let mut input = _samples.get_complex_f32();
        
        let mut arr = input.to_vec();
        
        for x in arr.iter_mut(){
            *x *= self.gain;
            
            let power = (x.re * x.re + x.im * x.im).sqrt();
            
            // calculate gain
            self.gain += (self.reference - power) * self.rate;
            self.gain = self.gain.max(0.0).min(self.max_gain);
        }
        
        input.get_real_array_wrapped().set_f32_array(arr.iter().map(|x| x.re).collect::<Vec<f32>>().as_slice());
        input.get_imag_array_wrapped().set_f32_array(arr.iter().map(|x| x.im).collect::<Vec<f32>>().as_slice());
    }

    fn halt(&self) -> bool {
        true
    }
    fn stop(&self, samples: &mut ElementParameter) -> bool { false }

    fn is_source(&self) -> bool {
        false
    }
}

impl AGC {
    pub fn new(rate: f32, reference: f32, gain: f32, max_gain: f32) -> AGC {
        AGC {
            rate,
            gain,
            reference,
            max_gain,
        }
    }
}