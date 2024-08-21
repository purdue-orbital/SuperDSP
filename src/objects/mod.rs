#[cfg(feature = "gui")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "gui")]
use std::thread::spawn;

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(feature = "std")]
use std::boxed::Box;

use spin::RwLock;
use crate::gui::{DSPChart, GUI, Message};
use crate::objects::object::{DSPObject, DSPObjectClonable};

pub mod object;
pub mod wave_gen;

#[cfg(feature = "std")]
pub mod wave_gen_time;

#[cfg(feature = "std")]
pub mod wave_gen_time_complex;
pub mod wave_gen_complex;

pub(crate) static F64_OUTPUT_BUFFERS: [RwLock<f64>; 64] = [
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
];
pub(crate) static COMPLEX_OUTPUT_BUFFERS: [RwLock<num::Complex<f64>>; 64] = [
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),RwLock::new(num::Complex::new(0.0, 0.0)),
];

pub(crate) static F64_OUTPUT_BUFFER_INDEX: spin::Mutex<usize> = spin::Mutex::new(0);
pub(crate) static COMPLEX_OUTPUT_BUFFER_INDEX: spin::Mutex<usize> = spin::Mutex::new(0);


#[cfg(feature = "gui")]
pub struct GUIExecutor{
    arr: Vec<Box<dyn DSPChart<Message=Message, State=()>>>,
    first: Box<dyn DSPObject>
}

#[cfg(feature = "gui")]
impl GUIExecutor{
    pub fn run(arr: Vec<Box<dyn DSPChart<Message=Message, State=()>>>, mut first_element: Box<dyn DSPObject>) {
        spawn(move || {
            first_element.start()
        });
        
        let gui = GUI{
            width: 800,
            height: 600,
            elements: arr
        };
        gui.start();
    }
}
