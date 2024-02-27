use crate::elements::element::Element;
use crate::math::prelude::*;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::line_chart::LineChart;

#[derive(Clone)]
pub struct TimeChart {
    boxed_chart: Option<Box<LineChart>>,

    len: usize,
}

impl TimeChart {
    pub fn new(len: usize) -> TimeChart {
        TimeChart {
            boxed_chart: None,
            len,
        }
    }
}

impl Element for TimeChart {
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {
        let chart = LineChart::new(self.len);

        self.boxed_chart = Some(win_builder.add_chart(chart));
    }

    fn init(&mut self, _builder: &mut WorkflowBuilder, _samples: &mut ElementParameter) {}

    fn run(&mut self, samples: &mut ElementParameter) {
        let unwrapped = self.boxed_chart.as_mut().unwrap();

        for x in samples.get_complex_f32().to_vec().iter() {
            unwrapped.add(*x)
        }
    }

    fn halt(&self) -> bool {
        true
    }

    fn is_source(&self) -> bool {
        false
    }
}