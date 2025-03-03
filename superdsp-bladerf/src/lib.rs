mod bladerf;

use num::Complex;
use superdsp_core::{Res, ResMut, Scheduler};
use crate::bladerf::BladeRf;

pub struct TxSettings{
    pub frequency: u64,
    pub sample_rate: u32,
    pub gain: i32,
    pub bandwidth: u32,
    
    pub enabled: bool,
}

pub struct RxSettings{
    pub frequency: u64,
    pub sample_rate: u32,
    pub gain: i32,
    pub bandwidth: u32,
    
    pub enabled: bool,
}

pub fn BladeRfPlugin(s: &mut Scheduler){
    s.add_resource(TxSettings{
        frequency: 0,
        sample_rate: 0,
        gain: 0,
        bandwidth: 0,
        
        enabled: false,
    });
    
    s.add_resource(RxSettings{
        frequency: 0,
        sample_rate: 0,
        gain: 0,
        bandwidth: 0,
        
        enabled: false,
    });

    s.add_resource(vec![Complex::new(0i16, 0i16); 1]);
    
    s.add_resource(BladeRf::new());
}

pub fn configure_radio(radio: ResMut<BladeRf>, tx_settings: Res<TxSettings>, rx_settings: Res<RxSettings>){
    if tx_settings.enabled {
        radio.set_frequency(tx_settings.frequency, bladerf::Channel::TX);
        radio.set_sample_rate(tx_settings.sample_rate, bladerf::Channel::TX);
        radio.set_gain(tx_settings.gain, bladerf::Channel::TX);
        radio.set_bandwidth(tx_settings.bandwidth, bladerf::Channel::TX);
        radio.sync_config(bladerf::Channel::TX);
        radio.enable_module(bladerf::Channel::TX);
    }
    
    if rx_settings.enabled {
        radio.set_frequency(rx_settings.frequency, bladerf::Channel::RX);
        radio.set_sample_rate(rx_settings.sample_rate, bladerf::Channel::RX);
        radio.set_gain(rx_settings.gain, bladerf::Channel::RX);
        radio.set_bandwidth(rx_settings.bandwidth, bladerf::Channel::RX);
        radio.sync_config(bladerf::Channel::RX);
        radio.enable_module(bladerf::Channel::RX);
    }
}

pub fn radio_source(radio: Res<BladeRf>, mut buffer: ResMut<Vec<Complex<i16>>>){
    radio.sync_rx(&mut buffer[..]);
}

pub fn radio_sink(radio: Res<BladeRf>, buffer: Res<Vec<Complex<i16>>>){
    radio.sync_tx(&buffer[..]);
}