use num_complex::Complex;

use crate::elements::element::Element;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::point_chart::PointChart;

#[derive(Clone)]
pub struct ConstellationChart {
    boxed_chart: Option<Box<PointChart>>,

    len: usize,
}

impl ConstellationChart {
    pub fn new(len: usize) -> ConstellationChart {
        ConstellationChart {
            boxed_chart: None,
            len,
        }
    }
}

impl Element for ConstellationChart {
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {
        let chart = PointChart::new(self.len);

        self.boxed_chart = Some(win_builder.add_chart(chart));
    }

    fn run(&mut self, samples: &mut [Complex<f32>]) {
        let unwrapped = self.boxed_chart.as_mut().unwrap();

        for x in samples.iter().copied() {
            unwrapped.add(x)
        }
    }
}