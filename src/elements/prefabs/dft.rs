use crate::math::prelude::*;

/// This will preform dft on an input array and send it to the output array
pub fn dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, out: &ComplexF32) {
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    let i_out = out.get_real_array_wrapped();
    let q_out = out.get_imag_array_wrapped();
    // preform fft
    builder.dft_f32(&i_arr, &q_arr, &i_out, &q_out);
}

/// This will preform idft on a input array and send it to the output array
pub fn idft(builder: &mut WorkflowBuilder, samples: &ComplexF32, out: &ComplexF32) {
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    let i_out = out.get_real_array_wrapped();
    let q_out = out.get_imag_array_wrapped();

    // preform fft
    builder.idft_f32(&i_arr, &q_arr, &i_out, &q_out);
}

/// This reorders the buffer, so that the center component of the dft is the dc or the center frequency of interest and
/// the ends are the nyquist frequencies
pub fn fft_shift(builder: &mut WorkflowBuilder, samples: &ComplexF32) {

    // divide by 2
    let len = samples.to_vec().len();
    let mut k = len >> 1;

    // shift index offset by 1 for odd length sizes
    if k % 2 == 1 {
        k += 1
    }

    // create index array
    let mut index_arr = vec![];
    for x in 0..len {
        index_arr.push(((x + k) % len) as f32);
    }

    // store index array in ram
    let index_array_ram = ElementParameter::new_f32_array(index_arr.as_slice());

    // make copy of same element size
    let scratch = ElementParameter::new_f32_array(index_arr.as_slice());

    // get complex values
    let i_arr = samples.get_real_array_wrapped();
    let q_arr = samples.get_imag_array_wrapped();

    // do shift on i array
    builder.fetch_f32(&i_arr, &index_array_ram, &scratch);
    builder.copy_f32(&scratch, &i_arr);


    // do shift on q array
    builder.fetch_f32(&q_arr, &index_array_ram, &scratch);
    builder.copy_f32(&scratch, &q_arr);
}