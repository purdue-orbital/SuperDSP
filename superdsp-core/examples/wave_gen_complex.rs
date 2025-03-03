use num::Complex;
use superdsp_core::{DSPCore, DspInformation, Res, ResMut, Scheduler};
use superdsp_core::Schedule::{Startup, Update};
use superdsp_core::stages::wave_gen::{wave_gen_complex_f32, wave_gen_f32, WaveGenInformation};

fn configure(mut wave_gen_information: ResMut<WaveGenInformation>, mut dsp_information: ResMut<DspInformation>, mut arr: ResMut<Vec<Complex<f32>>>) {
    dsp_information.carrier_frequency = 1000.0;
    dsp_information.gain = 10.0;
    dsp_information.sample_rate = 4000.0;

    *wave_gen_information = dsp_information.create_wave_gen_settings();

    *arr = vec![Complex::new(0.0 as f32,0.0); 16];
}

fn print_wav_gen(arr: Res<Vec<Complex<f32>>>){
    for x in arr.iter(){
        println!("{:.2} ", x);
    }
}

fn main() {
    let mut s = Scheduler::new();

    s.add_plugin(DSPCore);
    s.add_resource(WaveGenInformation::default());
    s.add_resource(vec![Complex::new(0.0 as f32, 0.0)]);

    s.add_stage(Startup, configure);

    s.add_stage(Update, wave_gen_complex_f32);
    s.add_stage(Update, print_wav_gen);

    s.setup();

    loop {
        s.run();
    }
}