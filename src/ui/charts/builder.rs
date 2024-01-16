use eframe::NativeOptions;

use crate::ui::charts::chart::Chart;
use crate::ui::frame::Frame;

pub struct WindowBuilder {
    charts: Vec<Box<dyn Chart>>,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            charts: vec![],
        }
    }

    pub fn add_chart<T: Chart + 'static>(&mut self, chart: T) -> Box<T> {
        let b = Box::new(chart);

        self.charts.push(b.clone_box());

        b
    }

    pub fn build(&self) {
        let native_options = NativeOptions::default();

        let mut frame = Frame::new();

        for x in self.charts.iter() {
            frame.add(x.clone_box())
        }

        eframe::run_native("test", native_options, Box::new(|cc| Box::new(frame.create(cc)))).unwrap();
    }
}