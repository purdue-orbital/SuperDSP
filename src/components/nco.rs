//! Voltage Controlled Oscillator (NCO) is a major component in the construction of loops. Since
//! voltage doesn't exist in software, we create a impression of this with a fake voltage to adjust
//! the oscillation. A positive voltage increases the

use std::f32::consts::PI;
use num_complex::Complex;

pub struct NCO {
    sample_rate: f32,
    phi: f32,
    time_passed: f32,
}

impl NCO{
    /// Creation of new NCO instance
    pub fn new(
        sample_rate: f32,
        carrier_frequency: f32) -> NCO{

        let phi = 2.0 * PI * carrier_frequency;

        NCO{
            sample_rate,
            phi,
            time_passed: 0.0
        }
    }

    /// this runs the loop once
    pub fn run(&mut self, offset:f32) -> Complex<f32>{
        // create I and Q values
        let i = (self.phi * self.time_passed + offset).cos();
        let q = (self.phi * self.time_passed + offset).sin();

        // adjust time
        self.time_passed += 1.0/self.sample_rate;
        self.time_passed %= 2.0 * PI;

        Complex::new(i,q)
    }
}