use rustdsp::elements::builder::{PipelineBuilder};
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {
    let sps = 16;
    let sample_rate = 2.0;
    let frequency = 1.0;

    let mut builder = PipelineBuilder::new();
    let signal_generator: SignalGenerator = SignalGenerator::new(frequency, sample_rate, sps,true); // samples per symbol

    builder.add(signal_generator);
    builder.add(WaterfallChart::new());
    builder.add(TimeChart::new(sps * 10));
    //builder.add(signal_adder::SignalAdder::new(-400.0, sample_rate, sps));
    //builder.add(WaterfallChart::new(sps));
    

    builder.build().run();
}