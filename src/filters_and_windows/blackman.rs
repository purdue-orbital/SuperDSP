// This function is based on the triangle filter function from wikipedia found here: https://en.wikipedia.org/wiki/Window_function

/// Creates a blackman filter of size N
/// Formula: f(x) = 0.42 - (1/2) cos((2*pi*n)/N) + 0.08 cos(4*pi*n)/N for n in 0..N
///
/// Where:
/// o is the bin offset
///
/// # Arguments
/// N: usize - Size of the filter
/// l: usize - Filter setting (N, N+1, or N+2)
/// offset: usize - Bin offset
///
/// # Returns
/// [f32; N] - Blackman window
pub fn blackman_window<const N: usize>(offset: usize) -> [f32; N] {
    let mut filter = [0.0; N];
    let a0 = 0.42;
    let a1 = 0.5;
    let a2 = 0.08;

    for n in 0..N {
        filter[n] = a0 - a1 * f32::cos((2.0 * std::f32::consts::PI * ((n - offset) as f32) ) / (N as f32))
            + a2 * f32::cos((4.0 * std::f32::consts::PI * ((n - offset) as f32)) / (N as f32));
    }

    filter
}