use iced::Command;
use plotters_iced::{Chart, ChartBuilder, DrawingBackend};

use crate::gui::{DSPChart, Message};
use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone)]
pub struct Slider {
    bus: Bus<'static>,
    min: f32,
    max: f32,
}

impl Slider {
    pub fn new(min: f32, max: f32) -> Slider {
        Slider {
            bus: Bus::new_complex(),
            min,
            max,
        }
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new(0.0, 10.0)
    }
}

impl Chart<Message> for Slider {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {}
}

impl DSPObject for Slider {
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
        bus.subscribe(self);
    }

    fn start(&mut self) {
        panic!("Charts can not be root object");
    }

    fn process(&mut self) {}
}

impl DSPChart for Slider {
    type Message = Message;
    type State = ();

    fn view(&self) -> iced::Element<Self::Message> {
        iced::widget::Slider::new(self.min..=self.max,*self.bus.buffer_f32.unwrap().read(), |value| {
            self.bus.trigger_f32(value);
            Message::SliderChanged
        }).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

