use rustdsp::gui::time_chart_complex::TimeChartComplex;
use rustdsp::gui::waterfall::Waterfall;
use rustdsp::objects::object::DSPObject;
use rustdsp::radios;

fn main() {
    let mut src = radios::bladerf::src::BladeRfSrc::new(915000000, 1_000_000, 1_000_000, 1_000_000, 1024);
    let mut chart = TimeChartComplex::new();
    let mut waterfall = Waterfall::new(1024);
    
    let s = src.get_bus();

    waterfall.set_bus(s);
    chart.set_bus(s);
    
    rustdsp::objects::GUIExecutor::run(vec![Box::new(waterfall), Box::new(chart)], Box::new(src));
}