use core::f32::consts::PI;

use crate::objects::object::{Bus, DSPObject};

#[derive(Clone, Copy)]
pub struct WaveStepGen {
    pub frequency: Bus<'static>,
    pub amplitude: Bus<'static>,
    pub phase: Bus<'static>,
    pub sample_rate: Bus<'static>,

    pub bus: Bus<'static>,

    pub time: f32,
}

impl WaveStepGen {
    pub fn new(frequency: Bus<'static>, amplitude: Bus<'static>, phase: Bus<'static>, sample_rate: Bus<'static>) -> WaveStepGen {
        let bus = Bus::new_f32();

        WaveStepGen {
            frequency,
            amplitude,
            phase,
            sample_rate,

            bus,
            time: 0.0,
        }
    }
}

impl DSPObject for WaveStepGen {
    fn return_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::F32
    }
    fn input_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::NONE
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        panic!("WaveStepGen does not listen on a bus");
    }

    fn process(&mut self) {
        self.bus.trigger_f32(*self.amplitude.buffer_f32.unwrap().read() * (2.0 * PI * *self.frequency.buffer_f32.unwrap().read() * self.time + *self.phase.buffer_f32.unwrap().read()).sin());
        self.time += 1.0 / *self.sample_rate.buffer_f32.unwrap().read();
    }

    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

