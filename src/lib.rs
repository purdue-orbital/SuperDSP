//! Digital Signal Processing (DSP) a widely studied field in communications and is involves
//! taking digital samples from a radio or microphone and preforming sets of operations on those
//! samples with a computer.

pub mod ecc;
#[cfg(feature = "ui")]
pub mod ui;
pub mod elements;
pub mod math;
