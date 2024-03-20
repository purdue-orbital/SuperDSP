use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct ConditionalBitMatch {
    active: bool,
    delay: bool,
    reset_channel: Arc<Mutex<Receiver<()>>>,
    buffer: Vec<f32>
}

impl Element for ConditionalBitMatch {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        
        if self.delay{
            self.delay = false;
            self.active = true;
        }
        
        // if the condition is met, set active to true and wait for reset
        if !self.active && _samples.get_f32_array() == self.buffer{
            self.delay = true
        }

        if self.active && self.reset_channel.as_ref().lock().unwrap().try_recv().is_ok(){
            self.active = false;
        }
    }

    fn halt(&self) -> bool {
        true
    }
    fn stop(&self, samples: &mut ElementParameter) -> bool {
        // this will allow the pipeline to continue if a condition is found
        !self.active
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl ConditionalBitMatch {
    pub fn new(bits:&[u8]) -> (Sender<()>, ConditionalBitMatch) {
        let (tx,rx) = channel();

        (tx, ConditionalBitMatch {
            active: false,
            delay: false,
            reset_channel: Arc::new(Mutex::new(rx)),
            buffer: bits.iter().map(|&b| b as f32).collect(),
        })
    }
}