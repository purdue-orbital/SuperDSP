//! Finite Impulse Response (FIR) is the reading and processing data of finite length.
//! Radio signals can be interpreted as finite and therefore FIR filters are very helpful. We could
//! also choose to read signals of a radio indefinitely and make changes and adjustments on the fly.
//! This is called Infinite Impulse Response (IIR). If this is the case, we would need a IIR filter
//! instead.

use std::sync::Arc;

use num_complex::Complex;
use rustfft::{Fft, FftPlanner};
use crate::common;

use crate::filters::fir::shapes::WindowShapes;

pub mod shapes;

/// This filtering method uses "window functions" to remove data or frequencies we don't want.
pub struct Windowing {
    window: Vec<Complex<f32>>,
    forward_fft: Arc<dyn Fft<f32>>,
    inverse_fft: Arc<dyn Fft<f32>>,
    scratch_space: Vec<Complex<f32>>,
}

impl Windowing {
    /// Generate a new windowing object
    pub fn new(window_shape: WindowShapes, fft_size: usize, alpha: f32) -> Windowing {
        // generate window
        let window = shapes::generate_shape(window_shape, fft_size, alpha);

        // create wave settings
        let mut fft: FftPlanner<f32> = FftPlanner::new();
        let forward = fft.plan_fft_forward(fft_size);
        let reverse = fft.plan_fft_inverse(fft_size);

        Windowing { window, forward_fft: forward, inverse_fft: reverse, scratch_space: vec![Complex::new(0.0, 0.0); fft_size] }
    }

    /// run/apply the filter onto a given set of data in place
    pub fn run(&mut self, arr: &mut [Complex<f32>]) {

        // preform fft
        self.forward_fft.process_with_scratch(arr, self.scratch_space.as_mut_slice());

        // reorder fft
        common::fftshift::split_reverse(arr);

        // apply filter and normalization
        for (index, x) in self.window.iter().enumerate() {
            arr[index] = Complex::new(arr[index].re * x.re, arr[index].im * x.im) / self.scratch_space.len() as f32;
        }

        // normalize one more time so its within the domain [-1,1]
        let max = unsafe { arr.iter().max_by(|x, y| x.norm().total_cmp(&y.norm())).unwrap_unchecked()};
        let max_deref = *max;

        for x in &mut *arr {
            x.re /= max_deref.re;
            x.im /= max_deref.im;
        }

        // reorder back in order
        common::fftshift::split_reverse(arr);

        // preform inverse operation
        self.inverse_fft.process_with_scratch(arr, self.scratch_space.as_mut_slice());
    }
}

/// This creates a rectangular shape in samples by repeating samples "num_samples" times
pub fn rectangular_pulse_shaping<T>(arr:&[T],num_samples:usize)->Vec<T>
where T: Copy
{
    let mut out = Vec::with_capacity(arr.len()*(num_samples+1));

    for x in arr{
        
        out.push(*x);
        
        for _ in 0..num_samples{
            out.push(*x)
        }
    }

    out
}