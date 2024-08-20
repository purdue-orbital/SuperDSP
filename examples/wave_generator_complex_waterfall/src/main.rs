use rustdsp::gui::time_chart::TimeChart;
use rustdsp::gui::waterfall::Waterfall;
use rustdsp::objects::Pipeline;
use rustdsp::objects::wave_gen_time_complex::WaveStepGenTimeComplex;

fn main() {
    let mut pipeline = Pipeline::new();

    let mut waterfall = Waterfall::new(1024);
    let mut gen = WaveStepGenTimeComplex::new(8.0, 1.0, 0.0, 16.0);

    pipeline.add_object(&mut gen);
    pipeline.add_gui_element(&mut waterfall);

    pipeline.process();
}