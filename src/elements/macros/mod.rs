use crate::math::builder::WorkflowBuilder;
use crate::math::objects::ElementParameter;

pub mod nco;
pub mod time_bank;
pub mod wave_generators;
pub mod dft;

pub trait Part {
    fn add_to_builder(builder: &mut WorkflowBuilder, samples: &mut ElementParameter);
} 