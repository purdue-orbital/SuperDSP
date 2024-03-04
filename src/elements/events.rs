use crate::math::objects::ElementParameter;

pub trait Event: Send {
    /// the run function is called at runtime and returns true or false for the pipeline to 
    /// stop (On true, the pipeline will stop making this the last element in the pipe)
    fn run(&mut self, samples: &mut ElementParameter) -> bool;
}

pub trait EventClone {
    fn clone_box(&self) -> Box<dyn Event>;
}

impl<T> EventClone for T
    where
        T: 'static + Event + Clone,
{
    fn clone_box(&self) -> Box<dyn Event> {
        Box::new(self.clone())
    }
}

/// Very basic debug Event. Will print all values set of Element Parameter to terminal
pub struct Debug {}

impl Debug {
    pub fn new() -> Debug {
        Debug{

        }
    }
}
impl Event for Debug {
    fn run(&mut self, samples: &mut ElementParameter) -> bool {
        dbg!(samples);
        
        false
    }
}