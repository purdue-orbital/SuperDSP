use superdsp_core::Schedule::Update;
use superdsp_core::{Res, Scheduler};
use superdsp_core::stages::fsk::fsk_mod_f32;
use superdsp_core::stages::packet_detection::cross_correlation_f32;

const LEN: usize = 16;

pub fn setup(){
    
}

pub fn signal_print(signal: Res<isize>){
    println!("Signal: {}", *signal);
}

fn main() {
    let mut s = Scheduler::new();
    
    s.add_stage(Update, fsk_mod_f32);
    s.add_stage(Update, cross_correlation_f32::<LEN>);
    s.add_stage(Update, signal_print);

    s.setup();
    
    loop {
        s.run();   
    }
}