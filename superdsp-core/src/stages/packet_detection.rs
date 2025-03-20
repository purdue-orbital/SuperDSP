use crate::ResMut;
use alloc::vec::Vec;
use core::ops::Mul;
use nalgebra::DMatrix;
use num::Complex;

pub const CCSDS_PREAMBLE: u16 = 0b1110101110010000;

pub struct PacketDetectionSettingsF32 {
    pub matrix: DMatrix<Complex<f32>>,

    pub threshold: Complex<f32>,

    pub buffer: DMatrix<Complex<f32>>,
}

/// Computes the cross-correlation of the input vector using the provided settings.
///
/// # Arguments
///
/// * `input` - A mutable reference to a vector of complex floating-point numbers representing the input signal.
/// * `settings` - A mutable reference to packet detection settings containing the correlation matrix and threshold.
pub fn cross_correlation_f32(
    mut input: ResMut<Vec<Complex<f32>>>,
    mut settings: ResMut<PacketDetectionSettingsF32>,
) {
    settings.buffer = DMatrix::from_vec(1, input.len(), input.as_slice().to_vec());

    let out = (&settings.buffer).mul(&settings.matrix);

    *input = out.as_slice().to_vec();
}
