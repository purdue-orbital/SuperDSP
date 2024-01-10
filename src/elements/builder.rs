use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;

use num_complex::Complex;

use crate::elements::element::Element;


#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

pub struct Pipeline {
    sps: usize,


    #[cfg(feature = "ui")]
    window: WindowBuilder,

    elements: Vec<Box<dyn Element>>,

    rx: Arc<Mutex<Receiver<Vec<Complex<f32>>>>>,
    tx: Sender<Vec<Complex<f32>>>,
}

impl Pipeline {
    /// ***THIS WILL HALT THE THREAD AND IT MUST BE THE MAIN THREAD***
    pub fn run(&mut self) {
        let mut elements: Vec<Box<dyn Element>> = self.elements.iter().map(|val| val.clone_box()).collect();
        let rx = self.rx.clone();
        let tx = self.tx.clone();

        spawn(move || {
            loop {
                let mut mtu = rx.lock().unwrap().recv().unwrap();

                for x in elements.as_mut_slice() {
                    x.run(mtu.as_mut_slice())
                }

                tx.send(mtu).unwrap()
            }
        });


        #[cfg(feature = "ui")]
        self.window.build()
    }
}


pub struct PipelineBuilder {
    elements: Vec<Box<dyn Element>>,
}

impl PipelineBuilder {
    pub fn new() -> PipelineBuilder {
        PipelineBuilder {
            elements: vec![]
        }
    }

    pub fn add<T: Element>(&mut self, element: T) -> Box<T> {
        let b = Box::new(element);

        self.elements.push(b.clone_box());

        b
    }

    /// This will setup pipeline to run creating a sender, receiver, and pipeline
    pub fn build(&mut self, sps: usize) -> (Sender<Vec<Complex<f32>>>, Receiver<Vec<Complex<f32>>>, Pipeline) {
        let (main_tx, thread_rx) = mpsc::channel();
        let (thread_tx, main_rx) = mpsc::channel();

        let mut program = Pipeline {
            sps,

            #[cfg(feature = "ui")]
            window: WindowBuilder::new(),
            elements: vec![],

            tx: thread_tx,
            rx: Arc::new(Mutex::new(thread_rx)),
        };

        for x in self.elements.as_mut_slice() {

            #[cfg(feature = "ui")]
            x.init(&mut program.window);

            program.elements.push(x.clone_box())
        }


        (main_tx, main_rx, program)
    }
}