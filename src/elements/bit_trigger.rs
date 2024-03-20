use std::collections::VecDeque;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct BitTrigger {
    rx: Arc<Mutex<Receiver<Vec<u8>>>>,
    
    // This determines if the loop is currently waiting to take in data or parsing date
    intake_mode: bool,
    
    buffer: VecDeque<f32>,
}

impl Element for BitTrigger {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        if self.intake_mode{
            let input = self.rx.lock().unwrap().recv().unwrap();
            
            // break up bytes to bits
            for mut x in input {
                for _ in 0..8{
                    self.buffer.push_back((x & 1) as f32);
                    x >>= 1;
                }
            }
            
            self.intake_mode = false;
        }
        
        _samples.set_f32_array(&[self.buffer.pop_front().unwrap()]);
        
        self.intake_mode = self.buffer.is_empty();
    }

    fn halt(&self) -> bool {
        true
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool { false }
    fn is_source(&self) -> bool {
        true
    }
}

impl BitTrigger {
    pub fn new() -> (BitTrigger, Sender<Vec<u8>>) {
        let (tx, rx) = mpsc::channel();

        (BitTrigger {
            rx: Arc::new(Mutex::new(rx)),
            intake_mode: true,
            buffer: VecDeque::new()
        }, tx)
    }
}
