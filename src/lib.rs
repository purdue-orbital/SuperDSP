#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod math;
pub mod objects;
mod factory;

#[cfg(feature = "gui")]
pub mod gui;
