use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct DataTrigger {
    rx: Arc<Mutex<Receiver<Vec<f32>>>>,
}


impl Element for DataTrigger {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        // wait till the trigger has been tripped
        _samples.set_f32_array(self.rx.lock().unwrap().recv().unwrap().as_slice());
    }

    fn halt(&self) -> bool {
        true
    }

    fn is_source(&self) -> bool {
        true
    }
}

impl DataTrigger {
    pub fn new() -> (DataTrigger, Sender<Vec<f32>>) {
        let (tx, rx) = mpsc::channel();

        (DataTrigger {
            rx: Arc::new(Mutex::new(rx))
        }, tx)
    }
}