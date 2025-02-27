use num_complex::Complex;

fn first_stage(n: isize, samples: &[Complex<f32>], w0: f32) -> Complex<f32> {
    if n < 0 {
        return Complex::new(0.0, 0.0);
    }

    samples[n as usize] + 2.0 * (w0).cos() * first_stage(n - 1, samples, w0) - first_stage(n - 2, samples, w0)
}

pub fn goertzel_algorithm(n: usize, samples: &[Complex<f32>], frequency: f32, sample_rate: f32) -> Complex<f32> {
    let w0 = 2.0 * std::f32::consts::PI * frequency / sample_rate;

    first_stage(n as isize, samples, w0) - (Complex::new(-w0.cos(), -w0.sin()) * first_stage(n as isize - 1, samples, w0))
}