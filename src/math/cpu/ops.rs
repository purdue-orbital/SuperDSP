use std::sync::{Arc, Mutex, RwLock};

pub struct Data{
    pub(crate) f32_arrays: Vec<Arc<Mutex<Vec<f32>>>>,
    pub(crate) f32_const: Vec<Arc<RwLock<f32>>>
}

pub trait CPUOperation{
    fn run(&mut self, data: &mut Data);
}


pub struct ElementwiseMultiplyF32;
impl CPUOperation for ElementwiseMultiplyF32{
    fn run(&mut self, data: &mut Data) {

        let mb: &mut Vec<&mut [f32]> = data.f32_arrays.as_mut();

        // run
        for index in 0..mb.first().unwrap().len(){

            let val2 = mb.get(1).unwrap().get(index).unwrap();
            let val1 = mb.first().unwrap().get(index).unwrap();

            mb[1][index] = *val1 * *val2;
        }
    }
}

pub struct ConvolutionF32;
impl CPUOperation for ConvolutionF32 {
    fn run(&mut self, data: &mut Data) {

        let mb: &mut Vec<&mut [f32]> = data.f32_arrays.as_mut();

        // run
        for i in 0..mb.first().unwrap().len(){
            for j in 0..mb.get(1).unwrap().len(){
                mb[2][i+j] += mb.first().unwrap().get(i).unwrap() * mb.get(1).unwrap().get(j).unwrap();
            }
        }
    }
}

pub struct ScalarMultiplyF32;
impl CPUOperation for ScalarMultiplyF32 {
    fn run(&mut self, data: &mut Data) {

        let mb: &mut Vec<&mut [f32]> = data.f32_arrays.as_mut();
        let scalar: f32 = *data.f32_const[0];

        let _ = mb[0].iter_mut().map(|val| *val *= scalar);
    }
}