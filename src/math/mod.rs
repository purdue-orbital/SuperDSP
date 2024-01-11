#[cfg(feature = "vulkan")]
pub mod vulkan;
#[cfg(feature = "vulkan")]
use vulkano::buffer::Subbuffer;
#[cfg(feature = "vulkan")]
use crate::math::builder::VULKAN;

pub mod cpu;
pub mod builder;

use crate::math::ValueTypes::F32;

pub enum ValueTypes{
    F32,
}

/// This will dynamically be
pub struct ElementParameter {
    vtype: ValueTypes,


    #[cfg(not(feature = "vulkan"))]
    f32_cpu: Option<Vec<f32>>,


    #[cfg(feature = "vulkan")]
    f32_vulkan: Option<Subbuffer<[f32]>>
}

impl<'a> ElementParameter{
    pub fn new_f32(arr: &[f32]) -> ElementParameter {
        let mut new = ElementParameter{
            vtype: F32,

            #[cfg(not(feature = "vulkan"))]
            f32_cpu: None,

            #[cfg(feature = "vulkan")]
            f32_vulkan: None,
        };

        new.set_f32(arr);

        new
    }

    pub fn get_f32_array(&mut self) -> Vec<f32> {

        #[cfg(not(feature = "vulkan"))]
        let arr = self.f32_cpu.clone().unwrap();

        #[cfg(feature = "vulkan")]
        let arr = self.f32_vulkan.as_mut().unwrap().read().unwrap().to_vec();

        arr
    }

    #[cfg(not(feature = "vulkan"))]
    pub fn get_f32_array_mut(&mut self) -> &mut [f32] {
        self.f32_cpu.as_mut().unwrap().as_mut_slice()
    }

    #[cfg(not(feature = "vulkan"))]
    pub fn set_f32(&mut self, arr: &[f32]) {
        self.vtype = F32;
        self.f32_cpu = Some(arr.to_vec());
    }

    #[cfg(feature = "vulkan")]
    pub fn get_buffer_f32(&mut self) -> Subbuffer<[f32]>{
        self.f32_vulkan.clone().unwrap()
    }

    #[cfg(feature = "vulkan")]
    pub fn set_f32(&mut self, arr: &[f32]) {
        self.vtype = F32;
        self.f32_vulkan = Some(VULKAN.store_to_vram_array(arr));
    }
}