use std::sync::Arc;
use num_complex::Complex;
use rustfft::Fft;

use crate::elements::element::Element;
use crate::math::builder::WorkflowBuilder;
use crate::math::objects::ElementParameter;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::pixel_chart::PixelChart;

#[derive(Clone)]
pub struct WaterfallChart {
    boxed_chart: Option<Box<PixelChart>>,
    len: usize,
    fft: Arc<dyn Fft<f32>>,
}

impl WaterfallChart {
    pub fn new(len: usize) -> WaterfallChart {
        let mut planner = rustfft::FftPlanner::new();
        let fft = planner.plan_fft_forward(len);

        WaterfallChart {
            boxed_chart: None,
            len,
            fft
        }
    }
}

impl Element for WaterfallChart {
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {
        let chart = PixelChart::new(self.len, 100);

        self.boxed_chart = Some(win_builder.add_chart(chart));
    }

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {}

    fn run(&mut self, samples: &ElementParameter) {

        // get chart
        let unwrapped = self.boxed_chart.as_mut().unwrap();

        // make a copy (as fft is done in place)
        let mut samples_clone: Vec<Complex<f32>> = samples.get_complex_f32().to_vec();

        // fft
        self.fft.process(samples_clone.as_mut_slice());

        // divide by 2
        let mut k = samples_clone.len() >> 1;

        // preform fft shift
        if samples_clone.len() % 2 == 1{
            k += 1
        }

        samples_clone.rotate_right(k);


        // send fft to pixel chart
        for x in samples_clone {

            // we only need the real component as the imaginary component is just phase data
            let normalized = (((x.norm_sqr().sqrt()) / self.len as f32) * 255.0) as u8;

            unwrapped.add(normalized,0,255 - normalized);
        }
    }

    fn halt(&self) -> bool {
        true
    }

    fn is_source(&self) -> bool {
        false
    }
}