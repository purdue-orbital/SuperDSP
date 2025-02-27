use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use vulkano::buffer::{Buffer, Subbuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, Queue, QueueCreateInfo, QueueFlags};
use vulkano::half;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{ComputePipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::Pipeline as VulkanoPipeline;
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::sync::GpuFuture;
use crate::gpu::GpuDevice;
use crate::prelude::{form, form_first_stage, form_last_stage, Data, DataKind, FirstStageTrait, LastStageTrait, Stage};
use crate::prelude::wave_gen::VulkanWaveGen;

pub mod stages;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PipelineType {
    Loop,
    OnSend,
    OnRecv,
}

#[derive(Default, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) enum SubBuffer{
    F64(Subbuffer<[f64]>),
    F32(Subbuffer<[f32]>),
    F16(Subbuffer<[half::f16]>),
    WaveGen(Subbuffer<[VulkanWaveGen]>),
    
    #[default]
    Empty,
}

pub fn get_f64_buffer(buffer: SubBuffer) -> Subbuffer<[f64]> {
    match buffer {
        SubBuffer::F64(x) => x,
        _ => panic!("Expected F64"),
    }
}

pub fn get_f32_buffer(buffer: SubBuffer) -> Subbuffer<[f32]> {
    match buffer {
        SubBuffer::F32(x) => x,
        _ => panic!("Expected F32"),
    }
}

pub fn get_f16_buffer(buffer: SubBuffer) -> Subbuffer<[half::f16]> {
    match buffer {
        SubBuffer::F16(x) => x,
        _ => panic!("Expected F16"),
    }
}

#[derive(Default)]
pub struct PipelineSettings {
    frequency: Option<f64>,
    sample_rate: Option<f64>,

    sps: Option<usize>,

    num_taps: Option<usize>,

    pub prev_stage_output: SubBuffer,
    
    pub stages: Vec<PipelineShaderStageCreateInfo>,
    pub stages_work_groups: Vec<[u32; 3]>,

    pub(crate) gpu: Option<GpuDevice>,
    
    pub(crate) queue: Option<Arc<Queue>>,
    pub(crate) memory_allocator: Option<Arc<StandardMemoryAllocator>>,
    pub(crate) prev_stage_output_buffer: Option<Buffer>,
    pub(crate) device: Option<Arc<Device>>,
    pub(crate) pipeline_buffers: Vec<Vec<WriteDescriptorSet>>
}

impl PipelineSettings {
    pub fn set_frequency(&mut self, freq: f64) -> &mut Self {
        self.frequency = Some(freq);
        self
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) -> &mut Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    pub fn set_sps(&mut self, sps: usize) -> &mut Self {
        self.sps = Some(sps);
        self
    }

    pub fn set_num_taps(&mut self, num_taps: usize) -> &mut Self {
        self.num_taps = Some(num_taps);
        self
    }

    pub fn set_device(&mut self, gpu: GpuDevice) -> &mut Self {
        self.gpu = Some(gpu);

        self
    }
}

#[derive(Default)]
pub struct PipelineBuilder<I: Into<Data>, O: From<Data>> {
    first_stage: Option<Box<dyn FirstStageTrait<I>>>,
    last_stage: Option<Box<dyn LastStageTrait<O>>>,
    stages: Vec<Option<Box<dyn Stage>>>,
}


impl<I: Clone + Into<Data> + 'static, O: From<Data> + 'static + Send + Sync> PipelineBuilder<I, O> {
    pub fn add_stage<S: Stage + Default + 'static>(&mut self) -> &mut Self {
        self.stages.push(Some(Box::new(form::<S>())));

