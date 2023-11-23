//! DSP requires many loops that feed in on themselves

pub mod costas;
mod symbol_sync;

use num_complex::{Complex, ComplexFloat};

// pub struct Loops{
//     early_late_index: usize,
//     samples_per_symbol: usize,
// }
//
// impl Loops{
//
//     /// This prepares loops for operation
//     pub fn new(samples_per_symbol:usize)-> Loops{
//         Loops{
//             early_late_index: samples_per_symbol/2,
//             samples_per_symbol,
//         }
//     }
//
//     /// This is a early late gate that will sync out of sync symbols. This also down-samples to one
//     /// sample per a symbol
//     pub fn symbol_sync_early_late(&mut self, data: &[Complex<f32>]) -> Vec<Complex<f32>>{
//
//         let mut to_return = Vec::with_capacity(data.len()/self.samples_per_symbol);
//
//         for index in (0..data.len() - self.samples_per_symbol).step_by(10){
//             // early branch
//             let early = data[index+self.early_late_index.saturating_sub(1)];
//
//             // late branch
//             let late = data[index +
//                 if self.early_late_index + 1 > self.samples_per_symbol{
//                     self.samples_per_symbol - 1
//                 }else {
//                     self.early_late_index + 1
//                 }
//             ];
//
//             let diff = late.abs() - early.abs();
//
//             self.early_late_index += diff.round() as usize;
//
//             dbg!(diff);
//
//             to_return.push(data[self.early_late_index+index]);
//         }
//
//         to_return
//     }
// }