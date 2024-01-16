use std::sync::{Arc, Mutex, RwLock};

use rustdsp::math::cpu::CPUCommandBuilder;

fn main() {
    let src = Arc::new(Mutex::new(vec![3.0; 4]));
    let src2 = Arc::new(Mutex::new(vec![3.0; 4]));
    let dest = Arc::new(Mutex::new(vec![2.0; 7]));

    let var = Arc::new(RwLock::new(0.0));
    let mut builder = CPUCommandBuilder::default();

    builder.convolution_f32(src, src2, dest.clone());
    builder.scalar_multiply_f32(dest.clone(), var);

    let mut pipeline = builder.build();

    pipeline.run();

    dbg!(dest.lock().unwrap().as_mut_slice());
}