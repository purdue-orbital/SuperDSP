use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use num::Complex;
use spin::mutex::Mutex;
pub enum Type {
    NONE,
    F64,
    Complex,
    Vec,
    ComplexVec,
}

pub trait DSPObject: DSPObjectClonable + Send + Sync {
    fn return_type(&self) -> Type;
    fn input_type(&self) -> Type;
    fn set_input_buffer(&mut self, buffer: Arc<Mutex<f64>>);
    fn get_output_buffer(&self) -> Arc<Mutex<f64>>;
    fn set_input_buffer_complex(&mut self, buffer: Arc<Mutex<Complex<f64>>>);
    fn get_output_buffer_complex(&self) -> Arc<Mutex<Complex<f64>>>;
    fn set_input_buffer_vec(&mut self, buffer: Arc<Mutex<Vec<f64>>>);
    fn get_output_buffer_vec(&self) -> Arc<Mutex<Vec<f64>>>;
    fn set_input_buffer_complex_vec(&mut self, buffer: Arc<Mutex<Vec<Complex<f64>>>>);
    fn get_output_buffer_complex_vec(&self) -> Arc<Mutex<Vec<Complex<f64>>>>;
    fn process(&mut self);
}

pub trait DSPObjectClonable {
    fn clone_box(&self) -> Box<dyn DSPObject>;
}

impl<T> DSPObjectClonable for T
where
    T: 'static + DSPObject + Clone,
{
    fn clone_box(&self) -> Box<dyn DSPObject> {
        Box::new(self.clone())
    }
}
