use rustdsp::gui::time_chart_complex::TimeChartComplex;
use rustdsp::gui::waterfall::Waterfall;
use rustdsp::objects::Pipeline;
use rustdsp::radios;

fn main() {
    let mut pipeline = Pipeline::new();
    
    let mut radio = radios::bladerf::BladeRF::new(916000000, 100000, 20, 200000, 1024);
    let mut chart = TimeChartComplex::new();
    let mut waterfall = Waterfall::new(1024);

    pipeline.add_object(&mut radio);
    pipeline.add_gui_element(&mut chart);
    pipeline.add_gui_element(&mut waterfall);

    pipeline.process();
}