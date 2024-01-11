use lazy_static::lazy_static;
use crate::math::cpu::CPUCommandBuilder;



#[cfg(feature = "vulkan")]
use crate::math::vulkan::{Vulkan, VulkanCommandBuilder};

#[cfg(feature = "vulkan")]
lazy_static!(
    static ref VULKAN: Vulkan = Vulkan::default();
);

/// This builder is a wrapper class around the CPU and GPU compute method that will dynamiclly switch depending on the
/// feature set
#[cfg(not(feature = "vulkan"))]
pub struct WorkflowBuilder<'a> {
    cpu_builder: CPUCommandBuilder<'a>
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

#[cfg(not(feature = "vulkan"))]
impl<'a> Default for WorkflowBuilder<'a>{
    fn default() -> Self {
        WorkflowBuilder{
            cpu_builder: CPUCommandBuilder::default(),
        }
    }
}

#[cfg(not(feature = "vulkan"))]
impl<'a> WorkflowBuilder<'a>{
    pub fn pointwise_multiply_f32(&self){

    }
}
