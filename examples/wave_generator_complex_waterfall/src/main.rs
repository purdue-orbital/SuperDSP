use superdsp::gui::waterfall::Waterfall;
use superdsp::objects::GUIExecutor;
use superdsp::objects::object::DSPObject;
use superdsp::objects::wave_gen_time_complex::WaveStepGenTimeComplex;

fn main() {
    let mut waterfall: Waterfall<256> = Waterfall::new();
    let mut gen = WaveStepGenTimeComplex::new(200.0, 1.0, 0.0, 441.0);

    waterfall.set_bus(gen.get_bus());
    GUIExecutor::run(vec![Box::new(waterfall)], Box::new(gen))
}