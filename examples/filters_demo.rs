use rustdsp::elements::builder::{PipelineBuilder};
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;
use rustdsp::elements::interpolator::Interpolator;

fn main() {
    let sps = 2;
    let sample_rate = 500.0;
    let frequency = 250.0;

    let mut builder = PipelineBuilder::new();
    let signal_generator: SignalGenerator = SignalGenerator::new(frequency, sample_rate, sps, false); // samples per symbol

    builder.add(signal_generator);
    builder.add(Interpolator::new(16, sample_rate, frequency, 1.0));
    builder.add(WaterfallChart::new());
    builder.add(TimeChart::new(sps * 10));
    //builder.add(signal_adder::SignalAdder::new(-400.0, sample_rate, sps));
    //builder.add(WaterfallChart::new(sps));
    

    builder.build().run();
}