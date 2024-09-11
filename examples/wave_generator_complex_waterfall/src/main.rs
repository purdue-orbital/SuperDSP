#![feature(generic_const_exprs)]

use superdsp::gui::waterfall::Waterfall;
use superdsp::objects::GUIExecutor;
use superdsp::objects::object::DSPObject;
use superdsp::objects::wave_gen_time_complex::WaveStepGenTimeComplex;
use std::thread;

fn main() {
    let mut waterfall:Waterfall<256> = Waterfall::new();
    let mut gen = WaveStepGenTimeComplex::new(8.0, 1.0, 0.0, 16.0);

    waterfall.set_bus(gen.get_bus());
    GUIExecutor::run(vec![Box::new(waterfall)], Box::new(gen))
}