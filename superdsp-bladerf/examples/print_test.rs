use superdsp_bladerf::{configure_radio, radio_source, BladeRfPlugin, RxSettings, TxSettings};
use superdsp_core::{RadioCore, Res, ResMut, Scheduler};
use superdsp_core::Schedule::{Startup, Update};


fn set_radio_information(mut rx_settings: ResMut<RxSettings>, mut tx_settings: ResMut<TxSettings>, mut buffer: ResMut<Vec<i16>>){
    tx_settings.frequency = 915_000_000;
    tx_settings.sample_rate = 1_000_000;
    tx_settings.gain = 10;
    tx_settings.bandwidth = 1_000_000;
    tx_settings.enabled = false;
    
    rx_settings.frequency = 915_000_000;
    rx_settings.sample_rate = 1_000_000;
    rx_settings.gain = 10;
    rx_settings.bandwidth = 1_000_000;
    rx_settings.enabled = true;
    
    *buffer = vec![0.0 as i16; 32];
}

fn print_wave(buffer: Res<Vec<i16>>) {
    for x in buffer.iter() {
        println!("{}", *x as f32);
    }
}

fn main() {
    let mut s = Scheduler::new();

    s.add_plugin(RadioCore);
    s.add_plugin(BladeRfPlugin);
    
    // set the radio settings
    s.add_stage(Startup, set_radio_information);
    
    // configure the radio
    s.add_stage(Startup, configure_radio);
    
    // Radio source
    s.add_stage(Update, radio_source);
    
    // print the wave
    s.add_stage(Update, print_wave);
    
    s.setup();

    loop{
        s.run();
    }
}