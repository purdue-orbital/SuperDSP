use num_complex::Complex;
use crate::elements::macros::fir::{fir_bpf_dft};
use crate::math::builder::WorkflowBuilder;
use crate::math::prelude::*;

pub fn zero_pad(builder: &mut WorkflowBuilder, samples: &mut ComplexF32, upsample_amount:usize){
    // calculate new lengths
    let og_len = samples.get_real_array_wrapped().get_f32_array().len();
    let new_len = og_len * upsample_amount;
    
    // create array of indexes
    let mut indexes = Vec::with_capacity(new_len);
    
    // create mask
    let mut mask = Vec::with_capacity(new_len);
    
    let empty_output = vec![Complex::new(0.0,0.0);new_len];
    
    // add elements
    for x in 0..og_len{
        // add 1 to mask
        mask.push(1.0);
        
        // add index
        indexes.push(x as f32);
        
        // add redundant 
        for _ in 1..upsample_amount{
            // add 0 to mask
            mask.push(0.0);
            
            // add index
            indexes.push(x as f32)
        }
    }
    
    // put arrays into ram
    let indexes_ram = ElementParameter::new_f32_array(indexes.as_slice());
    let mask_ram = ElementParameter::new_f32_array(mask.as_slice());
    let padded_ram = ComplexF32::new(empty_output);
    
    // fetch indexes
    builder.fetch_f32(&samples.get_real_array_wrapped(),&indexes_ram,&padded_ram.get_real_array_wrapped());
    builder.fetch_f32(&samples.get_imag_array_wrapped(),&indexes_ram,&padded_ram.get_imag_array_wrapped());

    // apply mask to fetched data
    builder.pointwise_multiply_f32(&mask_ram,&padded_ram.get_real_array_wrapped());
    builder.pointwise_multiply_f32(&mask_ram,&padded_ram.get_imag_array_wrapped());
    
    // set new output
    samples.set_real_array_wrapped(&padded_ram.get_real_array_wrapped());
    samples.set_imag_array_wrapped(&padded_ram.get_imag_array_wrapped());
}

pub fn interpolate(builder: &mut WorkflowBuilder, samples:&mut ComplexF32, upsample_amount:usize, original_sample_rate:f32, roll_off:f32){
    zero_pad(builder, samples, upsample_amount);

    fir_bpf_dft(builder,samples,original_sample_rate * upsample_amount as f32,roll_off,-original_sample_rate/2.0,original_sample_rate/2.0)
}