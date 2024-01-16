use std::f32::consts::PI;
use num_complex::Complex;
use crate::math::objects::ComplexF32;
use crate::math::prelude::*;

/// This will add a NCO component to the builder. This takes in the samples per a symbol (sps) and
/// the error and adjusts the components. This will return the array where the complex values are sent
pub fn add_nco(builder: &mut WorkflowBuilder, sps:usize, sample_rate:f32, frequency:f32, frequency_error_scalar: &ElementParameter) -> ComplexF32{
    // create an array of frequencies
    let frequency_array = ElementParameter::new_f32_array(vec![frequency; sps].as_slice());

    // create 2 PI scalar
    let pi_2 = ElementParameter::new_f32(2.0 * PI);

    // create predetermined time step intervals
    let mut time_steps_rust = vec![];
    for x in 0.. sps {
        time_steps_rust.push(x as f32 / sample_rate)
    }

    // save initial time step
    let time = ElementParameter::new_f32_array(time_steps_rust.as_slice());

    // this is the workspace buffer
    let scratch = ElementParameter::new_f32_array(time_steps_rust.as_slice());

    // create time step
    let step_size = ElementParameter::new_f32(sps as f32 / sample_rate);

    // create return element
    let to_return = ComplexF32::new(vec![Complex::new(0.0,0.0); sps]);

    // get i and q arrays for ease of access
    let i_array = to_return.get_real_array_wrapped();
    let q_array = to_return.get_imag_array_wrapped();

    // start runtime operations
    //-------------------------

    // adjust frequency with error
    builder.scalar_add_f32(&frequency_array, frequency_error_scalar);

    // copy phi to workspace
    builder.copy_f32(&time, &scratch);

    // multiply time with frequency
    builder.pointwise_multiply_f32(&frequency_array,&scratch);

    // multiply phi with 2 PI
    builder.scalar_multiply_f32(&scratch, &pi_2);

    // copy scratch space to i and q array
    builder.copy_f32(&scratch, &i_array);
    builder.copy_f32(&scratch, &q_array);

    // preform sin and cos on i and q arrays
    builder.cos_f32(&i_array);
    builder.sin_f32(&q_array);

    // advance time step
    builder.scalar_add_f32(&time, &step_size);

    // mod phi to prevent value explosion
    builder.mod_f32(&time, &ElementParameter::new_f32(2.0 * PI));

    to_return
}