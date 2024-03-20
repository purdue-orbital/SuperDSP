use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SlidingBuffer {
    buffer_len:usize,
}

impl Element for SlidingBuffer {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        assert_eq!(samples.get_f32_array().len(), 1);
        
        // Create buffer and index to fetch from
        let buffer = &ElementParameter::new_f32_array(vec![0.0;self.buffer_len].as_slice());
        let scratch = &ElementParameter::new_f32_array(vec![0.0;self.buffer_len].as_slice());
        let index = &ElementParameter::new_f32_array(&[0.0]);
        
        // create shifting indexes
        let mut shift_indexes = Vec::with_capacity(self.buffer_len);
        for x in 0..self.buffer_len{
            shift_indexes.push((x as f32 - 1.0).abs())
        }
        
        // shift buffer
        builder.fetch_f32(buffer, &ElementParameter::new_f32_array(shift_indexes.as_slice()),scratch);
        builder.copy_f32(scratch,buffer);
        
        // fetch from input
        builder.fetch_f32(samples,index,buffer);
        
        samples.set_f32_array_wrapped(buffer);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool {false}

    fn is_source(&self) -> bool {
        false
    }
}

impl SlidingBuffer {
    pub fn new(len:usize) -> SlidingBuffer {
        SlidingBuffer {
            buffer_len: len,
        }
    }
}

#[derive(Clone)]
pub struct InverseSlidingBuffer {
}

impl Element for InverseSlidingBuffer {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        let len = samples.get_f32_array().len();

        // Create buffer and index to fetch from
        let buffer = &ElementParameter::new_f32_array(vec![0.0;1].as_slice());
        let index = &ElementParameter::new_f32_array(&[0.0]);
        
        builder.fetch_f32(samples,index,buffer);
        samples.set_f32_array_wrapped(buffer);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool {false}

    fn is_source(&self) -> bool {
        false
    }
}

impl InverseSlidingBuffer {
    pub fn new() -> InverseSlidingBuffer {
        InverseSlidingBuffer {
        }
    }
}