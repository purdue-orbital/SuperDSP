use nalgebra::Complex;
use superdsp_core::Schedule::{Startup, Update};
use superdsp_core::{DSPCore, DspInformation, Res, ResMut, Scheduler};
use superdsp_core::stages::fsk::{fsk_mod_f32, FSKSettings};
use superdsp_core::stages::packet_detection::{cross_correlation_f32, PacketDetectionSettingsF32};
use superdsp_core::stages::packing::{unpack};

const LEN: usize = 16;

pub fn setup(mut arr: ResMut<Vec<Complex<f32>>>, mut byte: ResMut<u8>, mut dsp_information: ResMut<DspInformation>, mut fsksettings: ResMut<FSKSettings>, mut pds: ResMut<PacketDetectionSettingsF32<LEN>>) {
    dsp_information.carrier_frequency = 1000.0;
    dsp_information.gain = 10f32;
    dsp_information.sample_rate = 2000.0;
    dsp_information.taps = LEN;
    
    *arr = vec![Complex::new(0.0, 0.0); LEN];

    *fsksettings = dsp_information.create_fsk_settings();
    *pds = fsksettings.create_packet_detection_settings(&[0b00001100], LEN);
}

pub fn reset(mut bytes: ResMut<Vec<u8>>) {
    *bytes = vec![0b11001100,0b10100001];
}

pub fn signal_print(signal: Res<isize>){
    println!("Signal: {}", *signal);
}

fn main() {
    let mut s = Scheduler::new();
    
    s.add_plugin(DSPCore::<LEN>);
    
    s.add_stage(Startup, setup);
    
    s.add_stage(Update, unpack::<1>);
    s.add_stage(Update, fsk_mod_f32);
    s.add_stage(Update, cross_correlation_f32::<LEN>);
    s.add_stage(Update, signal_print);

    s.setup();
    
    loop {
        s.run();   
    }
}