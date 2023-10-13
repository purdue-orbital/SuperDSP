use std::f32::consts::PI;

use rustdsp::common::constellation::{Constellation, ConstellationPoint};

#[test]
fn test_const_bpsk() {
    let message_freq = 1000.0;
    let sample_rate = 1000.0;
    let baud_rate = 100.0;

    let mut c = Constellation::new(message_freq, sample_rate, (sample_rate / baud_rate) as usize);

    let bin_zero = ConstellationPoint::new(0, 0.0, 0.0, 1.0, 0.0);
    let bin_one = ConstellationPoint::new(1, PI, 0.0, 1.0, 0.0);

    c.add_state(&bin_zero);
    c.add_state(&bin_one);

    let one_bin = [1, 1, 1, 1, 1, 1, 1];
    let zero_bin = [0, 0, 0, 0, 0, 0, 0];
    let mix_bin = [1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0];

    let one_signal = c.generate(&one_bin);
    let zero_signal = c.generate(&zero_bin);
    let mix_signal = c.generate(&mix_bin);

    let test1 = c.evaluate(one_signal.as_slice());
    let test2 = c.evaluate(zero_signal.as_slice());
    let test3 = c.evaluate(mix_signal.as_slice());

    assert_eq!(test1, one_bin);
    assert_eq!(test2, zero_bin);
    assert_eq!(test3, mix_bin);
}