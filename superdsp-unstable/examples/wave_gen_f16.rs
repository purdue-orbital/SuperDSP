#![feature(f16)]

use superdsp_core::Schedule::{Startup, Update};
use superdsp_core::stages::wave_gen::WaveGenInformation;
use superdsp_core::{DSPCore, DspInformation, Res, ResMut, Scheduler};
use superdsp_unstable::DSPCoreUnstable;
use superdsp_unstable::wave_gen::wave_gen_f16;

pub fn set_information(
    mut f16_arr: ResMut<Vec<f16>>,
    mut information: ResMut<DspInformation>,
    mut settings: ResMut<WaveGenInformation>,
) {
    information.gain = 10.0;
    information.carrier_frequency = 440.0;
    information.sample_rate = 44100.0;
    information.taps = 10;

    *settings = information.create_wave_gen_settings();

    *f16_arr = vec![0.0_f16; information.taps];
}

fn print_wave(f16_arr: Res<Vec<f16>>) {
    for x in f16_arr.iter() {
        println!("{}", *x as f32);
    }
}

fn main() {
    let mut s = Scheduler::new();

    s.add_plugin(DSPCoreUnstable);
    s.add_plugin(DSPCore);

    s.add_resource(WaveGenInformation::default());

    s.add_stage(Startup, set_information);

    s.add_stage(Update, wave_gen_f16);
    s.add_stage(Update, print_wave);

    s.build().run();
}
