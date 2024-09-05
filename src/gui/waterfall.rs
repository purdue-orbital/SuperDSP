use std::dbg;
use std::prelude::rust_2021::Vec;
use std::sync::Arc;
use std::thread::spawn;

use iced::{Command, Length};
use iced::widget::canvas::{Program};
use iced::widget::{Image, image};
use nalgebra::SVector;
use num::Complex;
use plotters_iced::{Chart, ChartBuilder, DrawingBackend};
use spin::RwLock;

use crate::gui::{DSPChart, Message};
use crate::math;
use crate::math::fourier::fft_shift;
use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone)]
pub struct Waterfall<const N: usize> {
    buffer: Arc<RwLock<SVector<Complex<f32>, N>>>,

    dft_matrix: nalgebra::SMatrix<Complex<f32>,N,N>,

    bus: Bus<'static>,

    pixels: Arc<RwLock<[[u8;3]; N]>>,
    width_and_width: usize,
}

impl<const N: usize> Waterfall<N> {
    pub fn new() -> Waterfall<N> {
        dbg!(1);
        let pixels = [[0;3]; N];
        dbg!(2);
        let dft_matrix = fft_shift() * math::fourier::make_basis();
        dbg!(3);
        

        let w = Waterfall {
            buffer: Arc::new(RwLock::new(<SVector<Complex<f32>, N>>::zeros())),
            dft_matrix,
            pixels: Arc::new(RwLock::new(pixels)),
            width_and_width: N,

            bus: Bus::new_complex(),
        };

        dbg!(4);
        

        let w_clone = w.clone();

        dbg!(5);
        

        spawn(move || {
            loop {
                // lock
                let mut locked_pixels = w_clone.pixels.write();
                let locked_buffer = w_clone.buffer.read();

                // Preform dft on buffer
                let dfted = w_clone.dft_matrix * *locked_buffer;
                let slice = dfted.as_slice();

                // Add new data
                for i in 0..w_clone.width_and_width {
                    let val = (slice[i].norm_sqr() * 255.0) as u8;

                    locked_pixels[i][0] = val;
                    locked_pixels[i][1] = val;
                    locked_pixels[i][2] = val;
                }

                // Shift pixels
                locked_pixels.rotate_left(w_clone.width_and_width * 4);
            }
        });

        w
    }
}

impl<const N: usize> Default for Waterfall<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Chart<Message> for Waterfall<N> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {}
}

impl<const N: usize> DSPObject for Waterfall<N> {
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
        bus.subscribe(self as *mut dyn DSPObject);
    }

    fn process(&mut self) {
        // rotate buffer popping last element
        for i in 0..self.buffer.read().len() - 2 {
            self.buffer.write()[i + 1] = self.buffer.read()[i];
        }
        
        // set first element
        self.buffer.write()[0] = *self.bus.buffer_complex.unwrap().read();
    }

    fn start(&mut self) {
        panic!("Charts can not be root object");
    }
}


impl<const N: usize> DSPChart for Waterfall<N> {
    type Message = Message;
    type State = ();

    fn view(&self) -> iced::Element<Self::Message> {
        let image = image::Handle::from_pixels(N as u32,N as u32, self.pixels.read().iter().flatten().copied().collect::<Vec<_>>());

        Image::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

