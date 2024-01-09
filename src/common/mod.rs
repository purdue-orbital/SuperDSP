use num_complex::Complex;
use rand::Rng;
use rand_distr::Normal;

pub mod generate_wave;
pub mod goertzel_algorithm;
pub mod constellation;
pub mod fftshift;
pub mod rational_resampler;

pub fn convolution(arr1: &[Complex<f32>], arr2: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut to_return = vec![Complex::new(0.0, 0.0); arr1.len() + arr2.len() - 1];

    for i in 0..arr1.len() {
        for j in 0..arr2.len() {
            to_return[i + j].re += arr1[i].re * arr2[j].re;
            to_return[i + j].im += arr1[i].im * arr2[j].im;
        }
    }

    to_return
}

/// this will take in a modulated signal and add noise to it simulating it be transmitted
pub fn wgn(data: &[Complex<f32>], snr: f32) -> Vec<Complex<f32>> {
    let snr_linear = 10.0_f32.powf(snr / 10.0);

    // Compute the noise power according to the desired SNR
    let noise_power = 1.0 / snr_linear;

    // The standard deviation of the noise is the square root of the noise power
    let noise_std_dev = noise_power.sqrt();

    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, noise_std_dev).unwrap();

    data.iter().map(|b| b + Complex::new(rng.sample(normal), rng.sample(normal))).collect()
}