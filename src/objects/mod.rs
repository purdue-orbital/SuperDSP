#[cfg(feature = "gui")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "gui")]
use std::thread::spawn;

pub mod object;
pub mod wave_gen;

#[cfg(not(feature = "std"))]
pub mod wave_gen_time;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[cfg(not(feature = "gui"))]
pub struct Pipeline {
    pub objects: Vec<Box<dyn object::DSPObject>>,
}

#[cfg(not(feature = "gui"))]
impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            objects: Vec::new(),
        }
    }

    pub fn add_object<T: object::DSPObject + 'static>(&mut self, object: &mut T) {
        if !self.objects.is_empty() {
            let last = self.objects.len() - 1;
            let last_obj = self.objects[last].clone_box();
            object.set_input_buffer(last_obj.get_output_buffer());
        }

        self.objects.push(object.clone_box());
    }

    pub fn process(&mut self) {
        loop {
            for obj in self.objects.iter_mut() {
                obj.process();
            };
        }

    }
}

#[cfg(feature = "gui")]
pub struct Pipeline {
    pub objects: Vec<Box<dyn object::DSPObject>>,
    pub gui: Option<crate::gui::GUI>,
}

#[cfg(feature = "gui")]
impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            objects: Vec::new(),
            gui: None,
        }
    }

    pub fn add_object<T: object::DSPObject + 'static>(&mut self, object: &mut T) {
        if !self.objects.is_empty() {
            let last = self.objects.len() - 1;
            let last_obj = self.objects[last].clone_box();
            match last_obj.return_type() { 
                object::Type::F64 => object.set_input_buffer(last_obj.get_output_buffer()),
                object::Type::Complex => object.set_input_buffer_complex(last_obj.get_output_buffer_complex()),
                object::Type::Vec => object.set_input_buffer_vec(last_obj.get_output_buffer_vec()),
                object::Type::ComplexVec => object.set_input_buffer_complex_vec(last_obj.get_output_buffer_complex_vec()),
                _ => panic!("Invalid input type"),
            }
        }

        self.objects.push(object.clone_box());
    }

    pub fn add_gui_element<T: crate::gui::DSPChart<State=(), Message=crate::gui::Message> + 'static>(&mut self, element: &mut T) {
        if let Some(gui) = &mut self.gui {
            gui.add_element(element);
        } else {
            let mut gui = crate::gui::GUI::new(800, 600);
            gui.add_element(element);
            self.gui = Some(gui);
        }

        self.add_object(element)
    }

    pub fn process(&mut self) {
        let objs_clone = Arc::new(Mutex::new(self.objects.iter().map(|obj| obj.clone_box()).collect::<Vec<_>>()));

        spawn(
            move || {
                loop {
                    let mut objs = objs_clone.lock().unwrap();
                    for obj in objs.iter_mut() {
                        obj.process();
                    };
                }
            }
        );

        self.gui.clone().unwrap().start();
    }
}
