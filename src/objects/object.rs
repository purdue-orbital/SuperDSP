use core::cmp::PartialEq;

use num::Complex;
use spin::barrier::Barrier;
use spin::RwLock;

#[cfg(feature = "multithreading-std")]
use crate::objects::{BARRIERS, BARRIERS_INDEX};
use crate::objects::{COMPLEX_OUTPUT_BUFFER_INDEX, COMPLEX_OUTPUT_BUFFERS, F32_OUTPUT_BUFFER_INDEX, F32_OUTPUT_BUFFERS};

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    NONE,
    F32,
    Complex,
}

#[derive(Clone, Copy)]
pub struct Bus<'a> {
    pub bust_type: Type,

    pub buffer_f32: Option<&'a RwLock<f32>>,
    pub buffer_complex: Option<&'a RwLock<Complex<f32>>>,

    subscribers: [Option<*mut dyn DSPObject>; 64],
    subscriber_index: usize,

    #[cfg(feature = "multithreading-std")]
    pub barrier: Option<&'a Barrier>,
}

#[cfg(not(feature = "multithreading-std"))]
fn increment_barrier() {}

#[cfg(feature = "multithreading-std")]
fn increment_barrier() {
    *BARRIERS_INDEX.lock() += 1;
}

impl Bus<'_> {
    pub fn new() -> Bus<'static> {
        Bus {
            bust_type: Type::NONE,
            buffer_f32: None,
            buffer_complex: None,
            subscribers: [None; 64],
            subscriber_index: 0,

            #[cfg(feature = "multithreading-std")]
            barrier: None,
        }
    }

    pub fn new_f32() -> Bus<'static> {
        let mut locked = F32_OUTPUT_BUFFER_INDEX.lock();

        let bus = Bus {
            bust_type: Type::F32,
            buffer_f32: Some(&F32_OUTPUT_BUFFERS[*locked]),
            buffer_complex: None,
            subscribers: [None; 64],
            subscriber_index: 0,

            #[cfg(feature = "multithreading-std")]
            barrier: Some(&BARRIERS[*BARRIERS_INDEX.lock()]),
        };

        *locked += 1;

        increment_barrier();

        bus
    }

    pub fn new_complex() -> Bus<'static> {
        let mut locked = COMPLEX_OUTPUT_BUFFER_INDEX.lock();

        let bus = Bus {
            bust_type: Type::Complex,
            buffer_f32: None,
            buffer_complex: Some(&COMPLEX_OUTPUT_BUFFERS[*locked]),
            subscribers: [None; 64],
            subscriber_index: 0,

            #[cfg(feature = "multithreading-std")]
            barrier: Some(&BARRIERS[*BARRIERS_INDEX.lock()]),
        };

        *locked += 1;

        increment_barrier();

        bus
    }
}

unsafe impl Send for Bus<'_> {}

unsafe impl Sync for Bus<'_> {}


impl Bus<'_> {
    pub fn trigger_f32(&self, value: f32) {
        debug_assert!(self.bust_type == Type::F32);

        if let Some(buffer) = self.buffer_f32 {
            *buffer.write() = value;
        }

        self.run_subscribers();
    }

    pub fn trigger_complex(&self, value: Complex<f32>) {
        debug_assert!(self.bust_type == Type::Complex);

        if let Some(buffer) = self.buffer_complex {
            *buffer.write() = value;
        }

        self.run_subscribers();
    }


    pub fn subscribe(&mut self, subscriber: *mut dyn DSPObject) {
        self.subscribers[self.subscriber_index] = Some(subscriber);

        self.subscriber_index += 1;
    }
}

impl Bus<'_> {
    fn run_subscribers(&self) {
        for i in 0..self.subscriber_index {
            unsafe { self.subscribers[i].unwrap_unchecked().as_mut().unwrap_unchecked().process() };
        }
    }
}

pub trait DSPObject: Send + Sync + DSPObjectClonable {
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
