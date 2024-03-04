use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use num_complex::Complex;

use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct CodeSinkF32Array {
    tx: Arc<Mutex<Sender<Vec<f32>>>>,
}


impl Element for CodeSinkF32Array {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        self.tx.lock().unwrap().send(_samples.get_f32_array()).unwrap()
    }

    fn halt(&self) -> bool {
        true
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool { false }
    fn is_source(&self) -> bool {
        true
    }
}

impl CodeSinkF32Array {
    pub fn new() -> (CodeSinkF32Array, Receiver<Vec<f32>>) {
        let (tx, rx) = mpsc::channel();

        (CodeSinkF32Array {
            tx: Arc::new(Mutex::new(tx))
        }, rx)
    }
}

#[derive(Clone)]
pub struct CodeSinkComplexF32 {
    tx: Arc<Mutex<Sender<Vec<Complex<f32>>>>>,
}


impl Element for CodeSinkComplexF32 {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, _samples: &mut ElementParameter) {
        self.tx.lock().unwrap().send(_samples.get_complex_f32().to_vec()).unwrap()
    }

    fn halt(&self) -> bool {
        true
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool { false }
    fn is_source(&self) -> bool {
        true
    }
}

impl CodeSinkComplexF32 {
    pub fn new() -> (CodeSinkComplexF32, Receiver<Vec<Complex<f32>>>) {
        let (tx, rx) = mpsc::channel();

        (CodeSinkComplexF32 {
            tx: Arc::new(Mutex::new(tx))
        }, rx)
    }
}
