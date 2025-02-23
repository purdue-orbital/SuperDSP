use num_complex::Complex;

pub fn wave_gen(frequency: f32, taps: usize, sample_rate: f32) -> Vec<Complex<f32>> {
    let mut output = Vec::new();
    let phi = 2.0 * std::f32::consts::PI * frequency / sample_rate;
    for x in 0..taps {
        output.push(Complex::new((phi * x as f32).cos(), (phi * x as f32).sin()));
    }
    output
}