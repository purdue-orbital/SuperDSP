
use rustdsp::pipeline::stages::modulation::fsk::fsk_mod;
use rustdsp::pipeline::stages::demodulation::fsk::fsk_demod;
use rustdsp::prelude::etc::duplicator::{duplicator, duplicator_i8};
use rustdsp::prelude::etc::nrz::{nrz, nrz_i8};

#[test]
fn test_fsk() {
    let data: Vec<u8> = vec![23,5,69];
    let frequency = 1000.0;
    let sample_rate = 44100.0;
    
    let nrz = nrz(data.as_slice());
    let dupped = duplicator(nrz.as_slice(), 2);
    
    let modulated = fsk_mod(dupped.as_slice(), frequency, sample_rate);
    let demodulated = fsk_demod(modulated.as_slice(), frequency, sample_rate);
    
    assert_eq!(modulated.len(), dupped.len());
    
}