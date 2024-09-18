use superdsp::gui::slider::Slider;
use superdsp::gui::waterfall::Waterfall;
use superdsp::objects::GUIExecutor;
use superdsp::objects::object::{Bus, DSPObject};
use superdsp::objects::wave_gen_time_complex::WaveStepGenTimeComplex;

fn main() {
    let mut frequency = Bus::from_f32(10.0);
    let amplitude = Bus::from_f32(1.0);
    let phase = Bus::from_f32(0.0);
    let sample_rate = Bus::from_f32(441.0);

    let mut waterfall: Waterfall<64> = Waterfall::new();
    let mut slider = Slider::new(-220.0,220.0);
    
    slider.set_bus(&mut frequency);
    
    let mut gen = WaveStepGenTimeComplex::new(frequency, amplitude, phase, sample_rate);

    waterfall.set_bus(gen.get_bus());
    GUIExecutor::run(vec![Box::new(waterfall), Box::new(slider)], Box::new(gen))
}