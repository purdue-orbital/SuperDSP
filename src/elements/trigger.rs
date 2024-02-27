use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct Trigger {
    rx: Arc<Mutex<Receiver<()>>>,
}


impl Element for Trigger {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        // wait till trigger has been tripped
        self.rx.lock().unwrap().recv().unwrap()
    }

    fn halt(&self) -> bool {
        true
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl Trigger {
    pub fn new() -> (Trigger, Sender<()>) {
        let (tx, rx) = mpsc::channel();

        (Trigger {
            rx: Arc::new(Mutex::new(rx))
        }, tx)
    }
}