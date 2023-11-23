use num_complex::Complex;
use rustdsp::common::generate_wave::generate_wave;
use rustdsp::encoders::nrz;
use rustdsp::loops::costas::Costas;

#[test]
fn costas(){
    // radio settings
    let samples_per_a_symbol:usize = 16;
    let sample_rate = 1e6;
    let baud_rate = sample_rate / samples_per_a_symbol as f32;
    let carrier_frequency = 1e3;

    // bpsk modulate
    let data = &[138u8,255,255,6,13];
    let encoded = nrz(data,samples_per_a_symbol);
    let carrier = generate_wave(carrier_frequency,sample_rate,encoded.len(),0,1.0,1.0,0.0,0.0);
    let modulated:Vec<Complex<f32>> = carrier.iter().enumerate().map(|(index,val)| val * encoded[index]).collect();

    // prepare costas loop
    let mut costas = Costas::new(sample_rate,carrier_frequency);

    // run loop
    let demodded:Vec<Complex<f32>> = modulated.iter().map(|b| costas.run(b)).collect();

    // down sample
    let mut down_sampled = Vec::with_capacity(demodded.len()/samples_per_a_symbol);

    for x in demodded.iter().step_by(samples_per_a_symbol){
      down_sampled.push(x.re)
    };

    dbg!(down_sampled);
}