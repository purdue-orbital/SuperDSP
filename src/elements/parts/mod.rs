use crate::math::builder::WorkflowBuilder;
use crate::math::objects::ElementParameter;

pub mod nco;

pub trait Part{
    fn add_to_builder(builder: &mut WorkflowBuilder, samples: &mut ElementParameter);
} 