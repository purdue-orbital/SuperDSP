use crate::gui::{DSPChart, Message};
use crate::objects::object::DSPObject;
use iced::Command;
use plotters::prelude::{LineSeries, RED};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use spin::Mutex;
use std::prelude::rust_2021::Vec;
use std::sync::Arc;
use std::vec;

#[derive(Clone)]
pub struct TimeChart {
    input_buffer: Arc<Mutex<f64>>,
    buffer: Arc<Mutex<Vec<f64>>>,
}

impl TimeChart {
    pub fn new() -> TimeChart {
        TimeChart { buffer: Arc::new(Mutex::new(vec![0.0; 50])), input_buffer: Arc::new(Default::default()) }
    }
}

impl Default for TimeChart {
    fn default() -> Self {
        Self::new()
    }
}

impl Chart<Message> for TimeChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        if self.buffer.lock().len() < 50 {
            return;
        }
        let mut chart = builder
            .margin(30)
            .caption("Time Chart", ("sans-serif", 22))
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f32..50f32, -1.2f32..1.2f32)
            .unwrap();

        chart
            .configure_mesh()
            .x_labels(3)
            .y_labels(3)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..50).map(|x| (x as f32, self.buffer.lock()[x] as f32)),
                &RED,
            ))
            .unwrap();
    }
}

impl DSPObject for TimeChart {
    fn set_input_buffer(&mut self, buffer: Arc<Mutex<f64>>) {
        self.input_buffer = buffer;
    }

    fn get_output_buffer(&self) -> Arc<Mutex<f64>> {
        self.input_buffer.clone()
    }

    fn set_input_buffer_vec(&mut self, buffer: Arc<Mutex<Vec<f64>>>) {
        panic!("TimeChart does not have a vector input buffer");
    }

    fn get_output_buffer_vec(&self) -> Arc<Mutex<Vec<f64>>> {
        panic!("TimeChart does not have a vector output buffer");
    }

    fn process(&mut self) {
        // Put input buffer into buffer
        self.buffer.lock().push(*self.input_buffer.lock());

        // Remove the first element if buffer is too long
        if self.buffer.lock().len() > 50 {
            self.buffer.lock().remove(0);
        }
    }
}

impl DSPChart for TimeChart {
    type Message = Message;
    type State = ();

    fn view(&self) -> iced::Element<Self::Message> {
        ChartWidget::new(self)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

