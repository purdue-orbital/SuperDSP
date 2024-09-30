// This function is based on the triangle filter function from wikipedia found here: https://en.wikipedia.org/wiki/Window_function

/// Creates a triangle filter of size N
/// Formula: f(x) = 1 - |((n - o) - (N / 2)) / (L / 2)| for n in 0..N
///
/// Where:
/// L can be N, N+1, or N+2
/// o is the bin offset
///
/// # Arguments
/// N: usize - Size of the filter
/// l: usize - Filter setting (N, N+1, or N+2)
/// offset: usize - Bin offset
///
/// # Returns
/// [f32; N] - Triangle window
pub fn triangle_window<const N: usize>(l: usize, offset: usize) -> [f32; N] {
    let mut filter = [0.0; N];

    for n in 0..N {
        filter[n] = 1.0 - (((n - offset) as f32 - (N as f32 / 2.0)) / (l as f32 / 2.0)).abs();
    }

    filter
}