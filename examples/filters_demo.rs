use rustdsp::elements::builder::{PipelineBuilder};
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::{frequency_shift, signal_adder};
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {
    let sps = 100;
    let sample_rate = 1e3;
    let frequency = 1.0;

    let mut builder = PipelineBuilder::new();
    let signal_generator: SignalGenerator = SignalGenerator::new(frequency, sample_rate, sps); // samples per system

    builder.add(signal_generator);
    builder.add(WaterfallChart::new(sps));
    builder.add(frequency_shift::FrequencyShift::new(-200.0, sample_rate, sps));
    builder.add(signal_adder::SignalAdder::new(-400.0, sample_rate, sps));
    builder.add(WaterfallChart::new(sps));
    

    builder.build().run();
}