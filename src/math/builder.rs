#[cfg(feature = "vulkan")]
use std::sync::Arc;
#[cfg(feature = "vulkan")]
use lazy_static::lazy_static;
#[cfg(feature = "vulkan")]
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
#[cfg(feature = "vulkan")]
use crate::math::vulkan::{Vulkan, VulkanCommandBuilder};

#[cfg(not(feature = "vulkan"))]
use crate::math::cpu::{CPUCommandBuilder, CPUPipeline};

use crate::math::objects::ElementParameter;
use crate::math::traits::PlatformSpecificOperations;


/// this will increment the number of operations by one in this workflow
fn increment_ops(builder: &mut WorkflowBuilder) {
    builder.num_operations += 1;
}

//----------------------------------------------------------------------------------------------------------------------
// CPU Mode
//----------------------------------------------------------------------------------------------------------------------

#[cfg(not(feature = "vulkan"))]
pub struct Workflow {
    pipeline: CPUPipeline,
}

#[cfg(not(feature = "vulkan"))]
impl Workflow {
    pub fn run(&mut self) {
        self.pipeline.run();
    }
}


/// This builder is a wrapper class around the CPU and GPU compute method that will dynamically switch depending on the
/// feature set
#[cfg(not(feature = "vulkan"))]
pub struct WorkflowBuilder {
    cpu_builder: CPUCommandBuilder,

    /// this is the number of operations stored in this builder
    pub(crate) num_operations: usize,
}


#[cfg(not(feature = "vulkan"))]
impl Default for WorkflowBuilder {
    fn default() -> Self {
        WorkflowBuilder {
            cpu_builder: CPUCommandBuilder::default(),
            num_operations: 0,
        }
    }
}

#[cfg(not(feature = "vulkan"))]
impl PlatformSpecificOperations for WorkflowBuilder {
    fn build(&mut self) -> Workflow {
        Workflow {
            pipeline: self.cpu_builder.build()
        }
    }

    fn pointwise_multiply_f32(&mut self, src: &ElementParameter, dest: &ElementParameter) {
        self.cpu_builder.elementwise_multiply_f32(src.get_f32_array_mut(), dest.get_f32_array_mut())
    }
    fn convolution_f32(&mut self, src1: &ElementParameter, src2: &ElementParameter, dest: &mut ElementParameter) {
        let size = src1.get_f32_array().len() + src2.get_f32_array().len() - 1;

        if dest.get_f32_array().len() != size {
            *dest.f32_array_cpu.clone().unwrap().lock().unwrap() = vec![0.0; size];
        }

        self.cpu_builder.convolution_f32(src1.get_f32_array_mut(), src2.get_f32_array_mut(), dest.get_f32_array_mut());

        increment_ops(self);
    }

    fn scalar_multiply_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter) {
        self.cpu_builder.scalar_multiply_f32(src.get_f32_array_mut(), scalar.get_f32_mut());

        increment_ops(self);
    }

    fn sin_f32(&mut self, src: &ElementParameter) {
        self.cpu_builder.sin_f32(src.get_f32_array_mut());

        increment_ops(self);
    }

    fn cos_f32(&mut self, src: &ElementParameter) {
        self.cpu_builder.cos_f32(src.get_f32_array_mut());

        increment_ops(self);
    }

    fn mod_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter) {
        self.cpu_builder.mod_f32(src.get_f32_array_mut(), scalar.get_f32_mut());

        increment_ops(self);
    }

    fn add_f32(&mut self, src: &ElementParameter, dest: &ElementParameter) {
        self.cpu_builder.add_f32(src.get_f32_array_mut(), dest.get_f32_array_mut());

        increment_ops(self);
    }
    fn scalar_add_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter) {
        self.cpu_builder.scalar_add_f32(src.get_f32_array_mut(), scalar.get_f32_mut());

        increment_ops(self);
    }
    fn copy_f32(&mut self, src: &ElementParameter, dest: &ElementParameter) {
        self.cpu_builder.copy_f32(src.get_f32_array_mut(), dest.get_f32_array_mut());

        increment_ops(self);
    }
    fn fetch_f32(&mut self, src: &ElementParameter, indexes: &ElementParameter, dest: &ElementParameter) {
        self.cpu_builder.fetch_f32(src.get_f32_array_mut(), indexes.get_f32_array_mut(), dest.get_f32_array_mut());

        increment_ops(self);
    }
    fn dft_f32(&mut self, i_src: &ElementParameter, q_src: &ElementParameter, i_dest: &ElementParameter, q_dest: &ElementParameter) {
        self.cpu_builder.dft_f32(i_src.get_f32_array_mut(), q_src.get_f32_array_mut(), i_dest.get_f32_array_mut(), q_dest.get_f32_array_mut());

        increment_ops(self);
    }

    fn idft_f32(&mut self, i_src: &ElementParameter, q_src: &ElementParameter, i_dest: &ElementParameter, q_dest: &ElementParameter) {
        self.cpu_builder.idft_f32(i_src.get_f32_array_mut(), q_src.get_f32_array_mut(), i_dest.get_f32_array_mut(), q_dest.get_f32_array_mut());

        increment_ops(self);
    }
}


