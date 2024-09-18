// This function is based on the sin filter function from wikipedia found here: https://en.wikipedia.org/wiki/Window_function
/// Creates a triangle filter of size N
/// Formula: f(x) = sinf(pi*(n-o)/N) for n in 0..N
/// 
/// Where:
/// o is the bin offset
/// 
/// # Arguments
/// N: usize - Size of the filter
/// offset: usize - Bin offset
/// 
/// # Returns
/// [f32; N] - Sin window
/// 
use core::f32::consts::PI;

pub fn sin_window<const N: usize>(offset: usize) -> [f32; N] {
    let mut filter = [0.0; N];

    for n in 0..N {
        filter[n] = (PI*(n-offset) as f32 /N as f32).sin();
    }
    
    filter
}
