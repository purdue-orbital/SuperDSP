use core::f16;
use core::f32::consts::PI;
use superdsp_core::{RadioInformation, Res, ResMut};

pub fn wave_gen_f16(information: Res<RadioInformation>, mut time: ResMut<f32>, mut output: ResMut<Vec<f16>>) {
    let phi = 2.0 * PI * information.frequency / information.sample_rate;

    for x in (*output).iter_mut() {
        *x = libm::sinf(phi * *time) as f16;
        *time += 1.0 / information.sample_rate;
        *time %= information.sample_rate;
    }

}