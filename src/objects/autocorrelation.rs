use std::dbg;
use nalgebra::{SMatrix};
use num::Complex;
use crate::math::correlation::create_autocorrelation_matrix;

use crate::objects::object::{Bus, DSPObject};

#[derive(Clone, Copy)]
pub struct Autocorrelation<const N: usize> {
    correlation_matrix: SMatrix<Complex<f32>, N, N>,
    
    buffer: SMatrix<Complex<f32>, N, 1>,
    
    input_bus: Option<Bus<'static>>,
    output_bus: Bus<'static>,
}

impl<const N: usize> Autocorrelation<N> {
    pub fn new(target_wave: SMatrix<Complex<f32>,N,1>) -> Autocorrelation<N> {
        
        Autocorrelation {
            correlation_matrix: create_autocorrelation_matrix(target_wave),
            
            buffer: SMatrix::<Complex<f32>,N,1>::zeros(),
            
            input_bus: None,
            output_bus: Bus::new_complex(),
        }
    }
}

impl<const N: usize> DSPObject for Autocorrelation<N> {
    fn return_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::Complex
    }
    fn input_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::NONE
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.output_bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        self.input_bus = Some(bus.clone());
    }

    fn process(&mut self) {
        let sample = *self.input_bus.unwrap().buffer_complex.unwrap().read();
        
        // save for output
        let out = self.buffer[N-1];
        
        // Shift buffer
        for i in 0..N-1 {
            self.buffer[i] = self.buffer[i+1];
        }
        // set new sample
        self.buffer[0] = sample;

        // preform correlation
        let autocorrelation = self.correlation_matrix * self.buffer;
        dbg!(autocorrelation);
        
        // set output
        self.output_bus.trigger_complex(out);
    }

    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

