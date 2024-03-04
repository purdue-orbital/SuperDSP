use crate::elements::element::Element;
use crate::math::builder::WorkflowBuilder;
use crate::math::objects::ElementParameter;
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

    fn init(&mut self, _builder: &mut WorkflowBuilder, _samples: &mut ElementParameter) {}

    fn run(&mut self, samples: &mut ElementParameter) {
        let unwrapped = self.boxed_chart.as_mut().unwrap();

        for x in samples.get_complex_f32().to_vec().iter().copied() {
            unwrapped.add(x)
        }
    }

    fn halt(&self) -> bool {
        false
    }

    fn stop(&self, samples: &mut ElementParameter) -> bool { false }
    fn is_source(&self) -> bool {
        false
    }
}