#![no_std]
#![feature(generic_const_exprs)]

#[cfg(feature = "std")]
extern crate std;

pub mod math;
pub mod objects;

#[cfg(feature = "gui")]
pub mod gui;
pub mod radios;
mod filters;
