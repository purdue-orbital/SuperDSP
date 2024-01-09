use num_complex::Complex;

use crate::components::interpolator::Interpolator;
use crate::components::match_filter::MatchFilter;
use crate::components::pi_loop::PILoop;
use crate::components::resampler::Resampler;
use crate::components::timing_error_detection::TimingErrorDetection;

// /// builds root raised filter. This was implemented using this paper https://engineering.purdue.edu/~ee538/SquareRootRaisedCosine.pdf
// pub fn build_rrc(samples_per_a_symbol:usize, roll_off:f32, baud_rate:f32) -> Vec<f32>{
//     let time_increment = 1.0 / baud_rate;
//     let mut t = -1.0 *  time_increment * (samples_per_a_symbol / 2) as f32;
//     let mut arr = Vec::with_capacity(samples_per_a_symbol);
//
//     for _ in 0..samples_per_a_symbol {
//         arr.push(
//             // lord have mercy
//             ((2.0 * roll_off) / (PI * time_increment.sqrt())) * ((((1.0 + roll_off) * PI * (t / time_increment)).cos() + (((1.0 - roll_off) * PI * (t / time_increment)).sin())) / (1.0 - (4.0 * roll_off * (t / time_increment)).powi(2)))
//         );
//
//         t += time_increment
//     }
//
//     arr
// }


pub struct SymbolSync {
    samples_per_a_symbol: usize,

    sample_index: f32,

    resampler: Resampler,
    pi_filter: PILoop,
    ted: TimingErrorDetection,
    match_filter: MatchFilter,
    interpolator: Interpolator,
}

impl SymbolSync {
    /// Run symbol sync. Will fail if samples.len != samples per a symbol
    pub fn run(&mut self, samples: &[Complex<f32>]) -> Complex<f32> {
        debug_assert_eq!(samples.len(), self.samples_per_a_symbol);

        // preform sync
        let matched = self.match_filter.run(samples);
        let resampled = self.resampler.run(self.sample_index as usize, matched.as_slice());

        // preform error adjustment
        let error = self.ted.early_late_gate(self.sample_index as usize, samples);
        let error_filtered = self.pi_filter.run(error);
        self.sample_index += error_filtered.re;

        resampled[0]
    }

    /// New symbol sync loop instance
    pub fn new(samples_per_a_symbol: usize, pulse_shape: &[Complex<f32>]) -> SymbolSync {
        debug_assert_eq!(samples_per_a_symbol, pulse_shape.len());

        SymbolSync {
            samples_per_a_symbol,

            resampler: Resampler::new(samples_per_a_symbol, 1),
            pi_filter: PILoop::new(1.0, 1.0),
            ted: TimingErrorDetection::new(samples_per_a_symbol),
            match_filter: MatchFilter::new(samples_per_a_symbol, pulse_shape),
            interpolator: Interpolator::new(),

            sample_index: 0.0,
        }
    }
}