mod ops;

use std::sync::{Arc, Mutex, RwLock};
use crate::math::cpu::ops::{ConvolutionF32, CPUOperation, Data, ElementwiseMultiplyF32, ScalarMultiplyF32};

struct BoxedCPUOperation{
    buffer: Data,
    operation: Box<dyn CPUOperation>
}

pub struct CPUPipeline{
    operations: Vec<BoxedCPUOperation>
}

impl CPUPipeline{
    pub fn run(&mut self){
        for x in self.operations.iter_mut(){
            x.operation.run(&mut x.buffer);
        }
    }
}

pub struct CPUCommandBuilder{
    operations: Option<Vec<BoxedCPUOperation>>
}

impl<'a> Default for CPUCommandBuilder{
    fn default() -> Self {
        CPUCommandBuilder{
            operations: Some(vec![]),
        }
    }
}

impl CPUCommandBuilder{

    pub fn build(&mut self) -> CPUPipeline{
        CPUPipeline{
            operations: self.operations.take().unwrap(),
        }
    }

    pub fn elementwise_multiply_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, destination: Arc<Mutex<Vec<f32>>>){
        // create data structure
        let data = Data{
            f32_arrays: vec![source,destination],
            f32_const: vec![]
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation{
            buffer: data,
            operation: Box::new(ElementwiseMultiplyF32),
        })

    }

    pub fn convolution_f32(&mut self, source1:  Arc<Mutex<Vec<f32>>>, source2: Arc<Mutex<Vec<f32>>>, dest: Arc<Mutex<Vec<f32>>>){
        // create data structure
        let data = Data{
            f32_arrays: vec![source1,source2,dest],
            f32_const: vec![]
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation{
            buffer: data,
            operation: Box::new(ConvolutionF32),
        });
    }

    pub fn scalar_multiply_f32(&mut self, source:  Arc<Mutex<Vec<f32>>>, scalar: Arc<RwLock<f32>>){
        // create data structure
        let data = Data{
            f32_arrays: vec![source],
            f32_const: vec![scalar]
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation{
            buffer: data,
            operation: Box::new(ScalarMultiplyF32),
        });
    }
}