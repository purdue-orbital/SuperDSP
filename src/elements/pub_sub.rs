use std::sync::{Arc, Mutex};

use crate::elements::element::Element;
use crate::elements::events::Event;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct PubSub {
    event: Arc<Mutex<Box<dyn Event>>>
}

impl Element for PubSub {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        true
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool {
        self.event.lock().unwrap().run(samples)
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl PubSub {
    pub fn new<T:Event + 'static>(event: T) -> PubSub {
        PubSub {
            event: Arc::new(Mutex::new(Box::new(event))),
        }
    }
}