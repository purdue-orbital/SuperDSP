use crate::{Res, ResMut};
use alloc::vec::Vec;
use core::f32::consts::PI;
use num::Complex;

#[derive(Default)]
pub struct WaveGenInformation {
    pub c_radians: f32,
    pub radians_a_sample: f32,
}

/// Generates a sine wave and stores it in the output vector.
///
/// # Arguments
/// * `settings` - A mutable reference to the wave generation settings, which includes the current radians (`c_radians`) and the increment per sample (`radians_a_sample`).
/// * `output` - A mutable reference to a vector where the generated sine values will be stored.
pub fn wave_gen_f32(mut settings: ResMut<WaveGenInformation>, mut output: ResMut<Vec<f32>>) {
    for x in output.iter_mut() {
        *x = libm::sinf(settings.c_radians);

        settings.c_radians += settings.radians_a_sample;
        settings.c_radians %= 2.0 * PI;
    }
}

/// Generates a complex sine wave and stores it in the output vector.
///
/// # Arguments
/// * `settings` - A mutable reference to the wave generation settings, which includes the current radians (`c_radians`) and the increment per sample (`radians_a_sample`).
/// * `output` - A mutable reference to a vector where the generated complex sine values will be stored.
pub fn wave_gen_complex_f32(
    mut settings: ResMut<WaveGenInformation>,
    mut output: ResMut<Vec<Complex<f32>>>,
) {
    for x in output.iter_mut() {
        x.re = libm::cosf(settings.c_radians);
        x.im = libm::sinf(settings.c_radians);

        settings.c_radians += settings.radians_a_sample;
        settings.c_radians %= 2.0 * PI;
    }
}
