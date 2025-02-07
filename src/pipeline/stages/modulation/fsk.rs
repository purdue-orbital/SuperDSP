use num_complex::Complex;

// Requires data to be nrz encoded as f32
pub fn fsk_mod(data: &[f32], carrier_frequency: f32, sample_rate: f32) -> Vec<Complex<f32>> {
    let mut modulated_data = Vec::new();
    let mut phase = 0.0;
    let phase_increment = 2.0 * std::f32::consts::PI * carrier_frequency / sample_rate;
    for sample in data {
        let phi = phase * sample;
        modulated_data.push(Complex::new(phi.cos(), phi.sin()));

        phase += phase_increment;
    }
    modulated_data
}