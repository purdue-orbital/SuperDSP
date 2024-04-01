use std::f32::consts::PI;
use num_complex::Complex;


/// Credit to GNU Radio for the following implementation of the phase detector
/// link: https://github.com/gnuradio/gnuradio/blob/main/gr-analog/lib/pll_carriertracking_cc_impl.h#L36
pub fn mod_2pi(input: f32) -> f32
{
    if (input > PI){
        return input - (2.0 * PI);
    }
    else if (input < -PI){
        return input + (2.0 * PI);
    }
    else{
        return input;
    }
}

pub fn phase_detector(sample: Complex<f32>, ref_sample: Complex<f32>) -> f32 {
    mod_2pi(sample.im.atan2(sample.re) - ref_sample.im.atan2(ref_sample.re))
}
