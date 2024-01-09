mod glsl;

use std::hash::Hash;
use std::sync::Arc;
use eframe::egui::ahash::{HashMap, RandomState};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::{sync, VulkanLibrary};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::shader::ShaderModule;
use vulkano::sync::future::{FenceSignalFuture, NowFuture};
use vulkano::sync::GpuFuture;

pub struct Vulkan{
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,

    compute_shaders: HashMap<String,Arc<ShaderModule>>,

    queue_family_index: u32
}

impl Default for Vulkan{
    fn default() -> Self {
        // all this code come from the vulkano tutorial so it should be setup properly

        // Generate vulkan instance
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

        // Get the first vulkan device
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family") as u32;

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
            .expect("failed to create device");

        let queue = queues.next().unwrap();
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        // compile compute shaders
        let pw = glsl::compute_shaders::pointwise_multiplication_f32::load(device.clone()).expect("Failed to compile compute shaders!");

        // create hash map
        let mut compute_shaders = HashMap::default();

        compute_shaders.insert("pw mul".to_string(), pw);

        // We save these variables so we can execute operations on them later
        Vulkan{
            device,
            queue,
            memory_allocator,
            compute_shaders,
            queue_family_index
        }
    }
}

impl Vulkan {
    /// this will take in an array and store it in vram
    pub fn store_to_vram<T: Copy + bytemuck::Pod + Send + Sync>(&self, data: &[T]) -> Subbuffer<[T]>{
        Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            data.iter().copied(),
        ).expect("failed to create buffer")
    }

    pub fn run_command_builder(&self, buffer: Arc<PrimaryAutoCommandBuffer>) -> FenceSignalFuture<CommandBufferExecFuture<NowFuture>> {
        sync::now(self.device.clone())
            .then_execute(self.queue.clone(), buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
    }

    pub fn create_command_builder(&self) -> VulkanCommandBuilder{
        VulkanCommandBuilder::new(self.device.clone(), self.queue_family_index, self.compute_shaders.clone())
    }
}

/// This is a wrapper struct around the Vulkan CommandBuffer used to make vulkan compute pipelines
pub struct VulkanCommandBuilder{
    command_buffer_allocator: StandardCommandBufferAllocator,
    builder:  Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    compute_shaders: HashMap<String,Arc<ShaderModule>>,
    device: Arc<Device>
}

impl VulkanCommandBuilder{
    fn new(device: Arc<Device>, queue_family_index: u32, compute_shaders: HashMap<String, Arc<ShaderModule>>) -> Self {
        let command_buffer_allocator = StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default()
        );

        let builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue_family_index,
            CommandBufferUsage::MultipleSubmit,
        ).unwrap();

        VulkanCommandBuilder{
            command_buffer_allocator,
            builder: Some(builder),
            compute_shaders,
            device: device.clone()
        }
    }

    fn stage_pipeline(&self, shader: &str) -> Arc<ComputePipeline>{
        let cs = self.compute_shaders[shader].entry_point("main").unwrap();
        let stage = PipelineShaderStageCreateInfo::new(cs);
        let layout = PipelineLayout::new(
            self.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                .into_pipeline_layout_create_info(self.device.clone())
                .unwrap(),
        ).unwrap();

        ComputePipeline::new(
            self.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage, layout),
        ).expect("failed to create compute pipeline")
    }

    fn set_layout<T: Copy + bytemuck::Pod + Send + Sync>(&self, pipeline: Arc<ComputePipeline>, set_index:usize, binding_index: u32, source: Subbuffer<[T]>) -> Arc<PersistentDescriptorSet>{
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(self.device.clone(), Default::default());
        let pipeline_layout = pipeline.layout();
        let descriptor_set_layouts = pipeline_layout.set_layouts();

        let descriptor_set_layout = descriptor_set_layouts
            .get(set_index)
            .unwrap();

        PersistentDescriptorSet::new(
            &descriptor_set_allocator,
            descriptor_set_layout.clone(),
            [WriteDescriptorSet::buffer(binding_index, source.clone())],
            [],
        ).unwrap()
    }

    /// This is a simple, elementwise multiplication (dest\[n\] =  src\[n\] * dest\[n\])
    pub fn elementwise_multiply_f32(&mut self, source:  Subbuffer<[f32]>, destination: Subbuffer<[f32]>){

        let pipeline = self.stage_pipeline("pw mul");
        let descriptor_set_source = self.set_layout(pipeline.clone(),0,0,source);
        let descriptor_set_destination = self.set_layout(pipeline.clone(),1,1,destination);

        let work_group_counts = [1024, 1, 1];

         self.builder.as_mut().unwrap()
             .bind_pipeline_compute(pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                descriptor_set_source
            )
            .unwrap()
             .bind_descriptor_sets(
                 PipelineBindPoint::Compute,
                 pipeline.layout().clone(),
                 1,
                 descriptor_set_destination
             )
             .unwrap()
            .dispatch(work_group_counts)
            .unwrap();
    }

    pub fn build(&mut self) -> Arc<PrimaryAutoCommandBuffer>{
        self.builder.take().unwrap().build().unwrap()
    }
}