use std::dbg;
use std::prelude::rust_2021::Vec;
use std::sync::Arc;
use std::thread::spawn;

use iced::{Command, Length};
use iced::widget::{Image, image};
use nalgebra::{SMatrix, SVector};
use num::Complex;
use plotters_iced::{Chart, ChartBuilder, DrawingBackend};
use spin::RwLock;

use crate::gui::{DSPChart, Message};
use crate::math;
use crate::math::fourier::{fft_shift};
use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone)]
pub struct Waterfall<const N: usize>
{
    buffer: Arc<RwLock<SVector<Complex<f32>, N>>>,

    dft_matrix: Arc<SMatrix<Complex<f32>, N, N>>,

    bus: Bus<'static>,

    pixels: Arc<RwLock<Vec<u8>>>,
}

impl<const N: usize> Waterfall<N>
{
    pub fn new() -> Waterfall<N> {
        let mut pixels = Vec::with_capacity(N * N * 4);

        for i in 0..N * N * 4 {
            if i % 4 == 3 {
                pixels.push(255);
            } else {
                pixels.push(0);
            }
        }

        let t = math::fourier::make_basis::<N>();
        let fft_shift = fft_shift::<N>();
        
        let t_clone = t.clone();
        let fft_shift_clone = fft_shift.clone();

        let dft_matrix = Arc::new(t * fft_shift);

        let w = Waterfall {
            buffer: Arc::new(RwLock::new(<SVector<Complex<f32>, N>>::zeros())),
            dft_matrix,
            pixels: Arc::new(RwLock::new(pixels)),

            bus: Bus::new_complex(),
        };

        let w_clone = w.clone();

        spawn(move || {
            loop {
                // lock
                let mut locked_pixels = w_clone.pixels.write();
                let locked_buffer = w_clone.buffer.read();

                // Preform dft on buffer
                let dfted: SMatrix<Complex<f32>,N,1> = t_clone * fft_shift_clone * *locked_buffer;
                let slice = dfted.as_slice();
                
                dbg!(dfted);

                // Add new data
                for i in 0..N {
                    locked_pixels[i * 4] = (slice[i].norm_sqr().sqrt() * 255.0) as u8;
                    locked_pixels[i * 4 + 1] = 0;
                    locked_pixels[i * 4 + 2] = 0;
                    locked_pixels[i * 4 + 3] = 255;
                }

                locked_pixels.rotate_left(N * 4);
            }
        });

        w
    }
}

impl<const N: usize> Default for Waterfall<N>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Chart<Message> for Waterfall<N>
{
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {}
}

impl<const N: usize> DSPObject for Waterfall<N>
{
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
        let mut locked_buffer = self.buffer.write();

        // rotate buffer popping last element
        for i in 0..locked_buffer.len() - 2 {
            locked_buffer[i + 1] = locked_buffer[i];
        }

        // set first element
        locked_buffer[0] = *self.bus.buffer_complex.unwrap().read();
    }

    fn start(&mut self) {
        panic!("Charts can not be root object");
    }
}


impl<const N: usize> DSPChart for Waterfall<N>
{
    type Message = Message;
    type State = ();

    fn view(&self) -> iced::Element<Self::Message> {
        let image = image::Handle::from_pixels(N as u32, N as u32, self.pixels.read().clone());

        Image::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

