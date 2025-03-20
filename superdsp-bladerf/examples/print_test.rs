use num::Complex;
use superdsp_bladerf::{BladeRfPlugin, RxSettings, TxSettings, configure_radio, radio_source};
use superdsp_core::Schedule::{Startup, Update};
use superdsp_core::{DSPCore, Res, ResMut, Scheduler};

fn set_radio_information(
    mut rx_settings: ResMut<RxSettings>,
    mut tx_settings: ResMut<TxSettings>,
    mut buffer: ResMut<Vec<Complex<i16>>>,
) {
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

    *buffer = vec![Complex::new(0i16, 0i16); 32];
}

fn print_wave(buffer: Res<Vec<Complex<i16>>>) {
    for x in buffer.iter() {
        println!("{}", x);
    }
}

fn main() {
    let mut s = Scheduler::new();

    s.add_plugin(DSPCore);
    s.add_plugin(BladeRfPlugin);

    // set the radio settings
    s.add_stage(Startup, set_radio_information);

    // configure the radio
    s.add_stage(Startup, configure_radio);

    // Radio source
    s.add_stage(Update, radio_source);

    // print the wave
    s.add_stage(Update, print_wave);

    s.build().run();
}
