use rustdsp::prelude::demodulation::fsk::fsk_demod;
use rustdsp::prelude::etc::duplicator::duplicator;
use rustdsp::prelude::etc::nrz::nrz;
use rustdsp::prelude::modulation::fsk::fsk_mod;

#[test]
fn test_fsk() {
    let data: Vec<u8> = vec![23, 5, 69];
    let frequency = 1000.0;
    let sample_rate = 44100.0;

    let nrz = nrz(data.as_slice());
    let dupped = duplicator(nrz.as_slice(), 8);

    let modulated = fsk_mod(dupped.as_slice(), frequency, sample_rate);
    let demodulated = fsk_demod(modulated.as_slice(), frequency, sample_rate, 8, 0.5);

    println!("{:?}", demodulated);
}