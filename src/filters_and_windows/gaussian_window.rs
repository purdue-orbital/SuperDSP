/// Creates a gaussian filter of size N
/// Formula: w(x) = exp(-0.5 * ((x - N / 2) / (σ * N / 2))^2)
/// 
/// # Arguments
/// N: usize - Size of the filter
/// sigma: f32 - Standard deviation, controls the width of the Gaussian
/// 
/// # Returns
/// [f32; N] - Gaussian window
pub fn gaussian_window<const N: usize>(offset: f32, sigma: f32) -> [f32; N] {
    let mut filter = [0.0; N];
    let mid = N as f32 / 2.0;
    
    for x in 0..N {
        let norm_x = (x as f32 - mid - offset) / (sigma * mid);
        filter[x] = (-0.5 * norm_x * norm_x).exp();
    }
    
    filter
}
