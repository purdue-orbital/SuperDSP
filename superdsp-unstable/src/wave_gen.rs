use core::f16;
use core::f32::consts::PI;
use num::Complex;
use superdsp_core::ResMut;
use superdsp_core::stages::wave_gen::WaveGenInformation;

pub fn wave_gen_f16(mut settings: ResMut<WaveGenInformation>, mut output: ResMut<Vec<f16>>) {
    for x in output.iter_mut() {
        *x = libm::sinf(settings.c_radians) as f16;
        
        settings.c_radians += settings.radians_a_sample;
        settings.c_radians %= 2.0 * PI;
    }
}

pub fn wave_gen_complex_f16(mut settings: ResMut<WaveGenInformation>, mut output: ResMut<Vec<Complex<f16>>>) {
    for x in output.iter_mut() {
        *x = Complex::new(settings.c_radians.cos() as f16, settings.c_radians.sin() as f16);
        
        settings.c_radians += settings.radians_a_sample;
        settings.c_radians %= 2.0 * PI;
    }
}