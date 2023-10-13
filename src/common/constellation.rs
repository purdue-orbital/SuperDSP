use std::f32::consts::PI;
use num_complex::Complex;
use rustfft::num_traits::Pow;
use rustfft::num_traits::real::Real;
use crate::common::generate_wave::generate_wave;

/// This is a record of one state in a constellation map
#[derive(Clone)]
pub struct ConstellationPoint {
    // This is the binary value
    value: u128,

    // I offset
    i_offset: f32,

    // Q offset
    q_offset: f32,

    // Amplitudes
    i_amplitude: f32,
    q_amplitude: f32,
}

impl ConstellationPoint{
    pub fn new(value:u128,i_offset:f32,q_offset:f32,i_amplitude:f32,q_amplitude:f32) -> ConstellationPoint {
        ConstellationPoint{
            value,
            i_offset,
            q_offset,
            i_amplitude,
            q_amplitude
        }
    }
}


/// This a constellation map of states that each map to different binary values
pub struct Constellation {
    states: Vec<ConstellationPoint>,

    // Specifications of the reference signal
    message_frequency: f32,
    sample_rate:f32,
    samples_per_a_symbol:usize
}

impl Constellation{

    /// This will create a new Constellation map. The reference signal is what will be considered
    /// 0 radians
    pub fn new(message_frequency: f32, sample_rate:f32,samples_per_a_symbol:usize) -> Constellation{
        Constellation{ states: vec![],
            message_frequency,
            sample_rate,
            samples_per_a_symbol
        }
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
                        i_amplitude: 0.0,
                        q_amplitude: 0.0,
                    }
                );
            }
        }

        // set state
        self.states[state.value as usize] = state.clone();
    }

    /// This will calculate the I and Q phase offset and return them as a tuple in that order
    pub fn calculate_phase_offset(reference_signal: &[Complex<f32>], arr: &[Complex<f32>]) -> (f32, f32){
        let mut avg_i_diff = 0.0;
        let mut avg_q_diff = 0.0;

        for (index, x) in reference_signal.iter().enumerate(){
            avg_i_diff += x.re * arr[index].re;
            avg_q_diff += x.im * arr[index].im;
        }

        avg_i_diff /= arr.len() as f32;
        avg_q_diff /= arr.len() as f32;

        (avg_i_diff.acos(),avg_q_diff.acos())
    }

    fn distance(i_0:f32,q_0:f32,i_1:f32,q_1:f32) -> f32{
        ((i_0 - i_1).pow(2) as f32 + (q_0 - q_1).pow(2) as f32).sqrt()
    }

    /// given a set of complex values, this will find the state closest to the set and return it
    pub fn evaluate(&self, arr: &[Complex<f32>]) -> Vec<u128> {
        let mut to_return = vec![0; arr.len() / self.samples_per_a_symbol];

        // create reference signal
        let r = generate_wave(
            self.message_frequency,
            self.sample_rate,
            arr.len() as i32,
            0,
            1.0,
            1.0,
            0.0,
            0.0
        );

        for y in (0..arr.len()).step_by(self.samples_per_a_symbol){
            let offset = Constellation::calculate_phase_offset(&r[y..y+self.samples_per_a_symbol], &arr[y..y+self.samples_per_a_symbol]);
            let mut closest_state = &self.states[0];

            for x in &self.states{
                if
                Constellation::distance(offset.0,offset.1,x.i_offset,x.q_offset)
                    <
                    Constellation::distance(offset.0,offset.1,closest_state.i_offset,closest_state.q_offset){
                    closest_state = x;
                }
            }

            to_return[y/self.samples_per_a_symbol] = closest_state.value;
        }


        to_return
    }

    pub fn generate(&self, arr: &[u128]) -> Vec<Complex<f32>> {
        let mut to_return = vec![];

        for (index,&x) in arr.iter().enumerate(){

            // get point
            let point = &self.states[x as usize];

            // create wave
            let wave = generate_wave(
                self.message_frequency,
                self.sample_rate,
                self.samples_per_a_symbol as i32,
                index as i32 * self.samples_per_a_symbol as i32,
                point.i_amplitude,
                point.q_amplitude,
                point.i_offset,
                point.q_offset
            );

            // add signal to return value
            to_return.extend_from_slice(wave.as_slice())
        }

        to_return
    }
}