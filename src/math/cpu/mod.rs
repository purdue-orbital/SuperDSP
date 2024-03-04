use crate::math::cpu::ops::*;
use std::sync::{Arc, Mutex, RwLock};

mod ops;

struct BoxedCPUOperation {
    buffer: Data,
    operation: Box<dyn CPUOperation>,
}

unsafe impl Send for BoxedCPUOperation {}


pub struct CPUPipeline {
    operations: Vec<BoxedCPUOperation>,
}

impl CPUPipeline {
    pub fn run(&mut self) {
        for x in self.operations.iter_mut() {
            x.operation.run(&mut x.buffer);
        }
    }
}

pub struct CPUCommandBuilder {
    operations: Option<Vec<BoxedCPUOperation>>,
}

impl Default for CPUCommandBuilder {
    fn default() -> Self {
        CPUCommandBuilder {
            operations: Some(vec![]),
        }
    }
}

impl CPUCommandBuilder {
    pub fn build(&mut self) -> CPUPipeline {
        CPUPipeline {
            operations: self.operations.take().unwrap(),
        }
    }

    pub fn elementwise_multiply_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, destination: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source, destination],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(ElementwiseMultiplyF32),
        })
    }
    
    pub fn elementwise_divide_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, destination: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source, destination],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(ElementwiseDivideF32),
        })
    }

    pub fn convolution_f32(&mut self, source1: Arc<Mutex<Vec<f32>>>, source2: Arc<Mutex<Vec<f32>>>, dest: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source1, source2, dest],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(ConvolutionF32),
        });
    }

    pub fn scalar_multiply_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, scalar: Arc<RwLock<f32>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source],
            f32_const: vec![scalar],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(ScalarMultiplyF32),
        });
    }

    pub fn sin_f32(&mut self, source: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(SinF32),
        });
    }
    pub fn cos_f32(&mut self, source: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(CosF32),
        });
    }

    pub fn sqrt_f32(&mut self, source: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(SqrtF32),
        });
    }
    
    pub fn mod_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, scalar: Arc<RwLock<f32>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source],
            f32_const: vec![scalar],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(ModF32),
        });
    }

    pub fn add_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, dest: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source, dest],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(AddF32),
        });
    }

    pub fn scalar_add_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, scalar: Arc<RwLock<f32>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source],
            f32_const: vec![scalar],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(ScalarAddF32),
        });
    }
    pub fn copy_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, destination: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source, destination],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(CopyF32),
        })
    }

    pub fn fetch_f32(&mut self, source: Arc<Mutex<Vec<f32>>>, indexes: Arc<Mutex<Vec<f32>>>, destination: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![source, indexes, destination],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(FetchF32),
        })
    }

    pub fn dft_f32(&mut self, i_source: Arc<Mutex<Vec<f32>>>, q_source: Arc<Mutex<Vec<f32>>>, i_dest: Arc<Mutex<Vec<f32>>>, q_dest: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![i_source, q_source, i_dest, q_dest],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(DFTF32),
        })
    }

    pub fn idft_f32(&mut self, i_source: Arc<Mutex<Vec<f32>>>, q_source: Arc<Mutex<Vec<f32>>>, i_dest: Arc<Mutex<Vec<f32>>>, q_dest: Arc<Mutex<Vec<f32>>>) {
        // create data structure
        let data = Data {
            f32_arrays: vec![i_source, q_source, i_dest, q_dest],
            f32_const: vec![],
        };

        // add to builder
        self.operations.as_mut().unwrap().push(BoxedCPUOperation {
            buffer: data,
            operation: Box::new(IDFTF32),
        })
    }
}