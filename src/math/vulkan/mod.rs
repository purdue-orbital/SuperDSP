use std::collections::HashMap;
use std::sync::Arc;

use vulkano::{sync, VulkanLibrary};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage, PrimaryAutoCommandBuffer};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::shader::ShaderModule;
use vulkano::sync::future::{FenceSignalFuture, NowFuture};
use vulkano::sync::GpuFuture;

mod glsl;

#[derive(Clone)]
pub struct Vulkan {
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,

    compute_shaders: HashMap<String, Arc<ShaderModule>>,

    queue_family_index: u32,
}

impl Default for Vulkan {
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
        let pw_mul = glsl::compute_shaders::pointwise_multiplication_f32::load(device.clone()).expect("Failed to compile point wise multiplication shaders!");
        let convolve = glsl::compute_shaders::convolution_f32::load(device.clone()).expect("Failed to compile convolution shaders!");
        let s_mul = glsl::compute_shaders::scalar_multiplication_f32::load(device.clone()).expect("Failed to compile scalar multiplication shaders!");
        let sin = glsl::compute_shaders::sin_f32::load(device.clone()).expect("Failed to compile sine shaders!");
        let cos = glsl::compute_shaders::cos_f32::load(device.clone()).expect("Failed to compile cosine shaders!");
        let mod_f32 = glsl::compute_shaders::mod_f32::load(device.clone()).expect("Failed to compile mod shaders!");
        let add_f32 = glsl::compute_shaders::add_f32::load(device.clone()).expect("Failed to compile add shaders!");
        let s_add_f32 = glsl::compute_shaders::scalar_add_f32::load(device.clone()).expect("Failed to compile scalar add shaders!");
        let copy = glsl::compute_shaders::copy_f32::load(device.clone()).expect("Failed to compile copy shaders!");
        let fetch_f32 = glsl::compute_shaders::fetch_f32::load(device.clone()).expect("Failed to compile fetch shaders!");
        let dft = glsl::compute_shaders::dft_f32::load(device.clone()).expect("Failed to compile dft shaders!");


        // create hash map
        let mut compute_shaders = HashMap::default();

        compute_shaders.insert("pw mul".to_string(), pw_mul);
        compute_shaders.insert("convolve".to_string(), convolve);
        compute_shaders.insert("s mul".to_string(), s_mul);
        compute_shaders.insert("sin".to_string(), sin);
        compute_shaders.insert("cos".to_string(), cos);
        compute_shaders.insert("mod".to_string(), mod_f32);
        compute_shaders.insert("add".to_string(), add_f32);
        compute_shaders.insert("s add".to_string(), s_add_f32);
        compute_shaders.insert("copy".to_string(), copy);
        compute_shaders.insert("fetch".to_string(), fetch_f32);
        compute_shaders.insert("dft".to_string(), dft);

        // We save these variables so we can execute operations on them later
        Vulkan {
            device,
            queue,
            memory_allocator,
            compute_shaders,
            queue_family_index,
        }
    }
}

impl Vulkan {
    /// this will take in an array and store it in vram
    pub fn store_to_vram_array<T: Copy + bytemuck::Pod + Send + Sync>(&self, data: &[T]) -> Subbuffer<[T]> {
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

    pub fn store_to_vram_var<T: Copy + bytemuck::Pod + Send + Sync>(&self, data: T) -> Subbuffer<T> {
        Buffer::from_data(
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
            data,
        ).expect("failed to create buffer")
    }

    pub fn run_command_builder(&self, buffer: Arc<PrimaryAutoCommandBuffer>) -> FenceSignalFuture<CommandBufferExecFuture<NowFuture>> {
        sync::now(self.device.clone())
            .then_execute(self.queue.clone(), buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
    }

    pub fn create_command_builder(&self) -> VulkanCommandBuilder {
        VulkanCommandBuilder::new(self.device.clone(), self.queue_family_index, self.compute_shaders.clone(), self.clone())
    }
}

/// This is a wrapper struct around the Vulkan CommandBuffer used to make vulkan compute pipelines
pub struct VulkanCommandBuilder {
    builder: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    compute_shaders: HashMap<String, Arc<ShaderModule>>,
    device: Arc<Device>,
    vulkan: Vulkan,
}

impl VulkanCommandBuilder {
    fn new(device: Arc<Device>, queue_family_index: u32, compute_shaders: HashMap<String, Arc<ShaderModule>>, vulkan: Vulkan) -> Self {
        let command_buffer_allocator = StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        );

        let builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue_family_index,
            CommandBufferUsage::MultipleSubmit,
        ).unwrap();

        VulkanCommandBuilder {
            builder: Some(builder),
            compute_shaders,
            device: device.clone(),
            vulkan,
        }
    }

