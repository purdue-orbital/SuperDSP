use std::sync::mpsc::{channel, Receiver, Sender};
use num_complex::Complex;
use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct DataBucketComplex {
    len: usize,
    buffer: Vec<Complex<f32>>
}

impl Element for DataBucketComplex {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
    }

    fn run(&mut self, _samples: &mut ElementParameter) {

        if self.len == self.buffer.len() {
            self.buffer = Vec::with_capacity(self.buffer.len());
        }
        
        self.buffer.extend_from_slice(_samples.get_complex_f32().to_vec().as_slice());
    }

    fn halt(&self) -> bool {
        true
    }
    fn stop(&self, samples: &mut ElementParameter) -> bool { 
        if self.len <= self.buffer.len() {
            // if the bucket size has been reached, return and continue the pipeline
            samples.set_complex_f32(ComplexF32::new(self.buffer.clone()));
        }

        self.len > self.buffer.len()
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl DataBucketComplex {
    pub fn new(bucket_trigger_len: usize) -> DataBucketComplex {
        DataBucketComplex {
            len: bucket_trigger_len,
            buffer: Vec::with_capacity(bucket_trigger_len),
        }
    }
}


#[derive(Clone)]
pub struct DataBucket {
    len: usize,
    buffer: Vec<f32>,
    tx: Sender<Vec<f32>>,
}

impl Element for DataBucket {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        
        self.buffer.extend_from_slice(_samples.get_f32_array().to_vec().as_slice());
        
        if self.len <= self.buffer.len() {
            self.buffer.reverse();
            self.tx.send(self.buffer.clone()).unwrap();
            self.buffer = Vec::with_capacity(self.buffer.len());
            dbg!("reset!");
        }
    }

    fn halt(&self) -> bool {
        true
    }
    fn stop(&self, samples: &mut ElementParameter) -> bool {
        self.len > self.buffer.len()
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl DataBucket {
    pub fn new(bucket_trigger_len: usize) -> (Receiver<Vec<f32>>, DataBucket) {
        
        let (tx, rx) = channel();

        (rx, DataBucket {
            len: bucket_trigger_len,
            buffer: Vec::with_capacity(bucket_trigger_len),
            tx 
        })
    }
}