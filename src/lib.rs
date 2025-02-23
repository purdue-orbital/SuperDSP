// #![no_std]
// extern crate alloc;

pub mod cpu;
pub mod gpu;

#[cfg(not(feature = "gpu"))]
pub mod prelude {
    pub use crate::cpu::pipeline::stages::*;
    pub use crate::cpu::pipeline::Pipeline;
}

#[cfg(feature = "gpu")]
pub mod prelude {
    pub use crate::gpu::pipeline::stages::*;
    pub use crate::gpu::pipeline::Pipeline;
}
