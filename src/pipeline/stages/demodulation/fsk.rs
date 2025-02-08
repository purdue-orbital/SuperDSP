use num_complex::Complex;

pub fn fsk_demod(taps: &[Complex<f32>], frequency: f32, sample_rate: f32) -> Vec<f32> {
    let mut demodulated_data = Vec::new();
    for tap in taps {
        demodulated_data.push(tap.arg());
    }
    demodulated_data
    
}