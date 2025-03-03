use alloc::vec::Vec;
use core::f32::consts::PI;
use num::Complex;
use crate::{Res, ResMut};

#[derive(Default)]
pub struct WaveGenInformation {
    pub c_radians: f32,
    pub radians_a_sample: f32,
}

pub fn wave_gen_f32(mut settings: ResMut<WaveGenInformation>, mut output: ResMut<Vec<f32>>) {
    for x in output.iter_mut() {
        *x = libm::sinf(settings.c_radians);
        
        settings.c_radians += settings.radians_a_sample;
        settings.c_radians %= 2.0 * PI;
    }
}

pub fn wave_gen_complex_f32(mut settings: ResMut<WaveGenInformation>, mut output: ResMut<Vec<Complex<f32>>>) {
    for x in output.iter_mut() {
        x.re = libm::cosf(settings.c_radians);
        x.im = libm::sinf(settings.c_radians);
        
        settings.c_radians += settings.radians_a_sample;
        settings.c_radians %= 2.0 * PI;
    }
}