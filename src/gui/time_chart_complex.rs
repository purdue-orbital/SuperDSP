use crate::gui::{DSPChart, Message};
use crate::objects::object::{Bus, DSPObject, Type};
use iced::Command;
use plotters::prelude::{LineSeries, RED};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use spin::Mutex;
use std::prelude::rust_2021::Vec;
use std::sync::Arc;
use std::vec;
use num::Complex;
use plotters::style::BLUE;

#[derive(Clone)]
pub struct TimeChartComplex {
    buffer: Arc<Mutex<Vec<Complex<f64>>>>,
    bus: Bus<'static>,
}

impl TimeChartComplex {
    pub fn new() -> TimeChartComplex {
        TimeChartComplex { 
            buffer: Arc::new(Mutex::new(vec![Complex::new(0.0,0.0); 50])),
            bus: Bus::new_complex()
        }
    }
}

impl Default for TimeChartComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl Chart<Message> for TimeChartComplex {
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
                (0..50).map(|x| (x as f32, self.buffer.lock()[x].re as f32)),
                &RED,
            ))
            .unwrap();
        
        chart
            .draw_series(LineSeries::new(
                (0..50).map(|x| (x as f32, self.buffer.lock()[x].im as f32)),
                &BLUE,
            ))
            .unwrap();
    }
}

impl DSPObject for TimeChartComplex {
    fn return_type(&self) -> Type {
        Type::Complex
    }

    fn input_type(&self) -> Type {
        Type::Complex
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        self.bus = *bus;
    }

    fn start(&mut self) {
        panic!("Charts can not be root object");
    }

    fn process(&mut self) {
        // Put input buffer into buffer
        self.buffer.lock().push(*self.bus.buffer_complex.unwrap().read());

        // Remove the first element if buffer is too long
        if self.buffer.lock().len() > 50 {
            self.buffer.lock().remove(0);
        }
    }
}

impl DSPChart for TimeChartComplex {
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

