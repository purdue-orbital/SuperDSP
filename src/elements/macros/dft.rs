use crate::math::prelude::*;

/// This will preform dft on a input array and send it to the output array
pub fn dft(builder: &mut WorkflowBuilder, samples:&ComplexF32, out: &ComplexF32){
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    let i_out = out.get_real_array_wrapped();
    let q_out = out.get_imag_array_wrapped();
    // preform fft
    builder.dft_f32(&i_arr, &q_arr, &i_out, &q_out);
}

/// This will preform idft on a input array and send it to the output array
#[allow(dead_code)]
pub fn idft(builder: &mut WorkflowBuilder, samples:&ComplexF32, out: &ComplexF32){
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    let i_out = out.get_real_array_wrapped();
    let q_out = out.get_imag_array_wrapped();

    // preform fft
    builder.idft_f32(&i_arr, &q_arr, &i_out, &q_out);
}