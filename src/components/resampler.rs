use num_complex::Complex;

pub struct Resampler {
    step_size: usize,
}

impl Resampler {
    pub fn new(start_size: usize, end_size: usize) -> Resampler {
        debug_assert!(start_size >= end_size);

        Resampler {
            step_size: start_size / end_size
        }
    }

    pub fn run(&self, index: usize, samples: &[Complex<f32>]) -> Vec<Complex<f32>> {
        let mut resampled = Vec::with_capacity(samples.len() / self.step_size);

        for x in samples.iter().skip(index).step_by(self.step_size) {
            resampled.push(*x)
        }

        resampled
    }
}