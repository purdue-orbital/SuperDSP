use crate::cpu::pipeline::stages::etc::goertzel_algorithm::goertzel_algorithm;
use num_complex::Complex;

pub fn fsk_demod(taps: &[Complex<f32>], carrier_frequency: f32, sample_rate: f32, sps: usize, threshold: f32) -> Vec<u8> {
    let index = ((carrier_frequency / sample_rate) * sps as f32) as usize;

    println!("Index: {}", index);

    let mut output = Vec::new();

    for x in 0..(taps.len() / sps) {
        output.push(if goertzel_algorithm(index, &taps[x * sps..(x + 1) * sps], carrier_frequency, sample_rate).norm() > threshold { 1 } else { 0 });
    }

    output
}