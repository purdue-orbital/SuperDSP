use core::f32::consts::PI;
use std::thread::sleep;

use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone, Copy)]
pub struct WaveStepGenTime {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub sample_rate: f32,

    bus: Bus<'static>,

    pub time: f32,
}

impl WaveStepGenTime {
    pub fn new(frequency: f32, amplitude: f32, phase: f32, sample_rate: f32) -> WaveStepGenTime {
        WaveStepGenTime {
            frequency,
            amplitude,
            phase,
            sample_rate,

            bus: Bus::new_f32(),

            time: 0.0,
        }
    }
}

impl DSPObject for WaveStepGenTime {
    fn return_type(&self) -> Type {
        Type::F32
    }
    fn input_type(&self) -> Type {
        Type::NONE
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        panic!("WaveStepGenTime does not listen on a bus");
    }

    fn process(&mut self) {
        self.bus.trigger_f32(self.amplitude * (2.0 * PI * self.frequency * self.time + self.phase).sin());
        self.time += 1.0 / self.sample_rate;

        sleep(std::time::Duration::from_secs_f32(1.0 / self.sample_rate));
    }

    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

