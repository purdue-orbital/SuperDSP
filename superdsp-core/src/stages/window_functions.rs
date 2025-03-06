extern crate libm;
use crate::PI;
use alloc::vec;
use alloc::vec::Vec;
use libm::cosf;

///
/// turbofish is built into rust. turbofish calculates N at compile time. N is set as a constant.
/// Its not efficient to have constant arguments in the function header, hence the modification.
///
fn triangle_window<const N: usize>(num: usize) -> Vec<f32> {

    let L: [f32; 3] = [N as f32 + 0.0, N as f32 + 1.0, N as f32 + 2.0];

    let mut w: Vec<f32> = vec![0.0; N];
    
    for i in 0..N {
        w[i] = 1.0 - ((i as f32  - (N as f32 / 2.0)) / (L[num] / 2.0)).abs();
    }

    w
}

fn blackman_nutall<const N: usize>() -> Vec<f32> {
    let a: [f32; 4] = [0.3635819, 0.4891775, 0.1365995, 0.0106411];
    let mut w: Vec<f32> = vec![0.0; N];
    for i in 0..N {
        w[i] = a[0] - a[1] * cosf(2.0 * (PI as f32) * (i as f32)/(N as f32)) + a[2] * cosf(4.0 * (PI as f32) * (i as f32)/(N as f32)) - a[3] * cosf(2.0 * (PI as f32) * (i as f32)/(N as f32));
    }
    w
}

fn blackman_harris<const N: usize>() -> Vec<f32> {
    let a: [f32; 4] = [0.35875, 0.48829, 0.14128, 0.01168];
    let mut w: Vec<f32> = vec![0.0; N];
    for i in 0..N {
        w[i] = a[0] - a[1] * cosf(2.0 * (PI as f32) * (i as f32)/(N as f32)) + a[2] * cosf(4.0 * (PI as f32) * (i as f32)/(N as f32)) - a[3] * cosf(2.0 * (PI as f32) * (i as f32)/(N as f32));
    }
    w
}

#[test]
fn test_triangle_window() {
    assert_eq!(triangle_window::<3>(3).len(), 3);
}