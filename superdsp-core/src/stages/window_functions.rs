extern crate libm;
use crate::PI;
use alloc::vec;
use alloc::vec::Vec;
use libm::cosf;

const BLACKMAN_HARRIS_COEFF: [f32; 4] = [0.35875, 0.48829, 0.14128, 0.01168];
const BLACKMAN_NUTALL_COEFF: [f32; 4] = [0.3635819, 0.4891775, 0.1365995, 0.0106411];

///
/// turbofish is built into rust. turbofish calculates N at compile time. N is set as a constant.
/// Its not efficient to have constant arguments in the function header, hence the modification.
///
/// Generates a triangle window of size N.
///
/// # Arguments
///
/// * `N` - A constant representing the length of the window.
/// * `num` - An index to select which value from the L array is used in the calculation.
///
pub fn triangle_window<const N: usize>(num: usize) -> Vec<f32> {
    let l: [f32; 3] = [N as f32 + 0.0, N as f32 + 1.0, N as f32 + 2.0];

    let mut w: Vec<f32> = vec![0.0; N];

    for i in 0..N {
        w[i] = 1.0 - ((i as f32 - (N as f32 / 2.0)) / (l[num] / 2.0)).abs();
    }

    w
}

/// Generates a Blackman-Nuttall window of length N.
///
/// # Arguments
///
/// * `N` - The constant size of the window to be generated.
pub fn blackman_nutall<const N: usize>() -> Vec<f32> {
    let mut w: Vec<f32> = vec![0.0; N];
    for i in 0..N {
        w[i] = BLACKMAN_NUTALL_COEFF[0]
            - BLACKMAN_NUTALL_COEFF[1] * cosf(2.0 * (PI as f32) * (i as f32) / (N as f32))
            + BLACKMAN_NUTALL_COEFF[2] * cosf(4.0 * (PI as f32) * (i as f32) / (N as f32))
            - BLACKMAN_NUTALL_COEFF[3] * cosf(6.0 * (PI as f32) * (i as f32) / (N as f32));
    }
    w
}

/// Generates a Blackman-Harris window of length N.
///
/// # Inputs:
/// - `N`: A compile-time constant representing the length of the window to be generated.
pub fn blackman_harris<const N: usize>() -> Vec<f32> {
    let mut w: Vec<f32> = vec![0.0; N];
    for i in 0..N {
        w[i] = BLACKMAN_HARRIS_COEFF[0]
            - BLACKMAN_HARRIS_COEFF[1] * cosf(2.0 * (PI as f32) * (i as f32) / (N as f32))
            + BLACKMAN_HARRIS_COEFF[2] * cosf(4.0 * (PI as f32) * (i as f32) / (N as f32))
            - BLACKMAN_HARRIS_COEFF[3] * cosf(6.0 * (PI as f32) * (i as f32) / (N as f32));
    }
    w
}

#[test]
pub fn test_triangle_window() {
    assert_eq!(triangle_window::<3>(3).len(), 3);
}
