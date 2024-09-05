use superdsp::gui::time_chart::TimeChart;
use superdsp::objects::GUIExecutor;
use superdsp::objects::object::DSPObject;
use superdsp::objects::wave_gen_time::WaveStepGenTime;

fn main() {
    let mut wave_step_gen = WaveStepGenTime::new(440.0, 1.0, 0.0, 44100.0);

    let mut chart = TimeChart::new();


    let s = wave_step_gen.get_bus();
    chart.set_bus(s);

    GUIExecutor::run(vec![Box::new(chart)], Box::new(wave_step_gen));
}