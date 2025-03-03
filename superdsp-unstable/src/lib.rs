#![feature(f16)]
extern crate core;

use core::f16;
use superdsp_core::Scheduler;

pub mod wave_gen;

pub fn DSPCoreUnstable(s: &mut Scheduler) {
    s.add_resource(vec![0.0 as f16; 1]);
    s.add_resource(0.0 as f32);
}