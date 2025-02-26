use std::sync::Arc;
use bytemuck::Pod;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::device::Device;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkano::shader::ShaderModule;
use crate::gpu::pipeline::{PipelineSettings, SubBuffer};
use crate::gpu::pipeline::SubBuffer::F64;

pub fn create_standard_buffer<I: Pod + Send + Sync>(standard_memory_allocator: Arc<StandardMemoryAllocator>, data: I) -> Subbuffer<I> {
    Buffer::from_data(
        standard_memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        data,
    ).expect("failed to create buffer")
}

pub fn create_standard_buffer_array<I: Pod + Send + Sync>(standard_memory_allocator: Arc<StandardMemoryAllocator>, data: Vec<I>) -> Subbuffer<[I]> {
    Buffer::from_iter(
        standard_memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        data,
    ).expect("failed to create buffer")
}

pub fn create_host_readable_buffer_array<I: Pod + Send + Sync>(standard_memory_allocator: Arc<StandardMemoryAllocator>, data: Vec<I>) -> Subbuffer<[I]> {
    Buffer::from_iter(
        standard_memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        data,
    ).expect("failed to create buffer")
}

pub fn create_host_writeable_buffer_array<I: Pod + Send + Sync>(standard_memory_allocator: Arc<StandardMemoryAllocator>, data: Vec<I>) -> Subbuffer<[I]> {
    Buffer::from_iter(
        standard_memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        data,
    ).expect("failed to create buffer")
}

pub fn add_to_pipeline(shader: Arc<ShaderModule>, settings: &mut PipelineSettings, buffers: Vec<WriteDescriptorSet>, output_buffer: SubBuffer, work_groups: [u32; 3]) {
    // create the pipeline
    let entry = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(entry);

    // add the stage to the pipeline
    settings.stages.push(stage);
    settings.pipeline_buffers.push(buffers);
    settings.prev_stage_output = output_buffer;
    settings.stages_work_groups.push(work_groups);
}