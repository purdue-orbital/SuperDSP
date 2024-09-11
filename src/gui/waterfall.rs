// use std::{dbg, println};
// use std::boxed::Box;
// use std::convert::TryInto;
// use std::prelude::rust_2021::Vec;
// use std::sync::Arc;
// use std::thread::spawn;
// 
// use iced::{Command, Length};
// use iced::widget::canvas::{Program};
// use iced::widget::{Image, image};
// use nalgebra::SVector;
// use num::Complex;
// use plotters_iced::{Chart, ChartBuilder, DrawingBackend};
// use spin::RwLock;
// 
// use std::vec;
// 
// use crate::gui::{DSPChart, Message};
// use crate::math;
// use crate::math::fourier::fft_shift;
// use crate::objects::object::{Bus, DSPObject, Type};
// 
// #[derive(Clone)]
// pub struct Waterfall<const N: usize> 
// where [u8; N * N * 4]: Clone + 'static
// {
//     buffer: Arc<RwLock<SVector<Complex<f32>, N>>>,
// 
//     dft_matrix: nalgebra::DMatrix<Complex<f32>>,
// 
//     bus: Bus<'static>,
// 
//     pixels: Arc<RwLock<Vec<u8>>>,
// }
// 
// impl<const N: usize> Waterfall<N>
//     where [u8; N * N * 4]: Clone + 'static
// {
//     pub fn new() -> Waterfall<N> {
//         let mut pixels = Vec::with_capacity(N * N * 4);
//         println!("Pixels: {}", N * N * 4);
// 
//         for _ in 0..N * N * 4 {
//             pixels.push(0);
//         }
// 
//         println!("Test 0.5");
// 
//         let t = math::fourier::dynamic_make_basis();
// 
//         println!("test 0.75");
// 
//         let dft_matrix = fft_shift::<N>().data.into() * t;
// 
// 
//         println!("test 1");
// 
//         let w = Waterfall {
//             buffer: Arc::new(RwLock::new(<SVector<Complex<f32>, N>>::zeros())),
//             dft_matrix,
//             pixels: Arc::new(RwLock::new(pixels)),
// 
//             bus: Bus::new_complex(),
//         };
// 
// 
//         println!("test");
// 
//         let w_clone = w.clone();
//         
//         spawn(move || {
//             loop {
//                 // lock
//                 let mut locked_pixels = w_clone.pixels.write();
//                 let locked_buffer = w_clone.buffer.read();
// 
//                 // Preform dft on buffer
//                 let dfted = w_clone.dft_matrix * *locked_buffer;
//                 let slice = dfted.as_slice();
// 
//                 // Add new data
//                 for i in 0..N{
//                     let val = (slice[i].norm_sqr() * 255.0) as u8;
// 
//                     locked_pixels[i] = val;
//                     locked_pixels[i + 1] = val;
//                     locked_pixels[i + 2] = val;
//                     locked_pixels[i + 3] = 255;
//                 }
//                 // Shift pixels
//                 locked_pixels.rotate_left(N * 4);
//             }
//         });
// 
//         w
//     }
// }
// 
// impl<const N: usize> Default for Waterfall<N>
//     where [u8; N * N * 4]: Clone + 'static
// {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// 
// impl<const N: usize> Chart<Message> for Waterfall<N>
//     where [u8; N * N * 4]: Clone + 'static
// {
//     type State = ();
// 
//     fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {}
// }
// 
// impl<const N: usize> DSPObject for Waterfall<N>
//     where [u8; N * N * 4]: Clone + 'static
// {
//     fn return_type(&self) -> Type {
//         Type::Complex
//     }
// 
//     fn input_type(&self) -> Type {
//         Type::Complex
//     }
// 
//     fn get_bus(&mut self) -> &mut Bus<'static> {
//         &mut self.bus
//     }
// 
//     fn set_bus(&mut self, bus: &mut Bus<'static>) {
//         self.bus = *bus;
//         bus.subscribe(self as *mut dyn DSPObject);
//     }
// 
//     fn process(&mut self) {
//         let mut locked_buffer = self.buffer.write();
//         
//         dbg!(&locked_buffer);
// 
//         // rotate buffer popping last element
//         for i in 0..locked_buffer.len() - 2 {
//             locked_buffer[i + 1] = locked_buffer[i];
//         }
// 
//         // set first element
//         locked_buffer[0] = *self.bus.buffer_complex.unwrap().read();
//     }
// 
//     fn start(&mut self) {
//         panic!("Charts can not be root object");
//     }
// }
// 
// 
// impl<const N: usize> DSPChart for Waterfall<N>
//     where [u8; N * N * 4]: Clone + 'static
// {
//     type Message = Message;
//     type State = ();
// 
//     fn view(&self) -> iced::Element<Self::Message> {
//         let image = image::Handle::from_pixels(N as u32, N as u32, self.pixels.read().clone());
// 
//         Image::new(image)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .into()
//     }
// 
//     fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
//         Command::none()
//     }
// }
// 
