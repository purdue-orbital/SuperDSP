#[cfg(not(feature = "vulkan"))]
use std::sync::{Arc,Mutex};

#[cfg(not(feature = "vulkan"))]
use std::sync::RwLock;
use num_complex::Complex;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "vulkan")]
use vulkano::buffer::Subbuffer;
#[cfg(feature = "vulkan")]
use crate::math::builder::VULKAN;

pub mod cpu;
pub mod builder;

use crate::math::ValueTypes::{F32, F32Array};

#[derive(Clone)]
pub struct ComplexF32{

    #[cfg(not(feature = "vulkan"))]
    real_comp_cpu: Arc<Mutex<Vec<f32>>>,

    #[cfg(not(feature = "vulkan"))]
    imag_comp_cpu: Arc<Mutex<Vec<f32>>>,

    #[cfg(feature = "vulkan")]
    real_comp_vulkan: Subbuffer<[f32]>,

    #[cfg(feature = "vulkan")]
    imag_comp_vulkan: Subbuffer<[f32]>,
}

impl ComplexF32{
    pub fn new(arr: Vec<Complex<f32>>) -> Self{
        // split array into two arrays
        let real_arr:Vec<f32> = arr.iter().map(|&val| val.re).collect();
        let imag_arr:Vec<f32> = arr.iter().map(|&val| val.im).collect();

        // convert to proper intermediate
        #[cfg(feature = "vulkan")]
        let real_comp_vulkan = VULKAN.store_to_vram_array(&real_arr);
        #[cfg(feature = "vulkan")]
        let imag_comp_vulkan = VULKAN.store_to_vram_array(&imag_arr);


        #[cfg(not(feature = "vulkan"))]
        let real_comp_cpu = Arc::new(Mutex::new(real_arr));
        #[cfg(not(feature = "vulkan"))]
        let imag_comp_cpu = Arc::new(Mutex::new(imag_arr));

        ComplexF32{
            #[cfg(feature = "vulkan")]
            real_comp_vulkan,
            #[cfg(feature = "vulkan")]
            imag_comp_vulkan,

            #[cfg(not(feature = "vulkan"))]
            real_comp_cpu,

            #[cfg(not(feature = "vulkan"))]
            imag_comp_cpu
        }
    }

    pub fn get_imag_array_wrapped(&self) -> ElementParameter{

        #[cfg(not(feature = "vulkan"))]
        let elem = ElementParameter{
            vtype: F32,
            f32_array_cpu: Some(self.imag_comp_cpu.clone()),
            f32_cpu: None,
            complex_f32: None,
        };


        #[cfg(feature = "vulkan")]
            let elem = ElementParameter{
            vtype: F32,
            complex_f32: None,
            f32_array_vulkan: Some(self.imag_comp_vulkan.clone()),
            f32_vulkan: None,
        };

        elem
    }
    pub fn get_real_array_wrapped(&self) -> ElementParameter{

        #[cfg(not(feature = "vulkan"))]
            let elem = ElementParameter{
            vtype: F32,
            f32_array_cpu: Some(self.real_comp_cpu.clone()),
            f32_cpu: None,
            complex_f32: None,
        };


        #[cfg(feature = "vulkan")]
            let elem = ElementParameter{
            vtype: F32,
            complex_f32: None,
            f32_array_vulkan: Some(self.real_comp_vulkan.clone()),
            f32_vulkan: None,
        };

        elem
    }

}


pub enum ValueTypes{
    F32Array,
    F32,
    F32Complex
}

/// This will dynamically switch between types
pub struct ElementParameter {
    vtype: ValueTypes,

    #[cfg(not(feature = "vulkan"))]
    f32_array_cpu: Option<Arc<Mutex<Vec<f32>>>>,

    #[cfg(not(feature = "vulkan"))]
    f32_cpu: Option<Arc<RwLock<f32>>>,

    complex_f32: Option<ComplexF32>,

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

            complex_f32: None,

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

            complex_f32: None,

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

    #[cfg(not(feature = "vulkan"))]
    pub fn get_complex_f32(&mut self) -> ComplexF32 {
        self.complex_f32.clone().unwrap()
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