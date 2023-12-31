use std::sync::{Arc, RwLock};
use std::thread::spawn;
use num_complex::Complex;
use crate::elements::element::Element;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::line_chart::LineChart;

#[derive(Clone)]
pub struct TimeChart{
    win_builder: Arc<RwLock<WindowBuilder>>,

    boxed_chart: Box<LineChart>,
}

impl TimeChart{
    pub fn new(len:usize) -> TimeChart{

        let mut win_builder = WindowBuilder::new();

        let chart = LineChart::new(len);

        let boxed = win_builder.add_chart(chart);

        TimeChart{
            win_builder: Arc::new(RwLock::new(win_builder)),

            boxed_chart: boxed
        }
    }
}

impl Element for TimeChart{
    fn init(&mut self) {

        let ui = self.win_builder.clone();
        spawn(move ||{
            ui.read().unwrap().build()
        });
    }

    fn run(&mut self, samples: &mut [Complex<f32>]) {
        for x in samples.iter().copied(){
            self.boxed_chart.add(x)
        }
    }
}