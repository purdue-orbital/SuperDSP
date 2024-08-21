use core::cmp::PartialEq;
use std::println;
use num::Complex;
use spin::RwLock;
use crate::objects::{COMPLEX_OUTPUT_BUFFER_INDEX, COMPLEX_OUTPUT_BUFFERS, F64_OUTPUT_BUFFER_INDEX, F64_OUTPUT_BUFFERS};

#[derive(Clone,Copy,PartialEq)]
pub enum Type {
    NONE,
    F64,
    Complex,
}

#[derive(Clone,Copy)]
pub struct Bus<'a>{
    pub bust_type: Type,

    pub buffer_f64: Option<&'a RwLock<f64>>,
    pub buffer_complex: Option<&'a RwLock<Complex<f64>>>,

    subscribers: [Option<*mut dyn DSPObject>; 64],
    subscriber_index: usize,
}

impl Bus<'_> {
    pub fn new() -> Bus<'static> {
        Bus {
            bust_type: Type::NONE,
            buffer_f64: None,
            buffer_complex: None,
            subscribers: [None; 64],
            subscriber_index: 0,
        }
    }

    pub fn new_f64() -> Bus<'static> {

        let mut locked = F64_OUTPUT_BUFFER_INDEX.lock();

        let bus = Bus {
            bust_type: Type::F64,
            buffer_f64: Some(&F64_OUTPUT_BUFFERS[*locked]),
            buffer_complex: None,
            subscribers: [None; 64],
            subscriber_index: 0,
        };

        *locked = *locked + 1;

        bus
    }

    pub fn new_complex() -> Bus<'static> {

        let mut locked = COMPLEX_OUTPUT_BUFFER_INDEX.lock();

        let bus = Bus {
            bust_type: Type::Complex,
            buffer_f64: None,
            buffer_complex: Some(&COMPLEX_OUTPUT_BUFFERS[*locked]),
            subscribers: [None; 64],
            subscriber_index: 0,
        };

        *locked = *locked + 1;

        bus
    }
}

unsafe impl Send for Bus<'_> {}
unsafe impl Sync for Bus<'_> {}

// Single threaded implementation of the bus
impl Bus<'_> {
    pub fn trigger_f64(&self, value: f64) {
        debug_assert!(self.bust_type == Type::F64);

        if let Some(buffer) = self.buffer_f64 {
            *buffer.write() = value;
        }

        self.run_subscribers();
    }

    pub fn trigger_complex(&self, value: Complex<f64>) {
        debug_assert!(self.bust_type == Type::Complex);

        if let Some(buffer) = self.buffer_complex {
            *buffer.write() = value;
        }

        self.run_subscribers();
    }

    fn run_subscribers(&self) {
        for i in 0..self.subscriber_index {
            unsafe { self.subscribers[i].unwrap().as_mut().unwrap().process() };
        }
    }

    pub fn subscribe(&mut self, subscriber: *mut dyn DSPObject) {
        self.subscribers[self.subscriber_index] = Some(subscriber);
        self.subscriber_index += 1;
    }

}

pub trait DSPObject: DSPObjectClonable + Send + Sync {
    fn return_type(&self) -> Type;
    fn input_type(&self) -> Type;
    fn get_bus(&mut self) -> &mut Bus<'static>;
    fn set_bus(&mut self, bus: &mut Bus<'static>);
    fn process(&mut self);
    fn start(&mut self);
}

pub trait DSPObjectClonable {
    fn clone_box(&self) -> &dyn DSPObject;
}

impl<T> DSPObjectClonable for T
where
    T: 'static + DSPObject + Clone,
{
    fn clone_box(&self) -> &dyn DSPObject {
        self
    }
}
