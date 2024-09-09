use std::prelude::rust_2021::{Box, String, Vec};
use std::thread::sleep;
use std::time::Duration;
use std::vec;

use iced::{Application, Command, Element, executor, Settings, Subscription, Theme};
use iced::widget::{Column, Row};

use crate::objects::object::DSPObject;

pub mod time_chart;
pub mod time_chart_complex;
pub mod waterfall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Tick,
}

pub trait DSPChart: DSPChartClone + DSPObject {
    type Message;
    type State;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
}

pub trait DSPChartClone {
    fn clone_chart(&self) -> Box<dyn DSPChart<Message=Message, State=()>>;
}


impl<T> DSPChartClone for T
    where
        T: 'static + DSPChart<State=(), Message=Message> + Clone,
{
    fn clone_chart(&self) -> Box<dyn DSPChart<Message=Message, State=()>> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DSPChart<Message=Message, State=()>> {
    fn clone(&self) -> Box<dyn DSPChart<Message=Message, State=()>> {
        self.clone_chart()
    }
}

#[derive(Clone)]
pub struct GUI {
    pub width: usize,
    pub height: usize,

    pub(crate) elements: Vec<Box<dyn DSPChart<State=(), Message=Message>>>,
}

pub fn tick() {
    sleep(Duration::from_secs_f32(1.0 / 60.0))
}

impl GUI {
    pub fn new(width: usize, height: usize) -> GUI {
        GUI { width, height, elements: vec![] }
    }

    pub fn add_element<T: DSPChart<State=(), Message=Message> + 'static + ?Sized>(&mut self, element: &mut T) {
        self.elements.push(element.clone_chart());
    }

    pub fn start(&self) {
        GUI::run(Settings::with_flags((self.width, self.height, self.elements.clone()))).expect("Failed to start GUI");
    }
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (usize, usize, Vec<Box<dyn DSPChart<State=(), Message=Message>>>);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (GUI { width: flags.0, height: flags.1, elements: flags.2 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("SuperDSP")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::Tick => {
                self.elements.iter_mut().for_each(|element| {
                    let _ = element.update(_message);
                });
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Evenly split graphs (if possible)
        let s = (self.elements.len() as f32).sqrt();

        let mut column = Column::new();

        // split elements into rows
        for i in 0..(s.ceil() as usize) {
            let mut row = Row::new();
            for j in 0..(s.ceil() as usize) {
                if i * (s.ceil() as usize) + j < self.elements.len() {
                    row = row.push(self.elements[i * (s.ceil() as usize) + j].view());
                }
            }
            column = column.push(row);
        }

        column.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_secs_f32(1.0 / 60.0)).map(|_| Message::Tick)
    }
}