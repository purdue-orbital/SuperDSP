// This function is based on the blackman-nuttal filter function from wikipedia found here: https://en.wikipedia.org/wiki/Window_function

/// Creates a blackman-nuttal filter of size N
/// Formula: f(x) = a_0 - a_1 * cos(2 * PI * (n - offset) / (N)) + a_2 * cos(4 * PI * (n - offset) / (N - 1)) - a_3 * cos(6 * PI * (n - offset) / (N - 1)) for (n - offset) in 0..N
///
/// # Arguments
/// N: usize - Size of the filter
/// offset: usize - Bin offset
///
/// # Returns
/// [f32; N] - Blackman-Nuttal window
///
use std::f32;

pub fn blackman_nuttall<const N: usize>(offset: usize) -> [f32; N] {
    let mut filter = [0.0; N];

    let a_0 = 0.3635819;
    let a_1 = 0.4891775;
    let a_2 = 0.1365995;
    let a_3 = 0.0106411;

    for n in 0..N {
        filter[n] = a_0 
        - a_1 * libm::cosf(2.0 * f32::consts::PI * (n - offset) as f32 / N as f32)
        + a_2 * libm::cosf(4.0 * f32::consts::PI * (n - offset) as f32 / (N as f32))
        - a_3 * libm::cosf(6.0 * f32::consts::PI * (n - offset) as f32 / (N as f32));
    }

    filter
}