//----------------------------------------------------------------------------------------------------------------------
// Vulkan Mode
//----------------------------------------------------------------------------------------------------------------------


#[cfg(feature = "vulkan")]
lazy_static!(
    pub static ref VULKAN: Vulkan = Vulkan::default();
);

#[cfg(feature = "vulkan")]
#[derive(Clone)]
pub struct Workflow {
    pipeline: Arc<PrimaryAutoCommandBuffer>,
}

#[cfg(feature = "vulkan")]
impl Workflow {
    pub fn run(&mut self) {
        VULKAN.run_command_builder(self.pipeline.clone()).wait(None).unwrap();
    }
}


#[cfg(feature = "vulkan")]
pub struct WorkflowBuilder {
    vulkan_builder: VulkanCommandBuilder,

    /// this is the number of operations stored in this builder
    pub(crate) num_operations: usize,
}

#[cfg(feature = "vulkan")]
impl Default for WorkflowBuilder {
    fn default() -> Self {
        WorkflowBuilder {
            vulkan_builder: VULKAN.create_command_builder(),
            num_operations: 0,
        }
    }
}

#[cfg(feature = "vulkan")]
impl PlatformSpecificOperations for WorkflowBuilder {
    fn build(&mut self) -> Workflow {
        Workflow {
            pipeline: self.vulkan_builder.build()
        }
    }
    fn pointwise_multiply_f32(&mut self, src: &ElementParameter, dest: &ElementParameter) {
        self.vulkan_builder.elementwise_multiply_f32(src.get_buffer_f32_array(), dest.get_buffer_f32_array());

        increment_ops(self);
    }
    fn convolution_f32(&mut self, src1: &ElementParameter, src2: &ElementParameter, dest: &mut ElementParameter) {
        dest.f32_array_vulkan = Some(self.vulkan_builder.convolution_f32(src1.get_buffer_f32_array(), src2.get_buffer_f32_array()));

        increment_ops(self);
    }
    fn scalar_multiply_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter) {
        self.vulkan_builder.scalar_multiply_f32(src.get_buffer_f32_array(), scalar.get_buffer_f32());

        increment_ops(self);
    }
    fn sin_f32(&mut self, src: &ElementParameter) {
        self.vulkan_builder.sin_f32(src.get_buffer_f32_array());

        increment_ops(self);
    }
    fn cos_f32(&mut self, src: &ElementParameter) {
        self.vulkan_builder.cos_f32(src.get_buffer_f32_array());

        increment_ops(self);
    }
    fn mod_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter) {
        self.vulkan_builder.mod_f32(src.get_buffer_f32_array(), scalar.get_buffer_f32());

        increment_ops(self);
    }

    fn add_f32(&mut self, src: &ElementParameter, dest: &ElementParameter) {
        self.vulkan_builder.add_f32(src.get_buffer_f32_array(), dest.get_buffer_f32_array());

        increment_ops(self);
    }
    fn scalar_add_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter) {
        self.vulkan_builder.scalar_add_f32(scalar.get_buffer_f32(), src.get_buffer_f32_array());

        increment_ops(self);
    }
    fn copy_f32(&mut self, src: &ElementParameter, dest: &ElementParameter) {
        self.vulkan_builder.copy_f32(src.get_buffer_f32_array(), dest.get_buffer_f32_array());

        increment_ops(self);
    }

    fn fetch_f32(&mut self, src: &ElementParameter, indexes: &ElementParameter, dest: &ElementParameter) {
        self.vulkan_builder.fetch_f32(src.get_buffer_f32_array(), indexes.get_buffer_f32_array(), dest.get_buffer_f32_array());

        increment_ops(self);
    }

    fn dft_f32(&mut self, i_src: &ElementParameter, q_src: &ElementParameter, i_dest: &ElementParameter, q_dest: &ElementParameter) {
        self.vulkan_builder.dft_f32(i_src.get_buffer_f32_array(), q_src.get_buffer_f32_array(), i_dest.get_buffer_f32_array(), q_dest.get_buffer_f32_array());

        increment_ops(self);
    }
    fn idft_f32(&mut self, i_src: &ElementParameter, q_src: &ElementParameter, i_dest: &ElementParameter, q_dest: &ElementParameter) {
        self.vulkan_builder.idft_f32(i_src.get_buffer_f32_array(), q_src.get_buffer_f32_array(), i_dest.get_buffer_f32_array(), q_dest.get_buffer_f32_array());

        increment_ops(self);
    }
}
