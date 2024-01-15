use std::f32::consts::PI;

use num_complex::Complex;

use crate::elements::element::Element;
use crate::math::objects::ComplexF32;
use crate::math::prelude::*;


#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalGenerator {
    step: f32,
    sps: usize,
}

impl Element for SignalGenerator {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // make step into an element
        let step = ElementParameter::new_f32(self.sps as f32 * self.step);

        // this is used to mod phi to prevent explosion in float value
        let pi_2 = ElementParameter::new_f32(2.0 * PI);

        // make starting phi values
        let mut starting_values = Vec::new();
        for x in 0..self.sps{
            starting_values.push(x as f32 * self.step);
        }
        let phi = ElementParameter::new_f32_array(starting_values.as_slice());

        // create complex var
        let mut complex = ComplexF32::new(vec![Complex::new(0.0,0.0); self.sps]);

        // get phis
        let i_array = complex.get_real_array_wrapped();
        let q_array = complex.get_imag_array_wrapped();

        // copy phi into i and q array
        builder.copy_f32(&phi,&i_array);
        builder.copy_f32(&phi,&q_array);

        // preform cos and sin operations
        builder.cos_f32(&i_array);
        builder.sin_f32(&q_array);

        // increment phi
        builder.scalar_add_f32(&phi,&step);

        // preform mod
        builder.mod_f32(&phi,&pi_2);

        // set output for nex element
        samples.set_complex_f32(complex);
    }

    fn run(&mut self, samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }
    
    fn is_source(&self) -> bool {
        true
    }
}

impl SignalGenerator {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> SignalGenerator {
        let step = 2.0 * PI * frequency * (1.0 / sample_rate);

        SignalGenerator {
            step,
            sps,
        }
    }
}