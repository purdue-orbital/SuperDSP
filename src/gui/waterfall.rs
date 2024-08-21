use std::collections::VecDeque;
use std::ops::Deref;
use crate::gui::{DSPChart, Message};
use crate::objects::object::{Bus, DSPObject, Type};
use iced::{Command, Length};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use spin::{Mutex, RwLock};
use std::prelude::rust_2021::Vec;
use std::sync::Arc;
use std::{print, println, vec};
use std::thread::spawn;
use iced::widget::{Container, Image};
use iced::widget::image::Handle;
use ndarray::{Array1, Axis};
use ndarray::linalg::Dot;
use num::Complex;
use crate::math;
use crate::math::fourier::fft_shift;

#[derive(Clone)]
pub struct Waterfall {
    buffer: Arc<RwLock<Array1<Complex<f64>>>>,

    dft_matrix: ndarray::Array2<Complex<f64>>,
    
    bus: Bus<'static>,

    pixels: Arc<RwLock<VecDeque<u8>>>,
    width_and_width: usize,
}

impl Waterfall {
    pub fn new(buff_size: usize) -> Waterfall {
        let mut pixels = vec![0; buff_size * buff_size * 4];
        
        for i in 0..buff_size * buff_size {
            pixels[4 * i] = 0;
            pixels[4 * i + 1] = 0;
            pixels[4 * i + 2] = 0;
            pixels[4 * i + 3] = 255;
        }

        let dft_matrix = fft_shift(buff_size).dot(&math::fourier::make_basis(buff_size));
        
        let mut w = Waterfall {
            buffer: Arc::new(RwLock::new(<Array1<Complex<f64>>>::from(vec![Complex::new(0.0, 0.0); buff_size]))),
            dft_matrix,
            pixels: Arc::new(RwLock::new(VecDeque::from(pixels))),
            width_and_width: buff_size,
            
            bus: Bus::new_complex(),
        };
        
        let w_clone = w.clone();
        
        spawn(move || {
            loop {
                // lock
                let mut locked_pixels = w_clone.pixels.write();
                let locked_buffer = w_clone.buffer.read();

                // skip if buffer is not full
                if locked_buffer.len() < w_clone.width_and_width {
                    continue;
                }

                // Preform dft on buffer
                let dfted = w_clone.dft_matrix.dot(&locked_buffer.view());
                let slice = dfted.as_slice().unwrap();

                // Add new data
                for i in 0..w_clone.width_and_width {
                    let val = (slice[i].norm_sqr() * 255.0) as u8;

                    locked_pixels[4 * i] = val;
                    locked_pixels[4 * i + 1] = val;
                    locked_pixels[4 * i + 2] = val;
                    locked_pixels[4 * i + 3] = 255;
                }

                // Shift pixels
                locked_pixels.rotate_left(w_clone.width_and_width * 4);
            }
        });
        
        w
    }
}

impl Default for Waterfall {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl Chart<Message> for Waterfall {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
    }
}

impl DSPObject for Waterfall {
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

    fn process(&mut self) {
        // Put input buffer into buffer
        self.buffer.write().remove_index(Axis(0),0);
        self.buffer.write().push(Axis(0), ndarray::arr0(*self.bus.buffer_complex.unwrap().read()).view()).unwrap();
    }
}

impl DSPChart for Waterfall {
    type Message = Message;
    type State = ();

    fn view(&self) -> iced::Element<Self::Message> {
        let vec = self.pixels.read().iter().cloned().collect::<Vec<_>>();

        Image::new(Handle::from_pixels(self.width_and_width as u32, self.width_and_width as u32, vec))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

