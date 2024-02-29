use num_complex::Complex;
use crate::elements::element::Element;
use crate::elements::prefabs::dft::{dft, fft_shift};
use crate::elements::prefabs::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct FrequencyDemodulation {
    sps: usize,
    carrier_frequency: f32,
    sample_rate: f32,
    threshold: f32
}

impl Element for FrequencyDemodulation {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        assert_eq!(samples.get_complex_f32().to_vec().len(),self.sps);
        
        let samples_copy = ComplexF32::new(vec![Complex::new(0.0,0.0);self.sps]);
        
        let dft_out = ComplexF32::new(vec![Complex::new(0.0,0.0);self.sps]);
        
        let one = &ElementParameter::new_f32(1.0);
        let half = &ElementParameter::new_f32(0.5);

        // calculate index
        let mut index = ((self.sps as f32 / 2.0) + ((self.carrier_frequency * self.sps as f32) / (self.sample_rate))) as usize;
        if index >= self.sps{
            index = self.sps - 1;
        }

        let index_ram = &ElementParameter::new_f32_array(&[index as f32]);
        
        let threshold = &ElementParameter::new_f32(-self.threshold);
        
        // preform dft
        dft(builder,&samples.get_complex_f32(),&dft_out);

        // fft shift
        fft_shift(builder,&dft_out);
        
        // compare with a threshold
        builder.scalar_add_f32(&dft_out.get_real_array_wrapped(),threshold);
        builder.scalar_add_f32(&dft_out.get_imag_array_wrapped(),threshold);
        
        // make copy
        builder.copy_f32(&dft_out.get_real_array_wrapped(),&samples_copy.get_real_array_wrapped());
        builder.copy_f32(&dft_out.get_imag_array_wrapped(),&samples_copy.get_imag_array_wrapped());

        // force the numbers be between 1 or 0
        builder.pointwise_multiply_f32(&dft_out.get_real_array_wrapped(),&samples_copy.get_real_array_wrapped());
        builder.pointwise_multiply_f32(&dft_out.get_imag_array_wrapped(),&samples_copy.get_imag_array_wrapped());
        
        builder.sqrt_f32(&samples_copy.get_real_array_wrapped());
        builder.sqrt_f32(&samples_copy.get_imag_array_wrapped());

        builder.pointwise_divide_f32(&dft_out.get_real_array_wrapped(),&samples_copy.get_real_array_wrapped());
        builder.pointwise_divide_f32(&dft_out.get_imag_array_wrapped(),&samples_copy.get_imag_array_wrapped());

        builder.scalar_add_f32(&samples_copy.get_real_array_wrapped(),one);
        builder.scalar_add_f32(&samples_copy.get_imag_array_wrapped(),one);
        
        builder.scalar_multiply_f32(&samples_copy.get_real_array_wrapped(), half);
        builder.scalar_multiply_f32(&samples_copy.get_imag_array_wrapped(), half);

        // return bit
        builder.fetch_f32(&samples_copy.get_real_array_wrapped(),index_ram,samples);
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl FrequencyDemodulation {
    pub fn new(sps: usize, carrier_frequency: f32, sample_rate: f32, threshold:f32) -> FrequencyDemodulation {
        FrequencyDemodulation {
            sps,
            carrier_frequency,
            sample_rate,
            threshold
        }
    }
}