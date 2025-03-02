use std::ops::Deref;
use std::sync::{Arc, Mutex};
use superdsp_core::{Frequency, Gain, Res, ResMut, Scheduler, Stage};

fn example1() {
    println!("Hello, world!");
}

fn example2(frequency: Res<Frequency>, mut gain: ResMut<Gain>) {
    *gain += 1;
    
    println!("Frequency: {}, Gain: {}", *frequency, *gain);
}

fn main() {
    
    let mut s = Scheduler::new();
    
    s.add_resource(0.0f32);
    s.add_resource(32u32);
    
    s.add_stage(example1);
    s.add_stage(example2);
    
    loop{
        s.run();
    }
}