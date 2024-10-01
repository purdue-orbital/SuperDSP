use superdsp::BladeRF;
use superdsp::fsk::{fsk_demod, fsk_mod};

#[test]
pub fn test_fsk() {
    let frequency = 100_000;
    let sample_rate = 1_000_000;
    
    let data = vec![0xFF, 0xAA, 0x55, 0xAA, 0xFF, 0xAA, 0x55, 0xAA, 0xf4, 0x0f, 0x0f, 0xf4, 0x0f, 0x0f, 0xf4, 0x0f];

    let modulated = fsk_mod(frequency, sample_rate, 500_000, &data);
    let demodulated = fsk_demod(sample_rate, 500_000, &modulated);

    assert_eq!(data, demodulated);
}