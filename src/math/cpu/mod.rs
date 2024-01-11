mod ops;

use crate::math::cpu::ops::{ConvolutionF32, CPUOperation, Data, ElementwiseMultiplyF32};

struct BoxedCPUOperation<'a>{
    buffer: Data<'a>,
    operation: Box<dyn CPUOperation>
}

pub struct CPUPipeline<'a>{
    operations: Vec<BoxedCPUOperation<'a>>
}

impl<'a> CPUPipeline<'a>{
    pub fn run(&mut self){
        for x in self.operations.iter_mut(){
            x.operation.run(&mut x.buffer);
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
            operation: Box::new(ElementwiseMultiplyF32),
        })

    }

    pub fn convolution_f32(&mut self, source1:  &'a mut [f32], source2: &'a mut [f32], dest: &'a mut [f32]){
        // create data structure
        let data = Data{
            f32_arrays: vec![source1,source2,dest],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation{
            buffer: data,
            operation: Box::new(ConvolutionF32),
        });
    }
}