use num_complex::Complex;
use rustfft::num_traits::Pow;

/// This will recover the carrier wave and return it
pub fn multiply_divide(samples:&[Complex<f32>],pow:u16) -> Vec<Complex<f32>>{
    samples.iter().map(|b| Complex::new(b.re.pow(pow).powf(1.0/pow as f32),b.im.pow(pow).powf(1.0/pow as f32))).collect()
}