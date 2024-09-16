use superdsp::gui::time_chart::TimeChart;
use superdsp::objects::GUIExecutor;
use superdsp::objects::object::{Bus, DSPObject};
use superdsp::objects::wave_gen_time::WaveStepGenTime;

fn main() {
    let frequency = Bus::from_f32(200.0);
    let amplitude = Bus::from_f32(1.0);
    let phase = Bus::from_f32(0.0);
    let sample_rate = Bus::from_f32(441.0);
    
    let mut wave_step_gen = WaveStepGenTime::new(frequency, amplitude, phase, sample_rate);
    let mut chart = TimeChart::new();

    let s = wave_step_gen.get_bus();
    chart.set_bus(s);

    GUIExecutor::run(vec![Box::new(chart)], Box::new(wave_step_gen));
}