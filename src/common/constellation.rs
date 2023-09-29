use std::f32::consts::PI;
use num_complex::Complex;
use rustfft::num_traits::Pow;
use rustfft::num_traits::real::Real;

/// This is a record of one state in a constellation map
#[derive(Clone)]
pub struct ConstellationPoint {
    // This is the binary value
    value: u128,

    // I offset
    i_offset: f32,

    // Q offset
    q_offset: f32,

    // Amplitude
    amplitude: f32,
}

impl ConstellationPoint{
    pub fn new(value:u128,i_offset:f32,q_offset:f32,amplitude:f32) -> ConstellationPoint {
        ConstellationPoint{
            value,
            i_offset,
            q_offset,
            amplitude,
        }
    }
}


/// This a constellation map of states that each map to different binary values
pub struct Constellation {
    states: Vec<ConstellationPoint>,

    reference_signal: Vec<Complex<f32>>
}

impl Constellation{

    /// This will create a new Constellation map. The reference signal is what will be considered
    /// 0 radians
    pub fn new(refernce_signal: &[Complex<f32>]) -> Constellation{
        Constellation{ states: vec![], reference_signal:refernce_signal.to_vec() }
    }

    pub fn add_state(&mut self, state: &ConstellationPoint){

        // allocate size
        if self.states.len() <= state.value as usize {
            for _ in 0..(state.value - self.states.len() as u128 + 1){
                self.states.push(
                    ConstellationPoint {
                        value: 0,
                        i_offset: 0.0,
                        q_offset: 0.0,
                        amplitude: 0.0,
                    }
                );
            }
        }

        // set state
        self.states[state.value as usize] = state.clone();
    }

    /// This will calculate the I and Q phase offset and return them as a tuple in that order
    pub fn calculate_offset(&self, arr: &[Complex<f32>]) -> (f32,f32){
        let mut avg_i_diff = 0.0;
        let mut avg_q_diff = 0.0;

        for (index, x) in self.reference_signal.iter().enumerate(){
            avg_i_diff += (x.re - arr[index].re).abs();
            avg_q_diff += (x.im - arr[index].im).abs();
        }

        (PI*0.5*(avg_i_diff / arr.len() as f32), PI*0.5*(avg_q_diff / arr.len() as f32))
    }

    fn distance(i_0:f32,q_0:f32,i_1:f32,q_1:f32) -> f32{
        ((i_0 - i_1).pow(2) as f32 + (q_0 - q_1).pow(2) as f32).sqrt()
    }

    /// given a set of complex values, this will find the state closest to the set and return it
    pub fn evaluate(&self, arr: &[Complex<f32>]) -> u128 {

        let mut closest_state = &self.states[0];
        let offset = self.calculate_offset(arr);

        for x in &self.states{
            if
            Constellation::distance(offset.0,offset.1,x.i_offset,x.q_offset)
                <
            Constellation::distance(offset.0,offset.1,closest_state.i_offset,closest_state.q_offset){
                closest_state = &x;
            }
        }


        closest_state.value
    }
}