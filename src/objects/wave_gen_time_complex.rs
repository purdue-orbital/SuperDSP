use core::f32::consts::PI;
use std::thread::sleep;

use num::Complex;

use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone, Copy)]
pub struct WaveStepGenTimeComplex {
    pub frequency: Bus<'static>,
    pub amplitude: Bus<'static>,
    pub phase: Bus<'static>,
    pub sample_rate: Bus<'static>,

    bus: Bus<'static>,

    pub time: f32,
}

impl WaveStepGenTimeComplex {
    pub fn new(frequency: Bus<'static>, amplitude: Bus<'static>, phase: Bus<'static>, sample_rate: Bus<'static>) -> WaveStepGenTimeComplex {
        WaveStepGenTimeComplex {
            frequency,
            amplitude,
            phase,
            sample_rate,

            bus: Bus::new_complex(),

            time: 0.0,
        }
    }
}

impl DSPObject for WaveStepGenTimeComplex {
    fn return_type(&self) -> Type {
        Type::Complex
    }
    fn input_type(&self) -> Type {
        Type::NONE
    }
    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }
    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        panic!("WaveStepGenTimeComplex does not listen on a bus");
    }

    fn process(&mut self) {
        let phi = 2.0 * PI * *self.frequency.buffer_f32.unwrap().read() * self.time + *self.phase.buffer_f32.unwrap().read();
        let value = Complex::new(*self.amplitude.buffer_f32.unwrap().read() * libm::sinf(phi), *self.amplitude.buffer_f32.unwrap().read() * libm::cosf(phi));
        self.bus.trigger_complex(value);

        self.time += 1.0 / *self.sample_rate.buffer_f32.unwrap().read();
        sleep(std::time::Duration::from_secs_f32(1.0 / *self.sample_rate.buffer_f32.unwrap().read()));
    }
    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

