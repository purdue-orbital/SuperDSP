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
pub struct WorkflowBuilder<'a> {
    #[cfg(feature = "vulkan")]
    vulkan_builder: VulkanCommandBuilder,

    #[cfg(not(feature = "vulkan"))]
    cpu_builder: CPUCommandBuilder<'a>
}

impl<'a> Default for WorkflowBuilder<'a>{
    fn default() -> Self {

        WorkflowBuilder{
            #[cfg(feature = "vulkan")]
            vulkan_builder: VULKAN.create_command_builder(),
            
            #[cfg(not(feature = "vulkan"))]
            cpu_builder: CPUCommandBuilder::default()

        }
    }
}

impl<'a> WorkflowBuilder<'a> {

}