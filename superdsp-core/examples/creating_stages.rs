use std::ops::Deref;
use std::sync::{Arc, Mutex};
use superdsp_core::{Frequency, Gain, Res, ResMut, Scheduler, Stage};

pub struct Test{
    pub frequency: f32,
    pub gain: f32,
}

fn example1() {
    println!("Hello, world!");
}

fn example2(test: Res<Test>) {
    let f = test.frequency;
    let gain = test.gain;
    
    
    println!("Frequency: {}, Gain: {}", f, gain);
}

fn main() {
    let mut s = Scheduler::new();
    
    s.add_resource(Test{
        frequency: 300.0,
        gain: 1.0,
    });
    
    s.add_stage(example1);
    s.add_stage(example2);
    
    loop{
        s.run();
    }
}