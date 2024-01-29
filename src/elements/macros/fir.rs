use num_complex::Complex;
use crate::elements::macros::dft::fft_shift;
use crate::math::prelude::*;

pub fn fir_filter_dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, fir_filter: &ElementParameter){
    let samples_i_array = samples.get_real_array_wrapped();
    let samples_q_array = samples.get_imag_array_wrapped();

    let scratch = ComplexF32::new(vec![Complex::new(0.0,0.0);samples_i_array.get_f32_array().len()]);

    let scratch_i_array = scratch.get_real_array_wrapped();
    let scratch_q_array = scratch.get_imag_array_wrapped();

    builder.dft_f32(&samples_i_array,&samples_q_array,&scratch_i_array,&scratch_q_array);

    fft_shift(builder,&scratch);

    builder.pointwise_multiply_f32(fir_filter,&scratch_i_array);
    builder.pointwise_multiply_f32(fir_filter,&scratch_q_array);

    fft_shift(builder,&scratch);

    builder.idft_f32(&scratch_i_array,&scratch_q_array,&samples_i_array,&samples_q_array);
}

pub fn fir_lpf_dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, sample_rate:f32, cutoff_freq:f32, roll_off:f32){
    // get baud rate
    let sps = samples.to_vec().len();

    // get the frequency increment for each bin index
    let step_size = sample_rate / sps as f32;

    // Create lpf filter
    let mut filter = vec![1.0;sps];
    for (index,x) in filter.iter_mut().enumerate() {
        let freq = (index as f32 - ((sps >> 1) as f32)) * step_size;
        if freq > cutoff_freq{
            *x = (-(freq - cutoff_freq) * roll_off) + 1.0;

            if x.is_sign_negative(){
                *x = 0.0;
            }
        }
    }

    // add filter
    fir_filter_dft(builder,samples,&ElementParameter::new_f32_array(filter.as_slice()));
}