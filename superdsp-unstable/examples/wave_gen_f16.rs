#![feature(f16)]

use superdsp_core::Schedule::{Startup, Update};
use superdsp_core::{RadioCore, RadioInformation, Res, ResMut, Scheduler};
use superdsp_unstable::RadioCoreUnstable;
use superdsp_unstable::wave_gen::wave_gen_f16;

pub fn set_information(mut f16_arr: ResMut<Vec<f16>>, mut information: ResMut<RadioInformation>) {
    information.gain = 10.0;
    information.frequency = 440.0;
    information.sample_rate = 44100.0;
    information.taps = 10;
    
    *f16_arr = vec![0.0_f16; information.taps];
}

fn print_wave(f16_arr: Res<Vec<f16>>) {
    for x in f16_arr.iter() {
        println!("{}", *x as f32);
    }
}

fn main() {
    let mut s = Scheduler::new();
    
    s.add_plugin(RadioCoreUnstable);
    s.add_plugin(RadioCore);
    
    s.add_stage(Startup, set_information);
    
    s.add_stage(Update, wave_gen_f16);
    s.add_stage(Update, print_wave);
    
    s.setup();
    
    loop{
        s.run();
    }
}