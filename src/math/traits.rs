use crate::math::builder::Workflow;
use crate::math::objects::ElementParameter;

pub trait PlatformSpecificOperations {
    fn build(&mut self) -> Workflow;
    fn pointwise_multiply_f32(&mut self, src: &ElementParameter, dest: &ElementParameter);
    fn convolution_f32(&mut self, src1: &ElementParameter, src2: &ElementParameter, dest: &mut ElementParameter);
    fn scalar_multiply_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter);
    fn sin_f32(&mut self, src: &ElementParameter);
    fn cos_f32(&mut self, src: &ElementParameter);
    fn mod_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter);
    fn add_f32(&mut self, src: &ElementParameter, dest: &ElementParameter);
    fn scalar_add_f32(&mut self, src: &ElementParameter, scalar: &ElementParameter);
    fn copy_f32(&mut self, src: &ElementParameter, dest: &ElementParameter);
}