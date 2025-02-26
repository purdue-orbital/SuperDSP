use crate::gpu::pipeline::stages::{Data, DataKind, FirstStage, Stage};
use crate::gpu::pipeline::{PipelineSettings, SubBuffer};
use crate::prelude::etc::{create_standard_buffer, create_standard_buffer_array, add_to_pipeline};
use bytemuck::{Pod, Zeroable};
use std::f32::consts::PI;
use std::fmt::Debug;
use vulkano::buffer::Subbuffer;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::half;
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use crate::gpu::pipeline::SubBuffer::{F16, F32};
use crate::prelude::shaders::wave_gen;

#[derive(Debug, Clone)]
pub struct WaveGen<I> {
    phantom: std::marker::PhantomData<I>,
    buf: SubBuffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct VulkanWaveGen {
    phi: f32,
    phi_offset: f32,
    time: f32,

    sample_rate: u32,
    taps: u32,
}

impl<I: Clone> Default for WaveGen<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Clone> WaveGen<I> {
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
            buf: SubBuffer::Empty,
        }
    }
}

impl<I: Clone + Debug + Send + Sync, O> Stage<I, O> for WaveGen<I> {
    fn configure(&mut self, data: &mut PipelineSettings) {
        // make sure the required fields are present
        assert_eq!(data.prev_stage_output, SubBuffer::Empty);

        let device = data.device.clone().expect("Expected device");
        let memory_allocator = data
            .memory_allocator
            .clone()
            .expect("Expected memory allocator");

        // create wave gen settings
        let wave_gen = VulkanWaveGen {
            phi: 2.0 * PI * data.frequency.expect("frequency not set") as f32,
            taps: data.num_taps.expect("num_taps not set") as u32,
            time: 1f32 / data.sample_rate.expect("sample_rate not set") as f32,
            phi_offset: 0.0,
            sample_rate: data.sample_rate.expect("sample_rate not set") as u32,
        };

        // create buffer for wave gen taps
        let taps_buffer = create_standard_buffer_array(
            memory_allocator.clone(),
            vec![half::f16::default(); data.num_taps.expect("num_taps not set")],
        );

        // create buffer for wave gen settings
        let settings_buffer = create_standard_buffer(memory_allocator.clone(), wave_gen);

        // load the shader
        let shader = wave_gen::load(device.clone()).expect("failed to load shader");

        self.buf = F16(taps_buffer.clone());

        add_to_pipeline(
            shader,
            data,
            vec![
                WriteDescriptorSet::buffer(0, settings_buffer),
                WriteDescriptorSet::buffer(1, taps_buffer.clone()),
            ],
            F16(taps_buffer),
            [data.num_taps.unwrap() as u32, 1, 1],
        );
    }

    fn process(&mut self, data: &Data, output: &mut Data) {}

    fn get_output_data_type(&self) -> DataKind {
        DataKind::F32
    }

    fn get_input_data_type(&self) -> DataKind {
        DataKind::Empty
    }
}

impl<I: Clone + Debug + Send + Sync, O> FirstStage<I, O> for WaveGen<I> {}
