use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

use eframe::egui::{Ui, Vec2b};
use egui_plot::{Legend, Plot, Points};
use num_complex::Complex;

use crate::ui::charts::chart::Chart;

#[derive(Clone)]
pub struct PointChart {
    i_array: Arc<RwLock<VecDeque<f32>>>,
    q_array: Arc<RwLock<VecDeque<f32>>>,
}

impl PointChart {
    /// Create a line chart
    pub fn new(len: usize) -> PointChart {
        PointChart {
            i_array: Arc::new(RwLock::new(VecDeque::from(vec![0.0; len]))),
            q_array: Arc::new(RwLock::new(VecDeque::from(vec![0.0; len]))),
        }
    }

    pub fn add(&mut self, data: Complex<f32>) {
        self.i_array.write().unwrap().pop_front();
        self.q_array.write().unwrap().pop_front();

        self.i_array.write().unwrap().push_back(data.re);
        self.q_array.write().unwrap().push_back(data.im);
    }
}

impl Chart for PointChart {
    fn update(&self, ui: &mut Ui) {
        let plot = Plot::new("Point Chart").legend(Legend::default());

        plot.auto_bounds(Vec2b::from([true, true])).allow_scroll(false).show(ui, |plot_ui| {
            plot_ui.points(Points::new(self.i_array.read().unwrap().iter().enumerate().map(|(index, val)| [self.q_array.read().unwrap()[index] as f64, *val as f64]).collect::<Vec<[f64; 2]>>()).name("I"));
        });
    }
}