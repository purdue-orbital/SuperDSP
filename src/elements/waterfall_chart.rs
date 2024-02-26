use num_complex::Complex;

use crate::elements::element::Element;
use crate::elements::macros::dft::{dft, fft_shift};
use crate::math::builder::WorkflowBuilder;
use crate::math::prelude::*;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::pixel_chart::PixelChart;

#[derive(Clone)]
pub struct WaterfallChart {
    boxed_chart: Option<Box<PixelChart>>,
    arr: Option<ComplexF32>,
    len: Option<usize>,
}

impl WaterfallChart {
    pub fn new() -> WaterfallChart {
        WaterfallChart {
            boxed_chart: None,
            arr: None,
            len: None,
        }
    }
}

impl Element for WaterfallChart {
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {
        let chart = PixelChart::new(self.len.unwrap(), 100);

        self.boxed_chart = Some(win_builder.add_chart(chart));
    }

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        self.len = Some(samples.get_complex_f32().to_vec().len());
        self.arr = Some(ComplexF32::new(vec![Complex::new(0.0, 0.0); self.len.unwrap()]));
        dft(builder, &samples.get_complex_f32(), self.arr.as_ref().unwrap());
        fft_shift(builder, self.arr.as_ref().unwrap());
    }

    fn run(&mut self, _samples: &ElementParameter) {
        // send dft to pixel chart
        for x in self.arr.as_ref().unwrap().to_vec() {
            // we only need the real component as the imaginary component is just phase data
            let normalized = ((x.norm_sqr().sqrt() / self.len.unwrap() as f32) * 255.0) as u8;

            self.boxed_chart.as_mut().unwrap().add(normalized, 0, 255 - normalized);
        }
    }

    fn halt(&self) -> bool {
        true
    }

    fn is_source(&self) -> bool {
        false
    }
}