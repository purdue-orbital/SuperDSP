use std::f32::consts::PI;
use num_complex::Complex;
use crate::common::convolution;


/// builds root raised filter. This was implemented using this paper https://engineering.purdue.edu/~ee538/SquareRootRaisedCosine.pdf
pub fn build_rrc(samples_per_a_symbol:usize, roll_off:f32, baud_rate:f32) -> Vec<f32>{
    let time_increment = 1.0 / baud_rate;
    let mut t = -1.0 *  time_increment * (samples_per_a_symbol / 2) as f32;
    let mut arr = Vec::with_capacity(samples_per_a_symbol);

    for _ in 0..samples_per_a_symbol {
        arr.push(
            // lord have mercy
            ((2.0 * roll_off) / (PI * time_increment.sqrt())) * ((((1.0 + roll_off) * PI * (t / time_increment)).cos() + (((1.0 - roll_off) * PI * (t / time_increment)).sin())) / (1.0 - (4.0 * roll_off * (t / time_increment)).powi(2)))
        );

        t += time_increment
    }

    arr
}


pub struct SymbolSync{

}

impl SymbolSync{
    /// Run symbol sync
    pub fn run(&mut self, samples: &[Complex<f32>]){

    }

    /// New symbol sync loop instance
    pub fn new(){

    }
}