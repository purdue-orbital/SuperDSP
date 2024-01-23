pub mod builder;
pub mod traits;
pub mod objects;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(not(feature = "vulkan"))]
pub mod cpu;

pub mod prelude {
    pub use crate::math::builder::Workflow;
    pub use crate::math::builder::WorkflowBuilder;
    pub use crate::math::objects::ComplexF32;
    pub use crate::math::objects::ElementParameter;
    pub use crate::math::traits::PlatformSpecificOperations;
}
