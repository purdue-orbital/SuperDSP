use nalgebra::{SMatrix, SVector};
use num::Complex;

use crate::objects::object::{Bus, DSPObject};

fn hermitian_transpose<const N: usize>(vector: SMatrix<Complex<f32>, N,1>) -> SMatrix<Complex<f32>,1,N> {
    let mut out = SMatrix::<Complex<f32>,1,N>::zeros();
    
    for i in 0..N {
        out[i] = vector[i].conj();
    }
    
    out
}

#[derive(Clone, Copy)]
pub struct Autocorrelation<const N: usize> {
    correlation_matrix: SMatrix<Complex<f32>, N, N>
}

impl<const N: usize> Autocorrelation<N> {
    pub fn new(target_wave: SMatrix<Complex<f32>,N,1>) -> Autocorrelation<N> {
        let correlation_matrix = target_wave * hermitian_transpose(target_wave);
        
        Autocorrelation {
            correlation_matrix
        }
    }
}

impl<const N: usize> DSPObject for Autocorrelation<N> {
    fn return_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::F32
    }
    fn input_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::NONE
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        panic!("Autocorrelation does not listen on a bus");
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        panic!("WaveStepGen does not listen on a bus");
    }

    fn process(&mut self) {
        
    }

    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

