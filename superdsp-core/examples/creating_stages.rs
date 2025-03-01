use std::sync::{Arc, Mutex};
use superdsp_core::{Frequency, Gain, Stage};

fn example1(){
    println!("Hello, world!");
}

fn example2(frequency: Frequency, gain: Arc<Mutex<Gain>>){
    *gain.lock().unwrap() += 1;
    println!("Frequency: {}, Gain: {}", frequency, gain.lock().unwrap());
}

fn main() {
    let mut p = (440.0, Arc::new(Mutex::new(3)));
    
    example1.invoke(());
    
    example2.invoke(p.clone());
    example2.invoke(p.clone());
    example2.invoke(p.clone());
    example2.invoke(p.clone());
    example2.invoke(p);
}