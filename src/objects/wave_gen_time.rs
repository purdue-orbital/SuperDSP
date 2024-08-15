use alloc::sync::Arc;
use alloc::vec::Vec;
use core::f64::consts::PI;
use spin::Mutex;
use std::thread::sleep;

use crate::objects::object::DSPObject;

#[derive(Debug, Clone)]
pub struct WaveStepGenTime {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub sample_rate: f64,
    pub samples: Vec<f64>,

    pub output_buffer: Arc<Mutex<f64>>,

    pub time: f64,
}

impl WaveStepGenTime {
    pub fn new(frequency: f64, amplitude: f64, phase: f64, sample_rate: f64) -> WaveStepGenTime {
        WaveStepGenTime {
            frequency,
            amplitude,
            phase,
            sample_rate,
            samples: Vec::new(),

            output_buffer: Arc::new(Mutex::new(1.0)),

            time: 0.0,
        }
    }
}

impl DSPObject for WaveStepGenTime {
    fn set_input_buffer(&mut self, buffer: Arc<spin::mutex::Mutex<f64>>) {
        // WaveGen does not take any input
        panic!("WaveGen does not have an input buffer");
    }
    fn get_output_buffer(&self) -> Arc<Mutex<f64>> {
        self.output_buffer.clone()
    }

    fn set_input_buffer_vec(&mut self, buffer: Arc<spin::mutex::Mutex<Vec<f64>>>) {
        panic!("WaveGen does not have a vector input buffer");
    }
    fn get_output_buffer_vec(&self) -> Arc<spin::mutex::Mutex<Vec<f64>>> {
        panic!("WaveGen does not have a vector output buffer");
    }

    fn process(&mut self) {
        *self.output_buffer.lock() = self.amplitude * (2.0 * PI * self.frequency * self.time + self.phase).sin();
        self.time += 1.0 / self.sample_rate;

        sleep(std::time::Duration::from_secs_f64(1.0 / self.sample_rate));
    }
}

