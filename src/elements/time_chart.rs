use std::sync::{Arc, RwLock};
use std::thread::spawn;
use num_complex::Complex;
use crate::elements::element::Element;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::line_chart::LineChart;

#[derive(Clone)]
pub struct TimeChart{
    boxed_chart: Option<Box<LineChart>>,

    len:usize
}

impl TimeChart{
    pub fn new(len:usize) -> TimeChart{

        let mut win_builder = WindowBuilder::new();

        TimeChart{
            boxed_chart: None,
            len,
        }
    }
}

impl Element for TimeChart{
    fn init(&mut self, win_builder: &mut WindowBuilder) {

        let chart = LineChart::new(self.len);

        self.boxed_chart =  Some(win_builder.add_chart(chart));
    }

    fn run(&mut self, samples: &mut [Complex<f32>]) {

        let unwrapped = self.boxed_chart.as_mut().unwrap();

        for x in samples.iter().copied(){
            unwrapped.add(x)
        }
    }
}