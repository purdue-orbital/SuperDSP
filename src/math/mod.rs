#[cfg(not(feature = "vulkan"))]
use std::sync::{Arc,Mutex};

#[cfg(not(feature = "vulkan"))]
use std::sync::RwLock;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "vulkan")]
use vulkano::buffer::Subbuffer;
#[cfg(feature = "vulkan")]
use crate::math::builder::VULKAN;

pub mod cpu;
pub mod builder;

use crate::math::ValueTypes::{F32, F32Array};

pub enum ValueTypes{
    F32Array,
    F32
}

/// This will dynamically switch between types
pub struct ElementParameter {
    vtype: ValueTypes,

    #[cfg(not(feature = "vulkan"))]
    f32_array_cpu: Option<Arc<Mutex<Vec<f32>>>>,

    #[cfg(not(feature = "vulkan"))]
    f32_cpu: Option<Arc<RwLock<f32>>>,


    #[cfg(feature = "vulkan")]
    f32_array_vulkan: Option<Subbuffer<[f32]>>,

    #[cfg(feature = "vulkan")]
    f32_vulkan: Option<Subbuffer<f32>>
}

impl ElementParameter{
    pub fn new_f32_array(arr: &[f32]) -> ElementParameter {
        let mut new = ElementParameter{
            vtype: F32Array,

            #[cfg(not(feature = "vulkan"))]
            f32_array_cpu: None,

            #[cfg(not(feature = "vulkan"))]
            f32_cpu: None,

            #[cfg(feature = "vulkan")]
            f32_array_vulkan: None,

            #[cfg(feature = "vulkan")]
            f32_vulkan: None,
        };

        new.set_f32_array(arr);

        new
    }

    pub fn new_f32(arr: f32) -> ElementParameter {
        let mut new = ElementParameter{
            vtype: F32,

            #[cfg(not(feature = "vulkan"))]
            f32_array_cpu: None,

            #[cfg(not(feature = "vulkan"))]
            f32_cpu: None,

            #[cfg(feature = "vulkan")]
            f32_array_vulkan: None,

            #[cfg(feature = "vulkan")]
            f32_vulkan: None,
        };

        new.set_f32(arr);

        new
    }

    pub fn get_f32_array(&self) -> Vec<f32> {

        #[cfg(not(feature = "vulkan"))]
        let arr = self.f32_array_cpu.as_ref().unwrap().lock().unwrap().to_vec();

        #[cfg(feature = "vulkan")]
        let arr = self.f32_array_vulkan.as_ref().unwrap().read().unwrap().to_vec();

        arr
    }


    #[cfg(not(feature = "vulkan"))]
    pub fn get_f32_array_mut(&self) -> Arc<Mutex<Vec<f32>>> {
        self.f32_array_cpu.clone().unwrap()
    }

    #[cfg(not(feature = "vulkan"))]
    pub fn get_f32_mut(&self) -> Arc<RwLock<f32>> {
        self.f32_cpu.clone().unwrap()
    }


    #[cfg(not(feature = "vulkan"))]
    pub fn set_f32_array(&mut self, arr: &[f32]) {
        self.vtype = F32Array;
        self.f32_array_cpu = Some(Arc::new(Mutex::new(arr.to_vec())));
    }

    #[cfg(not(feature = "vulkan"))]
    pub fn set_f32(&mut self, arr: f32) {
        self.vtype = F32;
        self.f32_cpu = Some(Arc::new(RwLock::new(arr)));
    }

    #[cfg(feature = "vulkan")]
    pub fn set_f32(&mut self, arr: f32) {
        self.vtype = F32;
        self.f32_vulkan = Some(VULKAN.store_to_vram_var(arr));
    }

    #[cfg(feature = "vulkan")]
    pub fn get_buffer_f32_array(&self) -> Subbuffer<[f32]>{
        self.f32_array_vulkan.clone().unwrap()
    }


    #[cfg(feature = "vulkan")]
    pub fn get_buffer_f32(&self) -> Subbuffer<f32>{
        self.f32_vulkan.clone().unwrap()
    }



    #[cfg(feature = "vulkan")]
    pub fn set_f32_array(&mut self, arr: &[f32]) {
        self.vtype = F32Array;
        self.f32_array_vulkan = Some(VULKAN.store_to_vram_array(arr));
    }
}