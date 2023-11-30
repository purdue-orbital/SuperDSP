use num_complex::Complex;

pub struct TimingErrorDetection{
    samples_per_a_symbol:usize
}

impl TimingErrorDetection{
    pub fn new(samples_per_a_symbol:usize) -> TimingErrorDetection{
        TimingErrorDetection{
            samples_per_a_symbol
        }
    }

    /// This is a early/late gate approach to Timing error
    pub fn early_late_gate(&self, index: usize, samples: &[Complex<f32>]) -> Complex<f32>{
        assert_eq!(samples.len(),self.samples_per_a_symbol);

        // early index
        let early_index = if index == 0{
            index
        } else {
            index - 1
        };

        // late index
        let late_index = if index == self.samples_per_a_symbol - 1{
            index
        }else {
            index + 1
        };

        samples[index] * (samples[late_index] - samples[early_index])
    }
}