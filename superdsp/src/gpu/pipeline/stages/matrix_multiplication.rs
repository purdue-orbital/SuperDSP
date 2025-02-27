use std::fmt::Debug;
use vulkano::half;
use superdsp_macros::LastStageCrate;
use crate::gpu::pipeline::{PipelineSettings, SubBuffer};
use crate::prelude::{Data, DataKind, Stage};

#[derive(Debug, Clone, Default, LastStageCrate)]
pub struct MatrixMultiplication<O: Default> {
    target_buffer: SubBuffer,
    phantom: std::marker::PhantomData<O>,
}

impl<O: std::marker::Send + std::marker::Sync + std::default::Default> Stage for MatrixMultiplication<O>{
    fn configure(&mut self, data: &mut PipelineSettings) {
        assert_ne!(data.prev_stage_output, SubBuffer::Empty);
        
        let mut test_matrix = vec![half::f16::default(); data.num_taps.unwrap() * data.num_taps.unwrap()];
        for (index, x) in test_matrix.iter_mut().enumerate() {
            if index % data.num_taps.unwrap() + 1 == 0{
                *x = half::f16::from_f32(1.0);
            }
        }
        for (index,y) in test_matrix.iter().enumerate() {
            if index % data.num_taps.unwrap() == 0{
                println!()
            }
            
            print!("{y }")
        }
        println!("{:?}", test_matrix);
    }

    fn process(&mut self, input: &Data, output: &mut Data) {
    }

    fn get_output_data_type(&self) -> DataKind {
        todo!()
    }

    fn get_input_data_type(&self) -> DataKind {
        todo!()
    }
}