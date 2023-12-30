use std::sync::Arc;
use eframe::{App, egui};
use eframe::egui::accesskit::ActionData::Value;
use eframe::egui::Context;
use egui_plot::{Line, Plot};
use crate::ui::charts::chart::Chart;
use crate::ui::charts::line_chart::LineChart;

pub struct Frame{
    pub(crate) charts: Vec<Box<dyn Chart>>
}

impl Frame{
    pub fn new() -> Frame{

        Frame{
            charts:vec![]
        }
    }

    pub fn add(&mut self, chart: &Box<dyn Chart>){
        self.charts.push(chart.clone_box())
    }

    pub fn add_all(&mut self, array: Vec<Box<dyn Chart>>){
        self.charts = array;
    }

    pub fn create(self, cc: &eframe::CreationContext<'_>) -> Frame{
        self
    }
}

impl App for Frame{
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        for x in self.charts.as_slice(){
            x.update(ctx);
        }
    }
}