    fn stage_pipeline(&self, shader: &str) -> Arc<ComputePipeline> {
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

    fn set_layout_array<T: Copy + bytemuck::Pod + Send + Sync>(&self, pipeline: Arc<ComputePipeline>, set_index: usize, binding_index: u32, source: Subbuffer<[T]>) -> Arc<PersistentDescriptorSet> {
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

    fn set_layout_var<T: Copy + bytemuck::Pod + Send + Sync>(&self, pipeline: Arc<ComputePipeline>, set_index: usize, binding_index: u32, source: Subbuffer<T>) -> Arc<PersistentDescriptorSet> {
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

    fn bind_descriptor_sets(&mut self, pipeline: Arc<ComputePipeline>, arr: &[Arc<PersistentDescriptorSet>], workgroups: [u32; 3]) {
        let mut buffer = self.builder.as_mut().unwrap().bind_pipeline_compute(pipeline.clone()).unwrap();

        for (index, x) in arr.iter().enumerate() {
            buffer = buffer.bind_descriptor_sets(
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                index as u32,
                x.clone(),
            )
                .unwrap();
        }

        buffer.dispatch(workgroups).unwrap();
    }

    /// This is a simple, elementwise multiplication (dest\[n\] =  src\[n\] * dest\[n\])
    pub fn elementwise_multiply_f32(&mut self, source: Subbuffer<[f32]>, destination: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("pw mul");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 0, 0, source.clone());
        let descriptor_set_destination = self.set_layout_array(pipeline.clone(), 1, 1, destination);

        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source, descriptor_set_destination];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn convolution_f32(&mut self, source1: Subbuffer<[f32]>, source2: Subbuffer<[f32]>) -> Subbuffer<[f32]> {
        // Create buffer that will be returned
        let size = source1.read().unwrap().len() + source2.read().unwrap().len() - 1;
        let dest = self.vulkan.store_to_vram_array(vec![0.0; size].as_slice());

        let pipeline = self.stage_pipeline("convolve");
        let descriptor_set_source1 = self.set_layout_array(pipeline.clone(), 0, 0, source1.clone());
        let descriptor_set_destination_source2 = self.set_layout_array(pipeline.clone(), 1, 1, source2.clone());
        let descriptor_set_destination_dest = self.set_layout_array(pipeline.clone(), 2, 2, dest.clone());


        let work_group_counts = [size as u32, 1, 1];
        let arr = [descriptor_set_source1, descriptor_set_destination_source2, descriptor_set_destination_dest];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);

        dest
    }

    pub fn scalar_multiply_f32(&mut self, source: Subbuffer<[f32]>, scalar: Subbuffer<f32>) {
        let pipeline = self.stage_pipeline("s mul");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 1, 1, source.clone());
        let descriptor_set_scalar = self.set_layout_var(pipeline.clone(), 0, 0, scalar.clone());


        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_scalar, descriptor_set_source];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn sin_f32(&mut self, source: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("sin");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 0, 0, source.clone());


        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn cos_f32(&mut self, source: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("cos");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 0, 0, source.clone());


        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn mod_f32(&mut self, source: Subbuffer<[f32]>, scalar: Subbuffer<f32>) {
        let pipeline = self.stage_pipeline("mod");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 1, 1, source.clone());
        let descriptor_set_scalar = self.set_layout_var(pipeline.clone(), 0, 0, scalar.clone());


        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_scalar, descriptor_set_source];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn add_f32(&mut self, source: Subbuffer<[f32]>, destination: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("add");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 0, 0, source.clone());
        let descriptor_set_destination = self.set_layout_array(pipeline.clone(), 1, 1, destination);

        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source, descriptor_set_destination];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn scalar_add_f32(&mut self, scalar: Subbuffer<f32>, destination: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("s add");
        let descriptor_set_source = self.set_layout_var(pipeline.clone(), 0, 0, scalar);
        let descriptor_set_destination = self.set_layout_array(pipeline.clone(), 1, 1, destination.clone());

        let work_group_counts = [destination.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source, descriptor_set_destination];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn copy_f32(&mut self, source: Subbuffer<[f32]>, destination: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("copy");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 0, 0, source.clone());
        let descriptor_set_destination = self.set_layout_array(pipeline.clone(), 1, 1, destination);

        let work_group_counts = [source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source, descriptor_set_destination];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn fetch_f32(&mut self, source: Subbuffer<[f32]>, indexes: Subbuffer<[f32]>, destination: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("fetch");
        let descriptor_set_source = self.set_layout_array(pipeline.clone(), 0, 0, source);
        let descriptor_set_indexes = self.set_layout_array(pipeline.clone(), 1, 1, indexes.clone());
        let descriptor_set_destination = self.set_layout_array(pipeline.clone(), 2, 2, destination);

        let work_group_counts = [indexes.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_source, descriptor_set_indexes, descriptor_set_destination];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn dft_f32(&mut self, i_source: Subbuffer<[f32]>, q_source: Subbuffer<[f32]>, i_dest: Subbuffer<[f32]>, q_dest: Subbuffer<[f32]>) {
        let pipeline = self.stage_pipeline("dft");
        let descriptor_set_i_source = self.set_layout_array(pipeline.clone(), 0, 0, i_source.clone());
        let descriptor_set_q_source = self.set_layout_array(pipeline.clone(), 1, 1, q_source);
        let descriptor_set_i_destination = self.set_layout_array(pipeline.clone(), 2, 2, i_dest);
        let descriptor_set_q_destination = self.set_layout_array(pipeline.clone(), 3, 3, q_dest);

        let work_group_counts = [i_source.read().unwrap().len() as u32, 1, 1];
        let arr = [descriptor_set_i_source, descriptor_set_q_source, descriptor_set_i_destination, descriptor_set_q_destination];

        self.bind_descriptor_sets(pipeline, &arr, work_group_counts);
    }

    pub fn build(&mut self) -> Arc<PrimaryAutoCommandBuffer> {
        self.builder.take().unwrap().build().unwrap()
    }
}