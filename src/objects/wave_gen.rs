use core::f64::consts::PI;

use crate::objects::object::{Bus, DSPObject};

#[derive(Clone, Copy)]
pub struct WaveStepGen{
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub sample_rate: f64,
    
    pub bus: Bus<'static>,

    pub time: f64,
}

impl WaveStepGen {
    pub fn new(frequency: f64, amplitude: f64, phase: f64, sample_rate: f64) -> WaveStepGen {

        let bus = Bus::new_f64();

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
        crate::objects::object::Type::F64
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
        self.bus.trigger_f64(self.amplitude * (2.0 * PI * self.frequency * self.time + self.phase).sin());
        self.time += 1.0 / self.sample_rate;
    }

    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

