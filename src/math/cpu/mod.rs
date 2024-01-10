mod ops;

use std::sync::{Arc, Mutex};
use crate::math::cpu::ops::{CPUOperation, Data, ElementwiseMultiplyF32};

struct BoxedCPUOperation<'a>{
    buffer: Data<'a>,
    operation: Arc<Mutex<Box<dyn CPUOperation>>>
}


pub struct CPUPipeline<'a>{
    operations: Vec<BoxedCPUOperation<'a>>
}

impl<'a> CPUPipeline<'a>{
    pub fn run(&mut self){
        for x in self.operations.iter_mut(){
            x.operation.lock().unwrap().run(&mut x.buffer);
        }
    }
}

pub struct CPUCommandBuilder<'a>{
    operations: Option<Vec<BoxedCPUOperation<'a>>>
}


impl<'a> Default for CPUCommandBuilder<'a>{
    fn default() -> Self {
        CPUCommandBuilder{
            operations: Some(vec![]),
        }
    }
}

impl<'a> CPUCommandBuilder<'a>{

    pub fn build(&mut self) -> CPUPipeline<'a>{
        CPUPipeline{
            operations: self.operations.take().unwrap(),
        }
    }

    pub fn elementwise_multiply_f32(&mut self, source:  &'a mut [f32], destination: &'a mut [f32]){
        // create data structure
        let data = Data{
            f32_arrays: vec![source,destination],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation{
            buffer: data,
            operation: Arc::new(Mutex::new(Box::new(ElementwiseMultiplyF32))),
        })

    }
}