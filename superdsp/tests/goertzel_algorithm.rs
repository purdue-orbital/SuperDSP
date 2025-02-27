use rustdsp::prelude::etc::goertzel_algorithm::goertzel_algorithm;
use rustdsp::prelude::etc::wave_gen::wave_gen;

#[test]
pub fn test_goertzel_algorithm() {
    let frequency = 1.0;
    let sample_rate = 16.0;
    let taps = 16;

    let wave = wave_gen(frequency, taps, sample_rate);
    let test = goertzel_algorithm(15, &wave, frequency, sample_rate);

    println!("Test: {:?}", test);
}