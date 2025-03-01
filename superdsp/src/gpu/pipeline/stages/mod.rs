pub mod wave_gen;
pub mod print_debug;

pub(crate) mod shaders;
pub(crate) mod etc;
pub mod matrix_multiplication;

use crate::gpu::pipeline::PipelineSettings;
use fixed::types::{U16F16, U8F8};
use fixed::{FixedI16, FixedI32};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DataKind {
    #[default]
    Empty,
    F64,
    ComplexF64,
    FixedPoint,
    ComplexFixedPoint,
    U8,
    F32,
}

#[derive(Clone, Default)]
pub struct Data {
    pub kind: DataKind,

    pub f64_data: Option<Vec<f64>>,
    pub complex_f64: Option<Vec<num::Complex<f64>>>,

    pub fixed_point: Option<Vec<FixedI16<U8F8>>>,
    pub complex_fixed_point: Option<Vec<num::Complex<FixedI32<U16F16>>>>,

    pub u8_data: Option<Vec<u8>>,
}

unsafe impl Send for Data {}
unsafe impl Sync for Data {}

impl Into<Data> for Vec<f64> {
    fn into(self) -> Data {
        let mut data = Data::new(DataKind::F64);
        data.set_f64_data(self);
        data
    }
}

impl From<Data> for Vec<f64> {
    fn from(value: Data) -> Self {
        value.get_f64_data().clone()
    }
}

impl Into<Data> for Vec<()> {
    fn into(self) -> Data {
        Data::new(DataKind::Empty)
    }
}

impl From<Data> for Vec<()> {
    fn from(value: Data) -> Self {
        vec![]
    }
}

impl Data {
    pub fn new(kind: DataKind) -> Self {
        Self {
            kind,

            f64_data: None,
            complex_f64: None,

            fixed_point: None,
            complex_fixed_point: None,

            u8_data: None,
        }
    }

    pub fn set_f64_data(&mut self, data: Vec<f64>) {
        self.f64_data = Some(data);
    }

    pub fn set_complex_data(&mut self, data: Vec<num::Complex<f64>>) {
        self.complex_f64 = Some(data);
    }

    pub fn set_fixed_point_data(&mut self, data: Vec<FixedI16<U8F8>>) {
        self.fixed_point = Some(data);
    }

    pub fn set_complex_fixed_point_data(&mut self, data: Vec<num::Complex<FixedI32<U16F16>>>) {
        self.complex_fixed_point = Some(data);
    }

    pub fn get_f64_data(&self) -> &Vec<f64> {
        self.f64_data.as_ref().expect("Data is not f64")
    }

    pub fn get_complex_data(&self) -> &Vec<num::Complex<f64>> {
        self.complex_f64.as_ref().expect("Data is not complex")
    }

    pub fn get_fixed_point_data(&self) -> &Vec<FixedI16<U8F8>> {
        self.fixed_point.as_ref().expect("Data is not fixed point")
    }

    pub fn get_complex_fixed_point_data(&self) -> &Vec<num::Complex<FixedI32<U16F16>>> {
        self.complex_fixed_point.as_ref().expect("Data is not complex fixed point")
    }

    pub fn is_f64(&self) -> bool {
        self.kind == DataKind::F64
    }

    pub fn is_complex_f64(&self) -> bool {
        self.kind == DataKind::ComplexF64
    }

    pub fn is_fixed_point(&self) -> bool {
        self.kind == DataKind::FixedPoint
    }
}

// pub trait Stage: Send + Sync + Default {
//     fn configure(&mut self, data: &mut PipelineSettings);
//     fn process(&mut self, input: &Data, output: &mut Data);
//     fn get_output_data_type(&self) -> DataKind;
//     fn get_input_data_type(&self) -> DataKind;
// }

pub trait FirstStageTrait<I>: Stage {}

pub trait LastStageTrait<O>: Stage {}

pub struct StageWrapper {}

pub(crate) fn form<S: Default + Stage>() -> S {
    S::default()
}

pub(crate) fn form_first_stage<S: Default + Stage>() -> S {
    S::default()
}

pub(crate) fn form_last_stage<S: Default + Stage>() -> S {
    S::default()
}