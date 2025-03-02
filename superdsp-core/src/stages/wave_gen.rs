use core::f32::consts::PI;

pub fn wave_gen(frequency: f32, sample_rate: f32, time: &mut f32, output: &mut [f32]) {
    let phi = 2.0 * PI * frequency / sample_rate;

    for x in output.iter_mut() {
        *x = libm::sinf(phi * *time);
        *time += 1.0 / sample_rate;
    }
}