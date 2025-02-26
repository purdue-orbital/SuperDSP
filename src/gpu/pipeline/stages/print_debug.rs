use std::fmt::Debug;
use vulkano::descriptor_set::WriteDescriptorSet;
use crate::gpu::pipeline::{get_f32_buffer, PipelineSettings, SubBuffer};
use crate::gpu::pipeline::SubBuffer::F32;
use crate::prelude::{Data, DataKind, LastStage, Stage};
use crate::prelude::etc::{add_to_pipeline, create_host_readable_buffer_array};
use crate::prelude::shaders::copy;

#[derive(Debug, Clone, Default)]
pub struct PrintDebug {
    target_buffer: SubBuffer,
}

impl<I: Clone + Debug + Send + Sync, O> Stage<I, O> for PrintDebug{
    fn configure(&mut self, data: &mut PipelineSettings) {
        assert_ne!(data.prev_stage_output, SubBuffer::Empty);
        
        let device = data.device.clone().expect("Expected device");
        let memory_allocator = data
            .memory_allocator
            .clone()
            .expect("Expected memory allocator");
        
        
        self.target_buffer = F32(create_host_readable_buffer_array(memory_allocator, vec![0f32; data.num_taps.expect("num_taps not set")]));
        
        add_to_pipeline(copy::load(device.clone()).expect("Failed to compile copy shader"), data, vec![WriteDescriptorSet::buffer(0, get_f32_buffer(data.prev_stage_output.clone())), WriteDescriptorSet::buffer(1, get_f32_buffer(self.target_buffer.clone()))], self.target_buffer.clone(), [data.num_taps.unwrap() as u32, 1, 1]);
    }

    fn process(&mut self, input: &Data, output: &mut Data) {
        let buf = get_f32_buffer(self.target_buffer.clone());
        let d = buf.read().expect("Failed to read buffer");
        
        println!("{:?}", d.iter().as_slice());
    }

    fn get_output_data_type(&self) -> DataKind {
        todo!()
    }

    fn get_input_data_type(&self) -> DataKind {
        todo!()
    }
}

impl<I: Clone + Send + Sync + Debug, O: Send + Sync> LastStage<I,O> for PrintDebug {}