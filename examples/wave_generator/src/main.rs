use rustdsp::gui::time_chart::TimeChart;
use rustdsp::objects::wave_gen_time::WaveStepGenTime;
use rustdsp::objects::Pipeline;

fn main() {
    let mut pipeline = Pipeline::new();

    let mut waterfall = TimeChart::default();
    let mut gen = WaveStepGenTime::new(1.0, 1.0, 0.0, 16.0);

    pipeline.add_object(&mut gen);
    pipeline.add_gui_element(&mut waterfall);

    pipeline.process();
}