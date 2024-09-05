use std::prelude::rust_2021::Vec;
use std::sync::Arc;
use std::vec;

use iced::Command;
use plotters::prelude::{LineSeries, RED};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use spin::RwLock;

use crate::gui::{DSPChart, Message};
use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone)]
pub struct TimeChart {
    buffer: Arc<RwLock<Vec<f64>>>,
    bus: Bus<'static>,
}

impl TimeChart {
    pub fn new() -> TimeChart {
        TimeChart { buffer: Arc::new(RwLock::new(vec![0.0; 50])), bus: Bus::new_f64() }
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
        if self.buffer.read().len() < 50 {
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
                (0..50).map(|x| (x as f32, self.buffer.read()[x] as f32)),
                &RED,
            ))
            .unwrap();
    }
}

impl DSPObject for TimeChart {
    fn return_type(&self) -> Type {
        Type::F64
    }

    fn input_type(&self) -> Type {
        Type::F64
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        bus.subscribe(self);
        self.bus = *bus;
    }

    fn process(&mut self) {
        // Put input buffer into buffer
        self.buffer.write().push(*self.bus.buffer_f64.unwrap().read());

        // Remove the first element if buffer is too long
        if self.buffer.read().len() > 50 {
            self.buffer.write().remove(0);
        }
    }

    fn start(&mut self) {
        panic!("Charts can not be root object");
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

