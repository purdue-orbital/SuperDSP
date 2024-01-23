use std::f32::consts::PI;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone)]
pub struct Data {
    pub(crate) f32_arrays: Vec<Arc<Mutex<Vec<f32>>>>,
    pub(crate) f32_const: Vec<Arc<RwLock<f32>>>,
}

pub trait CPUOperation {
    fn run(&mut self, data: &mut Data);
}


pub struct ElementwiseMultiplyF32;

impl CPUOperation for ElementwiseMultiplyF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr2.iter_mut().enumerate() {
            *x *= arr1[index];
        }
    }
}

pub struct ConvolutionF32;

impl CPUOperation for ConvolutionF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let dest = binding.as_mut_slice();

        // run
        for i in 0..arr1.len() {
            for j in 0..arr2.len() {
                dest[i + j] += arr1[i] * arr2[j];
            }
        }
    }
}

pub struct ScalarMultiplyF32;

impl CPUOperation for ScalarMultiplyF32 {
    fn run(&mut self, data: &mut Data) {
        let scalar: f32 = *data.f32_const[0].read().unwrap();

        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x *= scalar;
        }
    }
}

pub struct SinF32;

impl CPUOperation for SinF32 {
    fn run(&mut self, data: &mut Data) {
        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x = x.sin();
        }
    }
}

pub struct CosF32;

impl CPUOperation for CosF32 {
    fn run(&mut self, data: &mut Data) {
        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x = x.cos();
        }
    }
}

pub struct ModF32;

impl CPUOperation for ModF32 {
    fn run(&mut self, data: &mut Data) {
        let scalar = *data.f32_const[0].read().unwrap();

        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x %= scalar;
        }
    }
}

pub struct AddF32;

impl CPUOperation for AddF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr2.iter_mut().enumerate() {
            *x += arr1[index];
        }
    }
}

pub struct ScalarAddF32;

impl CPUOperation for ScalarAddF32 {
    fn run(&mut self, data: &mut Data) {
        let scalar: f32 = *data.f32_const[0].read().unwrap();

        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x += scalar;
        }
    }
}

pub struct CopyF32;

impl CPUOperation for CopyF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr2.iter_mut().enumerate() {
            *x = arr1[index];
        }
    }
}

pub struct FetchF32;

impl CPUOperation for FetchF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let src = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let indexes = binding.as_mut_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let dest = binding.as_mut_slice();

        // run
        for (index, x) in dest.iter_mut().enumerate() {
            *x = src[indexes[index] as usize];
        }
    }
}

pub struct DFTF32;

impl CPUOperation for DFTF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let i_src = binding.as_slice();

        let binding = data.f32_arrays[1].lock().unwrap();
        let q_src = binding.as_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let i_dest = binding.as_mut_slice();

        let mut binding = data.f32_arrays[3].lock().unwrap();
        let q_dest = binding.as_mut_slice();

        let len = i_dest.len();

        // run
        for k in 0..len {
            let mut phi: f32 = 0.0;
            let scalar = -2.0 * PI * (k as f32 / len as f32);

            for n in 0..len {
                // Set i value
                i_dest[k] += i_src[n] * phi.cos() - q_src[n] * phi.sin();
                q_dest[k] += i_src[n] * phi.sin() + q_src[n] * phi.cos();

                phi = scalar * n as f32;
            }
        }
    }
}