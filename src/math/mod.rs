pub mod builder;
pub mod traits;
pub mod objects;

#[cfg(all(feature = "vulkan",not(feature = "fpga")))]
pub mod vulkan;

#[cfg(all(not(feature = "vulkan"), not(feature = "fpga")))]
pub mod cpu;

#[cfg(all(feature = "fpga",not(feature = "vulkan"),not(feature = "ui")))]
pub mod fpga;

pub mod prelude {
    pub use crate::math::builder::Workflow;
    pub use crate::math::builder::WorkflowBuilder;
    pub use crate::math::objects::ComplexF32;
    pub use crate::math::objects::ElementParameter;
    pub use crate::math::traits::PlatformSpecificOperations;
}
