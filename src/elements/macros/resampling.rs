use crate::elements::macros::fir::fir_lpf_dft;
use crate::math::builder::WorkflowBuilder;
use crate::math::prelude::*;

/// Zero padding can increase frequency if done in the time domain so preform DFT and FFT shift
/// before zero pad if you want to interpolate preserving data integrity
pub fn zero_pad(builder: &mut WorkflowBuilder, samples: &mut ComplexF32, upsample_amount:usize){
    // make array of zeros
    let len = samples.get_real_array_wrapped().get_f32_array().len();
    let zeros = vec![0.0; len * upsample_amount];
    let zeros_ram = ElementParameter::new_f32_array(zeros.as_slice());

    // make scratch space
    let scratch = vec![0.0; len * upsample_amount];
    let scratch_ram_i = ElementParameter::new_f32_array(scratch.as_slice());
    let scratch_ram_q = ElementParameter::new_f32_array(scratch.as_slice());

    // copy data over
    builder.copy_f32(&zeros_ram,&scratch_ram_i);
    builder.copy_f32(&zeros_ram,&scratch_ram_q);
    builder.copy_f32(&samples.get_real_array_wrapped(),&scratch_ram_i);
    builder.copy_f32(&samples.get_real_array_wrapped(),&scratch_ram_q);

    samples.set_real_array_wrapped(&scratch_ram_i);
    samples.set_imag_array_wrapped(&scratch_ram_q);
}

pub fn interpolate(builder: &mut WorkflowBuilder, samples:&mut ComplexF32, upsample_amount:usize, original_sample_rate:f32, lpf_frequency:f32, roll_off:f32){
    zero_pad(builder, samples, upsample_amount);

    dbg!(samples.get_real_array_wrapped().get_f32_array().len());

    fir_lpf_dft(builder, samples, original_sample_rate, lpf_frequency, roll_off, upsample_amount as f32);
    
    dbg!(samples.get_real_array_wrapped().get_f32_array().len());
}