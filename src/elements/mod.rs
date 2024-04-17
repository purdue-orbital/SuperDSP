use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::elements::element::Element;

mod element;


pub struct Pipeline<T,Y> {
    pipe: Vec<dyn Element<T,Y>>
}

impl<T,Y>  Pipeline<T,Y> {
    pub async fn run(&mut self){
        loop {
            for _ in self.pipe{
            }
        }
    }
}

pub struct PipelineBuilder<T,Y> {
    ops: Vec<dyn Element<T, Y>>
}

impl<T,Y>  PipelineBuilder<T,Y> {
    pub fn new() -> PipelineBuilder<T,Y> {
        PipelineBuilder{ ops: Vec::new() }
    }
    
    pub fn add(&mut self, elem: Box<dyn Element<T, Y>>){
        self.ops.push(elem);
    }
    
    pub fn build(&self) -> Pipeline<T,Y> { Pipeline{ pipe: Vec::new() }}
}
