use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use eframe::egui;
use eframe::egui::Context;
use egui_plot::{Legend, Line, Plot};
use num_complex::Complex;
use crate::ui::charts::chart::Chart;

#[derive(Clone)]
pub struct LineChart{
    i_array: Arc<RwLock<VecDeque<f32>>>,
    q_array: Arc<RwLock<VecDeque<f32>>>
}

impl LineChart{

    /// Create a line chart
    pub fn new(len:usize) -> LineChart{
        LineChart{
            i_array: Arc::new(RwLock::new(VecDeque::from(vec![0.0; len]))),
            q_array: Arc::new(RwLock::new(VecDeque::from(vec![0.0; len])))
        }
    }

    pub fn add(&mut self, data:Complex<f32>){
        self.i_array.write().unwrap().pop_front();
        self.q_array.write().unwrap().pop_front();

        self.i_array.write().unwrap().push_back(data.re);
        self.q_array.write().unwrap().push_back(data.im);
    }
}

impl Chart for LineChart{
    fn update(&self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("Line Chart").legend(Legend::default());

            plot.auto_bounds_y().auto_bounds_x().show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.i_array.read().unwrap().iter().enumerate().map(|(index, val)| [index as f64, *val as f64]).collect::<Vec<[f64;2]>>()).name("I"));
                plot_ui.line(Line::new(self.q_array.read().unwrap().iter().enumerate().map(|(index, val)| [index as f64, *val as f64]).collect::<Vec<[f64;2]>>()).name("Q"));
            });

        });

        ctx.request_repaint()
    }
}