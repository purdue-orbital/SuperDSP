use alloc::sync::Arc;
use spin::mutex::Mutex;
use alloc::vec::Vec;
use alloc::boxed::Box;

pub trait DSPObject: DSPObjectClonable + Send + Sync {
    fn set_input_buffer(&mut self, buffer: Arc<Mutex<f64>>);
    fn get_output_buffer(&self) -> Arc<Mutex<f64>>;
    fn set_input_buffer_vec(&mut self, buffer: Arc<Mutex<Vec<f64>>>);
    fn get_output_buffer_vec(&self) -> Arc<Mutex<Vec<f64>>>;

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
