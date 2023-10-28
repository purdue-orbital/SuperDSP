use num_complex::Complex;

pub mod generate_wave;
pub mod bi_signal_generation;
pub mod bi_signal_demodulation;
pub mod goertzel_algorithm;
pub mod noise_generators;
pub mod constellation;
pub mod fftshift;
pub mod rational_resampler;

pub fn convolution(arr1:&[Complex<f32>],arr2:&[Complex<f32>]) -> Vec<Complex<f32>>{
    let mut to_return = vec![Complex::new(0.0,0.0);arr1.len()+arr2.len()-1];
    
    for i in 0..arr1.len(){
        for j in 0..arr2.len(){
            to_return[i+j].re += arr1[i].re * arr2[j].re;
            to_return[i+j].im += arr1[i].im * arr2[j].im;
        }
    }
    
    to_return
}