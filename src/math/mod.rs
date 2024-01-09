pub mod vulkan;

use num_complex::Complex;
use crate::math::ValueTypes::{ComplexF32, U8};

enum ValueTypes{
    ComplexF32,
    U8,
}


/// This will dynamically be
pub struct ElementParameter {
    vtype: ValueTypes,

    complexf32: Option<Vec<Complex<f32>>>,
    u8: Option<Vec<u8>>
}

impl ElementParameter {
    pub fn new_complex_f32(arr: &[Complex<f32>]) -> ElementParameter {
        ElementParameter{
            vtype: ComplexF32,

            complexf32: Some(arr.to_vec()),
            u8: None
        }
    }

    pub fn new_u8(arr: &[u8]) -> ElementParameter{
        ElementParameter{
            vtype: U8,

            complexf32: None,
            u8: Some(arr.to_vec())
        }
    }

    pub fn get_complex_f32_array(&self) -> Vec<Complex<f32>>{
        self.complexf32.clone().unwrap()
    }
    pub fn get_u8_array(&self) -> Vec<u8>{
        self.u8.clone().unwrap()
    }

    pub fn set_complex_f32(&self) -> Vec<Complex<f32>>{
        self.complexf32.clone().unwrap()
    }

    pub fn set_u8(&self) -> Vec<u8>{
        self.u8.clone().unwrap()
    }
}