// #![no_std]
// extern crate alloc;
#![feature(type_alias_impl_trait)]

extern crate core;
pub trait Stage<Args> {
    fn invoke(&mut self, args: Args);
}

impl<F> Stage<()> for F where
    F: FnMut(),
{
    fn invoke(&mut self, args: ()) {
        self()
    }
}

impl<F> Stage<Frequency> for F where
    F: FnMut(Frequency),
{
    fn invoke(&mut self, args: Frequency) {
        self(args)
    }
}

impl<F> Stage<(Frequency, Gain)> for F where
    F: FnMut(Frequency, Gain),
{
    fn invoke(&mut self, args: (Frequency, Gain)) {
        self(args.0, args.1)
    }
}


pub type Frequency = f32;
pub type CarrierFrequency = Frequency;
pub type Gain = u32;
pub type SampleRate = f32;
pub type SamplesPerSymbol = usize;
pub type NumberOfTaps = usize;
//
// use std::sync::Arc;
// #[cfg(feature = "gpu")]
// use vulkano::descriptor_set::WriteDescriptorSet;
// #[cfg(feature = "gpu")]
// use vulkano::device::{Device, Queue};
// #[cfg(feature = "gpu")]
// use vulkano::memory::allocator::StandardMemoryAllocator;
// #[cfg(feature = "gpu")]
// use vulkano::pipeline::PipelineShaderStageCreateInfo;
// #[cfg(feature = "gpu")]
// use crate::gpu::GpuDevice;
// #[cfg(feature = "gpu")]
// use crate::gpu::pipeline::SubBuffer;
//
// #[cfg(not(feature = "gpu"))]
// pub mod cpu;
//
//
// #[cfg(not(feature = "gpu"))]
// pub mod prelude {
//     pub use crate::cpu::pipeline::stages::*;
//     pub use crate::cpu::pipeline::PipelineBuilder;
// }
//
// #[cfg(feature = "gpu")]
// pub mod gpu;
//
// #[cfg(feature = "gpu")]
// pub mod prelude {
//     pub use crate::gpu::pipeline::stages::*;
//     pub use crate::gpu::pipeline::PipelineBuilder;
// }