        self
    }

    pub fn add_first_stage<S: FirstStageTrait<I> + 'static + Default>(&mut self) -> &mut Self {
        self.first_stage = Some(Box::new(form_first_stage::<S>()));
        
        self
    }

    pub fn add_last_stage<S: LastStageTrait<O> + 'static + Default>(&mut self) -> &mut Self {
        self.last_stage = Some(Box::new(form_last_stage::<S>()));
        self
    }

    pub async fn build(&mut self, pipeline_type: PipelineType, settings: &mut PipelineSettings) -> Pipeline<I, O> {
        // create gpu device
        let gpu = settings.gpu.take().expect("No GPU selected");

        gpu.physical_device.extension_properties()
            .iter()
            .enumerate()
            .position( |(_i,properties)|{
           println!("{}",properties.extension_name);

            false
        });

        let queue_family_index = gpu.physical_device.queue_family_properties().iter().enumerate().position(|(_i, properties)| {
            properties.queue_flags.contains(QueueFlags::COMPUTE)
        }).expect("No compute queue family found") as u32;


        let features = DeviceFeatures {
            storage_buffer16_bit_access: true,
            ..DeviceFeatures::empty()
        };

        let (device, mut queues) = Device::new(
            gpu.physical_device.clone(),
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,

                    ..Default::default()
                }],
                
                enabled_features: features,
                
                ..Default::default()
            },
        ).expect("failed to create device");
        
        // set device
        settings.device = Some(device.clone());

        // we only need one queue
        let queue = queues.next().unwrap();
        settings.queue = Some(queue.clone());
        
        // create memory allocator
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        settings.memory_allocator = Some(memory_allocator.clone());
        
        // first stage
        let mut first_stage = self.first_stage.take().expect("Expected first stage");
        first_stage.configure(settings);

        // configure pipeline
        for stage in self.stages.iter_mut() {
            let mut stage = stage.take().expect("Expected stage");

            stage.configure(settings);
        }
        
        // last stage
        let mut last_stage = self.last_stage.take().expect("Expected last stage");
        last_stage.configure(settings);
        
        // create command buffer allocator
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        // create command buffer
        let mut command_buffer_builder = AutoCommandBufferBuilder::primary (
            command_buffer_allocator,
            settings.queue.clone().expect("Expected queue").queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        ).unwrap();
        
        
        // create pipeline from stages
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(settings.stages.as_slice())
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        ).unwrap();
        
        // create compute pipelines
        for (index,x) in settings.stages.iter().enumerate() {
            let compute_pipeline = ComputePipeline::new(
                device.clone(),
                None,
                ComputePipelineCreateInfo::stage_layout(x.clone(), layout.clone()),
            ).expect("failed to create compute pipeline");

            // set descriptor set
            let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(device.clone(), Default::default()));
            let pipeline_layout = compute_pipeline.layout();
            let descriptor_set_layouts = pipeline_layout.set_layouts();

            let descriptor_set_layout_index = 0;
            let descriptor_set_layout = descriptor_set_layouts.get(descriptor_set_layout_index).unwrap();

            // add memory buffers to work environment
            let descriptor_set = DescriptorSet::new(
                descriptor_set_allocator,
                descriptor_set_layout.clone(),
                settings.pipeline_buffers[index].clone(),
                [],
            ).unwrap();
            
            command_buffer_builder.bind_pipeline_compute(compute_pipeline.clone()).unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Compute,
                    pipeline_layout.clone(),
                    0,
                    descriptor_set.clone(),
                ).unwrap();
            
            unsafe { command_buffer_builder.dispatch(settings.stages_work_groups[index]).unwrap(); }
        }
        
        // build command buffer
        let command_buffer = command_buffer_builder.build().unwrap();
        
        let mut first_stage_sender = None;
        let mut last_stage_reader = None;
        
        // put first stage last stage into a new thread
        if pipeline_type == PipelineType::Loop {
            tokio::spawn(async move {
                let not_used = Data::new(DataKind::Empty);
                let mut not_used2 = Data::new(DataKind::Empty);
                
                loop {
                    // run first stage cpu operation
                    first_stage.process(&not_used, &mut not_used2);
                    
                    // run gpu
                    vulkano::sync::now(device.clone()).then_execute(queue.clone(), command_buffer.clone()).unwrap().then_signal_fence_and_flush().unwrap().wait(None).unwrap();
                    
                    // run last stage cpu operation
                    last_stage.process(&not_used, &mut not_used2);
                }
            });
        }else if pipeline_type == PipelineType::OnSend {
            let (first_stage_in, mut first_stage_out) = tokio::sync::mpsc::channel(100);
            
            first_stage_sender = Some(first_stage_in);
            
            tokio::spawn(async move {
                // Prepare thread
                let not_used = Data::new(DataKind::Empty);
                let mut not_used2 = Data::new(DataKind::Empty);
                
                // Process data that is sent to the first stage
                while let Some(data) = first_stage_out.recv().await {
                    // Process data (this is primarily just sending data to the GPU)
                    first_stage.process(&data, &mut not_used2);
                    
                    // run the command buffer
                    vulkano::sync::now(device.clone()).then_execute(queue.clone(), command_buffer.clone()).unwrap().then_signal_fence_and_flush().unwrap().wait(None).unwrap();;
                    
                    // run the last stage cpu operation
                    last_stage.process(&not_used, &mut not_used2);
                }
            });
        }else {
            let (last_stage_in, last_stage_out) = tokio::sync::mpsc::channel(100);
            
            last_stage_reader = Some(last_stage_out);
            
            tokio::spawn(async move {
                // Prepare thread
                let not_used = Data::new(DataKind::Empty);
                let mut not_used2 = Data::new(DataKind::Empty);
                
                let mut data = Data::new(DataKind::Empty);
                
                loop{
                    // run first stage cpu operation
                    first_stage.process(&not_used, &mut not_used2);
                    
                    // run gpu
                    vulkano::sync::now(device.clone()).then_execute(queue.clone(), command_buffer.clone()).unwrap().then_signal_fence_and_flush().unwrap().wait(None).unwrap();
                    
                    // run last stage cpu operation
                    last_stage.process(&not_used, &mut data);
                    
                    // send data out to main program
                    last_stage_in.send(data.clone()).await.unwrap();
                }
            });
        }


        // create pipeline
        Pipeline {
            first_stage_in: first_stage_sender,
            last_stage_out: last_stage_reader,

            phantom: PhantomData,

            pipeline_type,
        }
    }
}

pub struct Pipeline<I, O> {
    first_stage_in: Option<Sender<Data>>,
    last_stage_out: Option<Receiver<Data>>,
    
    phantom: PhantomData<(I, O)>,

    pipeline_type: PipelineType,
}
impl<I: Into<Data> + Clone + Send + Sync + Debug + 'static, O: Into<O> + Clone + Send + Sync + 'static + Into<Data> + From<Data>> Pipeline<I, O> {
    pub async fn send(&mut self, data: I) -> anyhow::Result<()> {
        self.first_stage_in.as_ref().unwrap().send(data.into()).await?;

        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<O> {
        Ok(self.last_stage_out.as_mut().unwrap().recv().await.unwrap().into())
    }
}