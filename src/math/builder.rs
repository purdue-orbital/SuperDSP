use crate::math::ElementParameter;

//----------------------------------------------------------------------------------------------------------------------
// CPU Mode
//----------------------------------------------------------------------------------------------------------------------

#[cfg(not(feature = "vulkan"))]
use crate::math::cpu::{CPUCommandBuilder, CPUPipeline};
#[cfg(not(feature = "vulkan"))]
pub struct Workflow {
    pipeline: CPUPipeline
}

#[cfg(not(feature = "vulkan"))]
impl Workflow {
    pub fn run(&mut self){
        self.pipeline.run();
    }
}


/// This builder is a wrapper class around the CPU and GPU compute method that will dynamically switch depending on the
/// feature set
#[cfg(not(feature = "vulkan"))]
pub struct WorkflowBuilder {
    cpu_builder: CPUCommandBuilder
}


#[cfg(not(feature = "vulkan"))]
impl Default for WorkflowBuilder{
    fn default() -> Self {
        WorkflowBuilder{
            cpu_builder: CPUCommandBuilder::default(),
        }
    }
}

#[cfg(not(feature = "vulkan"))]
impl WorkflowBuilder{
    pub fn build(&mut self) -> Workflow{
        Workflow{
            pipeline: self.cpu_builder.build()
        }
    }

    pub fn pointwise_multiply_f32(&mut self, src: &ElementParameter, dest: &ElementParameter ){
        self.cpu_builder.elementwise_multiply_f32(src.get_f32_array_mut(), dest.get_f32_array_mut())
    }
    pub fn convolution_f32(&mut self, src1: &ElementParameter, src2: &ElementParameter, dest: &ElementParameter){

        let size = src1.get_f32_array().len() + src2.get_f32_array().len() - 1;

        if dest.get_f32_array().len() != size{
            *dest.f32_array_cpu.clone().unwrap().lock().unwrap() = vec![0.0; size];
        }

        self.cpu_builder.convolution_f32(src1.get_f32_array_mut(), src2.get_f32_array_mut(), dest.get_f32_array_mut())
    }

    pub fn scalar_multiply_f32(&mut self, src: &ElementParameter, scaler: &ElementParameter ){
        self.cpu_builder.scalar_multiply_f32(src.get_f32_array_mut(), scaler.get_f32_mut())
    }
}



//----------------------------------------------------------------------------------------------------------------------
// Vulkan Mode
//----------------------------------------------------------------------------------------------------------------------


#[cfg(feature = "vulkan")]
use lazy_static::lazy_static;

#[cfg(feature = "vulkan")]
use std::sync::{Arc};

#[cfg(feature = "vulkan")]
use vulkano::command_buffer::PrimaryAutoCommandBuffer;

#[cfg(feature = "vulkan")]
use crate::math::vulkan::{Vulkan, VulkanCommandBuilder};

#[cfg(feature = "vulkan")]
lazy_static!(
    pub static ref VULKAN: Vulkan = Vulkan::default();
);

#[cfg(feature = "vulkan")]
pub struct Workflow {
    pipeline: Arc<PrimaryAutoCommandBuffer>
}
#[cfg(feature = "vulkan")]
impl Workflow {
    pub fn run(&mut self){
        VULKAN.run_command_builder(self.pipeline.clone()).wait(None).unwrap();
    }
}


#[cfg(feature = "vulkan")]
pub struct WorkflowBuilder {
    vulkan_builder: VulkanCommandBuilder
}

#[cfg(feature = "vulkan")]
impl Default for WorkflowBuilder{
    fn default() -> Self {

        WorkflowBuilder{
            #[cfg(feature = "vulkan")]
            vulkan_builder: VULKAN.create_command_builder(),
        }
    }
}

#[cfg(feature = "vulkan")]
impl WorkflowBuilder{

    pub fn build(&mut self) -> Workflow{
        Workflow{
            pipeline: self.vulkan_builder.build()
        }
    }

    pub fn pointwise_multiply_f32(&mut self, src: &ElementParameter, dest: &ElementParameter){
        self.vulkan_builder.elementwise_multiply_f32(src.get_buffer_f32_array(), dest.get_buffer_f32_array());
    }
    pub fn convolution_f32(&mut self, src1: &ElementParameter, src2: &ElementParameter, dest: &mut ElementParameter){
        dest.f32_array_vulkan = Some(self.vulkan_builder.convolution_f32(src1.get_buffer_f32_array(), src2.get_buffer_f32_array()));
    }
    pub fn scalar_multiply_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter ){
        self.vulkan_builder.scalar_multiply_f32(src.get_buffer_f32_array(), scalar.get_buffer_f32())
    }
}
