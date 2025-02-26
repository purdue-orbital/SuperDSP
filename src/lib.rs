// #![no_std]
// extern crate alloc;

extern crate core;

#[cfg(not(feature = "gpu"))]
pub mod cpu;


#[cfg(not(feature = "gpu"))]
pub mod prelude {
    pub use crate::cpu::pipeline::stages::*;
    pub use crate::cpu::pipeline::PipelineBuilder;
}

#[cfg(feature = "gpu")]
pub mod gpu;

#[cfg(feature = "gpu")]
pub mod prelude {
    pub use crate::gpu::pipeline::stages::*;
    pub use crate::gpu::pipeline::PipelineBuilder;
}
