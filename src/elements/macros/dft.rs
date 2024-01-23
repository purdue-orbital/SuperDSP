use num_complex::Complex;
use crate::math::prelude::*;

/// This will preform dft on a input array and send it to the output array
pub fn dft(builder: &mut WorkflowBuilder, samples:&ComplexF32, out: &ComplexF32, len:usize){
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    let zero_arr = ComplexF32::new(vec![Complex::new(0.0, 0.0); len]);
    let i_zero = zero_arr.get_real_array_wrapped();
    let q_zero = zero_arr.get_imag_array_wrapped();

    let i_out = out.get_real_array_wrapped();
    let q_out = out.get_imag_array_wrapped();

    // reset
    builder.copy_f32(&i_zero, &i_out);
    builder.copy_f32(&q_zero, &q_out);

    // preform fft
    builder.dft_f32(&i_arr, &q_arr, &i_out, &q_out);
}

/// This will preform idft on a input array and send it to the output array
#[allow(dead_code)]
pub fn idft(builder: &mut WorkflowBuilder, samples:&ComplexF32, out: &ComplexF32, len:usize){
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    let zero_arr = ComplexF32::new(vec![Complex::new(0.0, 0.0); len]);
    let i_zero = zero_arr.get_real_array_wrapped();
    let q_zero = zero_arr.get_imag_array_wrapped();

    let i_out = out.get_real_array_wrapped();
    let q_out = out.get_imag_array_wrapped();

    // reset
    builder.copy_f32(&i_zero, &i_out);
    builder.copy_f32(&q_zero, &q_out);

    // preform fft
    builder.idft_f32(&i_arr, &q_arr, &i_out, &q_out);
}