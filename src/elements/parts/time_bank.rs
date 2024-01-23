use std::f32::consts::PI;

use num_complex::Complex;

use crate::math::prelude::*;

/// This generates a time bank at a frequency which is saved instances of at what time a sample is
/// generated. This saves a lot of time if you are only generating signal without any changes
pub fn time_bank(sample_rate: f32, frequency: f32) -> ComplexF32 {
    // create time banks
    let mut i_bank = vec![0.0; 2 * sample_rate as usize];
    let mut q_bank = vec![0.0; 2 * sample_rate as usize];

    for x in 0..((2.0 * sample_rate) as usize) {
        i_bank[x] = (2.0 * PI * (x as f32 / sample_rate) * frequency).cos();
        q_bank[x] = (2.0 * PI * (x as f32 / sample_rate) * frequency).sin();
    }

    // move time banks to ram
    let ram_i_bank = ElementParameter::new_f32_array(i_bank.as_slice());
    let ram_q_bank = ElementParameter::new_f32_array(q_bank.as_slice());

    // create a complex array for ease of return
    let mut arr = ComplexF32::new(vec![Complex::new(0.0, 0.0)]);
    arr.set_real_array_wrapped(&ram_i_bank);
    arr.set_imag_array_wrapped(&ram_q_bank);

    arr
}