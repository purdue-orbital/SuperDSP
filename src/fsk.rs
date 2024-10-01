use num_complex::Complex;
pub fn fsk_mod(carrier_freqency: u64, sample_rate: u32, bandwidth: u64, data: &[u8]) -> Vec<Complex<f32>> {
    let mut modulated = Vec::new();
    let mut phase: f32 = 0.0;
    for byte in data {
        for i in 0..8 {
            let bit = (byte >> i) & 0x01 as u8;
            let symbol = if bit == 0 as u8 { -1.0 } else { 1.0 };
            let phase_inc = 2.0 * std::f32::consts::PI * ((symbol * (bandwidth >> 1) as f32) + carrier_freqency as f32) / (sample_rate as f32);
            
            for _ in 0..8 {
                modulated.push(Complex::new(phase.cos(), phase.sin()));
                phase += phase_inc;
            }
        }
    }
    modulated
}

pub fn fsk_demod(sample_rate: u32, bandwidth: u64, modulated: &[Complex<f32>]) -> Vec<u8> {
    let mut demodulated = Vec::new();
    
    let fft_index = (bandwidth as f32 / (sample_rate as f32 / 2.0) * 7.0) as usize;
    let fft_size = 8;
    
    let fft = rustfft::FftPlanner::new().plan_fft(fft_size, rustfft::FftDirection::Forward);
    
    for i in (0..modulated.len()).step_by(fft_size) {
        if (i / 8) % 8 == 0 {
            demodulated.push(0);
        }
        
        let input = &modulated[i..i + fft_size];
        let mut input = input.to_vec();
        
        fft.process(&mut input);
        
        // check if signal is present (1) or not (0)
        let symbol = if input[fft_index].norm_sqr() > 4.0 { 0 } else { 1 };
        
        let len = demodulated.len();
        demodulated[len - 1] |= symbol << ((i/8) % 8);
    }
    
    demodulated
}