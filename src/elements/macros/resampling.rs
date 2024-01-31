use crate::elements::macros::fir::fir_lpf_dft;
use crate::math::builder::WorkflowBuilder;
use crate::math::prelude::*;

/// Zero padding can increase frequency if done in the time domain so preform DFT and FFT shift
/// before zero pad if you want to interpolate preserving data integrity
pub fn zero_pad(builder: &mut WorkflowBuilder, samples:&mut ElementParameter, upsample_amount:usize) -> ElementParameter{
    // make array of zeros
    let zeros = vec![0.0; samples.get_f32_array().len() * upsample_amount];
    let zeros_ram = ElementParameter::new_f32_array(zeros.as_slice());

    // make scratch space
    let scratch = vec![0.0; samples.get_f32_array().len() * upsample_amount];
    let scratch_ram = ElementParameter::new_f32_array(scratch.as_slice());

    // copy data over
    builder.copy_f32(&zeros_ram,&scratch_ram);
    builder.copy_f32(samples,&scratch_ram);

    scratch_ram
}

pub fn interpolate(builder: &mut WorkflowBuilder, samples:&ElementParameter, upsample_amount:usize, original_sample_rate:f32, lpf_frequency:f32, roll_off:f32){
    let mut i_array = samples.get_complex_f32().get_real_array_wrapped();
    let mut q_array = samples.get_complex_f32().get_imag_array_wrapped();

    samples.get_complex_f32().set_real_array_wrapped(&zero_pad(builder,&mut i_array,upsample_amount));
    samples.get_complex_f32().set_imag_array_wrapped(&zero_pad(builder,&mut q_array,upsample_amount));

    fir_lpf_dft(builder, &samples.get_complex_f32(),original_sample_rate, lpf_frequency, roll_off, upsample_amount as f32);
}