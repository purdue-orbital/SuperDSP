use num_complex::Complex;

use crate::elements::prefabs::time_bank::time_bank;
use crate::math::objects::{ComplexF32, ElementParameter};
use crate::math::prelude::*;

/// This a very efficient signal generator that will produce a constant tone.
/// The frequency and phase can't be altered while in use
pub fn wave_generator_complex_time_banked(builder: &mut WorkflowBuilder, sample_rate: f32, frequency: f32, sps: usize) -> ComplexF32 {

    // produce a time bank
    let bank = time_bank(sample_rate, frequency);

    // extract time banks
    let i_bank = bank.get_real_array_wrapped();
    let q_bank = bank.get_imag_array_wrapped();

    // create indexes
    let mut indexes = vec![0.0; sps];

    let bank_size = i_bank.get_f32_array().len();

    for x in 0..sps {
        indexes[x] = (x % bank_size) as f32
    }

    // move indexes to ram
    let ram_indexes = ElementParameter::new_f32_array(indexes.as_slice());

    // set time increment and max time range
    let step = ElementParameter::new_f32(sps as f32);
    let max = ElementParameter::new_f32(2.0 * sample_rate);


    // create complex value which is returned
    let to_return = ComplexF32::new(vec![Complex::new(0.0, 0.0); sps]);

    // get i and q for ease of access
    let to_return_i = to_return.get_real_array_wrapped();
    let to_return_q = to_return.get_imag_array_wrapped();

    // get time banks
    builder.fetch_f32(&i_bank, &ram_indexes, &to_return_i); // get indices from bank, fill up the to_return with data
    builder.fetch_f32(&q_bank, &ram_indexes, &to_return_q);

    // increment time
    builder.scalar_add_f32(&ram_indexes, &step); // add the timestep to each value in the ram indices array

    // mod time
    builder.mod_f32(&ram_indexes, &max); // modulate the indices, so we don't go too far past (errors)

    to_return
}