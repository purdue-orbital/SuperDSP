use std::sync::Arc;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::VulkanLibrary;

pub mod pipeline;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuType {
    Integrated,
    Discrete,
    Virtual,
    Cpu,
    Other,
}

#[derive(Clone)]
pub struct GpuDevice {
    pub(crate) physical_device: Arc<PhysicalDevice>,
}

impl GpuDevice {
    pub fn get_name(&self) -> String {
        self.physical_device.properties().device_name.clone()
    }

    pub fn get_type(&self) -> GpuType {
        match self.physical_device.properties().device_type {
            PhysicalDeviceType::IntegratedGpu => GpuType::Integrated,
            PhysicalDeviceType::DiscreteGpu => GpuType::Discrete,
            PhysicalDeviceType::VirtualGpu => GpuType::Virtual,
            PhysicalDeviceType::Cpu => GpuType::Cpu,
            _ => GpuType::Other,
        }
    }
}

pub fn list_devices() -> Vec<GpuDevice> {
    let mut vec = Vec::new();

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    ).expect("failed to create instance");

    let physical_device = instance.enumerate_physical_devices().expect("could not enumerate devices");

    for device in physical_device {
        vec.push(GpuDevice {
            physical_device: device,
        });
    }

    vec
}
